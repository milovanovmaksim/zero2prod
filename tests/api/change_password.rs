use crate::helpers::{assert_is_redirect_to, spawn_app};
use secrecy::ExposeSecret;
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    let app = spawn_app().await;
    let response = app.get_change_password().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_mutch() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password.expose_secret()
    }))
    .await;
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password.expose_secret(),
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - the field values must match.</i></p>"
    ));
}
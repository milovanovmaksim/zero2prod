use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

use crate::{session_state::TypedSession, utils::see_other};

use super::reject_anonymous;

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    let session_clone = session.clone();
    let _user_id = reject_anonymous(session_clone).await?;
    session.log_out();
    FlashMessage::info("You have successfully logged out.").send();
    Ok(see_other("/login"))
}

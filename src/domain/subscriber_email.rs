use validator::validate_email;

#[derive(Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{}is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}



#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    use super::SubscriberEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_eq!(SubscriberEmail::parse(email.clone()).err(), Some(format!("{}is not a valid subscriber email.", email)));
    }

    #[test]
    fn email_messing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_eq!(SubscriberEmail::parse(email.clone()).err(), Some(format!("{}is not a valid subscriber email.", email)));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_eq!(SubscriberEmail::parse(email.clone()).err(), Some(format!("{}is not a valid subscriber email.", email)));
    }

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email: String = SafeEmail().fake();
        assert_eq!(SubscriberEmail::parse(email.clone()).unwrap().as_ref(), email);
}
}
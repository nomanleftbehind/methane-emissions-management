use secrecy::{ExposeSecret, Secret};

#[derive(Debug, Clone)]
pub struct SessionCookieNameSecret(Secret<String>);

impl SessionCookieNameSecret {
    pub fn new(session_cookie_name_secret: Secret<String>) -> Self {
        Self(session_cookie_name_secret)
    }

    pub fn get_session_cookie_name(&self) -> &str {
        self.0.expose_secret()
    }
}

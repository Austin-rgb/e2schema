#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdated {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country: Option<String>,
}

use event_stream::Publishable;
use serde::{Deserialize, Serialize};

use event_stream::EventMetaData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreated {
    pub email: String,
    pub phone: Option<String>,
    pub country: Option<String>,
}

impl Publishable for UserCreated {
    const SUBJECT: &'static str = "user.user.created";
}

impl Publishable for UserUpdated {
    const SUBJECT: &'static str = "user.user.updated";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfirmed {
    pub email: String,
}

impl Publishable for EmailConfirmed {
    const SUBJECT: &'static str = "users.email.confirmed";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenReleased {
    pub link: String,
    pub token: u32,
}

impl Publishable for TokenReleased {
    const SUBJECT: &'static str = "emails.token.released";
}

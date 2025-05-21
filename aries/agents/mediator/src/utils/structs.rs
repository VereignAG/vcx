use serde::{Deserialize, Serialize};

pub type VerKey = String;

#[derive(Serialize, Deserialize)]
pub struct URLInvitation {
    pub invitation: String,
}

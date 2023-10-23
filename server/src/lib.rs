use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct LoginRequest {
    pub id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct LoginResponse {
    pub ok: bool,
    pub err: String,
    pub token: String,
}

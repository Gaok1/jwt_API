use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // subject (neste caso, username ou ID)
    pub exp: usize,    // expiration timestamp
}
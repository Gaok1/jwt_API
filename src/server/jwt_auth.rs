use std::error::Error;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use super::claim::Claims;



pub static SECRET : &[u8] = b"secret"; //jeito gigola, mas o foco é usar o jwt e não a segurança pura
pub static ALGORITHM : Algorithm = Algorithm::HS256;

pub struct AuthUser {
    id : i32,
}
impl AuthUser{
    pub fn new(id : i32) -> Self {
        Self { id }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}

pub fn validate_token(token : &str) -> Result<AuthUser, Box<dyn Error>> {
    let token = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(SECRET),&Validation::new(ALGORITHM) );
    match token {
        Ok(token) => Ok(AuthUser::new(token.claims.sub.parse::<i32>().unwrap())),
        Err(e) => Err(Box::new(e)),
    }
    
}
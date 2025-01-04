use axum::{http::StatusCode, Json, Router};

use chrono::{Duration, Utc};

use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{db::database::DATABASE, model::user::User, server::claim::Claims};

use super::structs::{LoginRequest, RegisterRequest};


pub fn add_routes(app:Router) -> Router {
    app.route("/register", axum::routing::post(register_handler))
        .route("/login", axum::routing::post(login_handler))
}

async fn register_handler(Json(payload): Json<RegisterRequest>) -> Result<Json<String>, (StatusCode, String)> {
    // Verifica se já existe usuário com esse username
    let mut db = DATABASE.lock().unwrap();
    
    if let Some(_) = db.get_user_by_username(&payload.username) {
        return Err((StatusCode::BAD_REQUEST, "Username already taken".to_string()));
    }

    // Caso não exista, cria novo user
    let new_user = User::new( payload.username, payload.password);
    let new_id = db.insert_user(new_user);

    Ok(Json(format!("User created with ID = {}", new_id)))
}


async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Json<String>, (StatusCode, String)> {
    let db = DATABASE.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB lock poisoned".to_string()))?;

    // Busca user pelo username
    let user = db.get_user_by_username(&payload.username)
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // Verifica password
    if user.password() != payload.password {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    //  gera JWT
    let expiration = Utc::now() + Duration::minutes(30); 
    let claims = Claims { // jwt token payload
        sub: user.id().to_string() , 
        exp: expiration.timestamp() as usize,
    };

    // Gera token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(crate::server::jwt_auth::SECRET),
    ).map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(token))
}

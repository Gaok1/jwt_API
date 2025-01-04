use axum::{http::StatusCode, Json, Router};

use crate::{
    db::database::DATABASE,
    model::{event::Event, user},
    server::jwt_auth::validate_token,
};

use super::structs::{ConsultEventRequest, DeleteEventRequest, WholeEventRequest};

pub fn add_routes(app: Router) -> Router {
    app.route("/event", axum::routing::get(get_events))
        .route("/event", axum::routing::post(create_event))
        .route("/event", axum::routing::put(update_event))
        .route("/event", axum::routing::delete(delete_event))
}

async fn get_events(
    Json(request): Json<ConsultEventRequest>,
) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    let token = request.token();
    let _ = validate_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    let db = DATABASE.lock().unwrap();
    let event = db.get_event_by_name_id(request.event_name(), request.user_id());
    match event {
        Some(events) => Ok(Json(events.into_iter().cloned().collect())),
        None => Err((StatusCode::NOT_FOUND, "Event not found".to_string())),
    }
}

async fn create_event(
    Json(request): Json<WholeEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let token = request.token();
    let _ = validate_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let mut db = DATABASE.lock().unwrap();
    let mut event = request.event().clone();
    event.generate_id(); // ID não é gerado pelo cliente, então é necessário
    let id = event.id();
    db.insert_event(event);
    Ok(Json(format!("Event created with ID = {id}")))
}

async fn update_event(
    Json(request): Json<WholeEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let token = request.token();
    let _ = validate_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let mut db = DATABASE.lock().unwrap();
    let event = request.event().clone();
    db.insert_event(event); // sobrescreve o evento, pois o id é o mesmo e o hashmap não permite duplicatas
    Ok(Json("Event updated".to_string()))
}

async fn delete_event(
    Json(request): Json<DeleteEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let token = request.token();
    let _ = validate_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    let mut db = DATABASE.lock().unwrap();
    let result = db.delete_event_by_id(request.event_id().parse::<i32>().unwrap());
    match result {
        Ok(_) => Ok(Json("Event deleted".to_string())),
        Err(_) => Err((StatusCode::NOT_FOUND, "Event not found".to_string())),
    }
}

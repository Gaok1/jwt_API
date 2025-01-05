use axum::{http::StatusCode, Json, Router};
use axum_extra::{headers::Authorization, TypedHeader};

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

use axum::extract::Query;

async fn get_events(
    TypedHeader(Authorization(token)): TypedHeader<Authorization<String>>,
    Query(params): Query<ConsultEventRequest>,
) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    // Aqui 'params.event_name' virá da query string, por ex: /event?event_name=Teste
    let auth = validate_token(&token).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    
    let events = DATABASE.lock()
        .unwrap()
        .get_event_by_name_id(&params.event_name(), auth.id());

    Ok(Json(events.into_iter().cloned().collect()))
}

async fn create_event(
    TypedHeader(Authorization(token)) :TypedHeader<Authorization<Bearer>>,
    Json(request) : Json<WholeEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let auth = validate_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    let event = Event::new_event(request.event_name().to_owned(), request.description().to_owned(), request.event_type(), request.date().to_owned(), auth.id());
    DATABASE.lock().unwrap().insert_event(event);
    Ok(Json("Event created".to_string()))
}

async fn update_event(
    TypedHeader(Authorization(token)) :TypedHeader<Authorization<String>>,
    Json(request): Json<WholeEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let auth = validate_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    let event = Event::new_event(request.event_name().to_owned(), request.description().to_owned(), request.event_type(), request.date().to_owned(), auth.id());
    DATABASE.lock().unwrap().insert_event(event);
    Ok(Json("Event updated".to_string()))
}

async fn delete_event(
    TypedHeader(Authorization(token)) :TypedHeader<Authorization<String>>,
    Json(request): Json<DeleteEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let auth = validate_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    let event_id = request.event_id().parse().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let event = DATABASE.lock().unwrap().get_event_by_id(event_id).ok_or((StatusCode::NOT_FOUND, "Event not found".to_string()))?;
    if event.user_id() != auth.id() {
        return Err((StatusCode::FORBIDDEN, "You can't delete this event".to_string()));
    }
    DATABASE.lock().unwrap().delete_event_by_id(event_id).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete event".to_string()))?;
    Ok(Json("Event deleted".to_string()))
}

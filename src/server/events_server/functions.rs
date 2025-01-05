use axum::{
    async_trait,
    debug_handler,
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    routing::{get, post, put, delete},
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

use crate::{
    db::database::DATABASE,
    model::event::Event,
    server::jwt_auth::validate_token,
};

use super::structs::{ConsultEventRequest, DeleteEventRequest, WholeEventRequest};

/// Adiciona as rotas referentes a `event`
pub fn add_routes(app: Router) -> Router {
    app.route("/event", get(get_events))
        .route("/event", post(create_event))
        .route("/event", put(update_event))
        .route("/event", delete(delete_event))
}

#[debug_handler]
async fn get_events(
    TypedHeader(Authorization(token)): TypedHeader<Authorization<String>>,
    Query(params): Query<ConsultEventRequest>,
) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    // Validate token
    let auth = validate_token(&token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Obtém eventos por nome & ID do usuário
    let events = DATABASE.lock()
        .unwrap()
        .get_event_by_name_id(&params.event_name(), auth.id());

    // Transforma em Json
    Ok(Json(events.into_iter().cloned().collect()))
}

#[debug_handler]
async fn create_event(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(request): Json<WholeEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    // Extrai o token
    let auth = validate_token(bearer.token())
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Cria um novo Event
    let event = Event::new_event(
        request.event_name().to_owned(),
        request.description().to_owned(),
        request.event_type(),
        request.date().to_owned(),
        auth.id(),
    );

    // Insere no banco
    DATABASE.lock().unwrap().insert_event(event);

    Ok(Json("Event created".to_string()))
}

#[debug_handler]
async fn update_event(
    TypedHeader(Authorization(token)): TypedHeader<Authorization<String>>,
    Json(request): Json<WholeEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let auth = validate_token(&token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let event = Event::new_event(
        request.event_name().to_owned(),
        request.description().to_owned(),
        request.event_type(),
        request.date().to_owned(),
        auth.id(),
    );

    DATABASE.lock().unwrap().insert_event(event);

    Ok(Json("Event updated".to_string()))
}

#[debug_handler]
async fn delete_event(
    TypedHeader(Authorization(token)): TypedHeader<Authorization<String>>,
    Json(request): Json<DeleteEventRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let auth = validate_token(&token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Tenta fazer parse do event_id
    let event_id = request.event_id().parse()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Busca no banco
    let event = DATABASE
        .lock()
        .unwrap()
        .get_event_by_id(event_id)
        .ok_or((StatusCode::NOT_FOUND, "Event not found".to_string()))?;

    // Verifica se o evento pertence mesmo ao usuário
    if event.user_id() != auth.id() {
        return Err((StatusCode::FORBIDDEN, "You can't delete this event".to_string()));
    }

    // Se der certo, deleta
    DATABASE
        .lock()
        .unwrap()
        .delete_event_by_id(event_id)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete event".to_string()))?;

    Ok(Json("Event deleted".to_string()))
}

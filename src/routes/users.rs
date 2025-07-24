use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::users_controller::{
    user_create_controller, user_delete_controller, user_index_controller, user_update_controller,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/users", get(user_index_controller))
        .route("/api/users/create", post(user_create_controller))
        .route("/api/users/:id/update", post(user_update_controller))
        .route("/api/users/:id/delete", post(user_delete_controller))
}
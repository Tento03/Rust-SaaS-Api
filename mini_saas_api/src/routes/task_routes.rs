use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::task_handler, AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/tasks",
            post(task_handler::create_task).get(task_handler::get_tasks),
        )
        .route(
            "/tasks/:id",
            get(task_handler::get_task_by_id)
                .put(task_handler::update_task)
                .delete(task_handler::delete_task),
        )
        .with_state(state)
}

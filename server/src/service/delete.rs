use crate::app_state::AppState;
use crate::db::user;
use crate::db::user::DeleteForm;
use actix_identity::Identity;
use actix_web::{delete, http, web, Error, HttpResponse};

#[delete("/delete_user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(user_id) = id.identity() {
        let user_no = user_id.parse().unwrap();
        let _ = user::delete_user(user_no, (*form).clone(), state.pool.clone());
        id.forget();
        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish()
            .into_body())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

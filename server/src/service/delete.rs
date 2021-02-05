use crate::app_state::AppState;
use crate::db::user;
use crate::db::user::DeleteForm;
use actix_identity::Identity;
use actix_web::{delete, web, Error, HttpResponse};

#[delete("/delete-user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if id.identity().is_some() {
        user::delete_user((*form).clone(), state.pool.clone())?;
        id.forget();
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

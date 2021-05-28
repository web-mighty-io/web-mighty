use crate::app_state::AppState;
use crate::db::user;
use crate::db::user::DeleteForm;
use actix_identity::Identity;
use actix_web::{delete, web, Error, HttpResponse};

pub struct DeleteUserForm {
    password: String,
}

#[delete("/delete-user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteUserForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        user::delete_user(
            &DeleteForm {
                user_no: id.parse().unwrap(),
                password: form.password.clone(),
            },
            state.pool.clone(),
        )?;
        id.forget();
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

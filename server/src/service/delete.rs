use crate::actor::db::{Delete, DeleteForm};
use crate::app_state::AppState;
use actix_identity::Identity;
use actix_web::{delete, http, web, Error, HttpResponse};
use futures::TryFutureExt;

#[delete("/delete_user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(user_id) = id.identity() {
        let _ = db::user::delete(
            user_id.parse().map_err(|_| Error::from(()))?,
            &form,
            (**db_pool).clone(),
        )
        .await?;
        id.forget();
        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish()
            .into_body())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

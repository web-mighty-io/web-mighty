use crate::actor::db::{Delete, DeleteForm};
use crate::app_state::AppState;
use actix::prelude::*;
use actix_identity::Identity;
use actix_web::{delete, http, web, Error, HttpResponse};
use std::future::IntoFuture;

#[delete("/delete_user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(user_id) = id.identity() {
        state
            .db
            .send(Delete(user_id.parse().map_err(|_| Error::from(()))?, (*form).clone()))
            .into_future()
            .await
            .unwrap()?;
        id.forget();
        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish()
            .into_body())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

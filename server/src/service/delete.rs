use crate::db;
use crate::db::user::DeleteForm;
use actix_identity::Identity;
use actix_web::{delete, http, web, Error, HttpResponse};
use deadpool_postgres::Pool;

#[delete("/delete_user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteForm>,
    db_pool: web::Data<Pool>,
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

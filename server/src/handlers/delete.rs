use crate::db;
use crate::handlers::DeleteUserForm;
use actix_identity::Identity;
use actix_web::{delete, http, web, Error, HttpResponse};
use deadpool_postgres::Pool;

#[delete("/delete_user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteUserForm>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    if let Some(user_id) = id.identity() {
        let _ = db::delete_user(&form, user_id, &db_pool).await?;
        id.forget();
        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish()
            .into_body())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

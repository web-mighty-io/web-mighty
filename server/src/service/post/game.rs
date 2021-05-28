use crate::actor::hub::MakeRoom;
use crate::{app_state::AppState, dev::*};
use actix_identity::Identity;
use actix_web::{post, web, HttpResponse};
use futures::TryFutureExt;
use serde_json::json;

#[post("/make-room")]
pub async fn make_room(
    id: Identity,
    web::Form(form): web::Form<MakeRoom>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let room_id = state.hub.send(form).into_future().await?;
        Ok(HttpResponse::Ok().body(serde_json::to_string(&json!({ "room_id": room_id }))))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

async fn join_room_impl(id: UserNo, room_id: RoomId, )

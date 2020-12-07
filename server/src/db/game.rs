use crate::db::Result;
use deadpool_postgres::Pool;
use mighty::rule::Rule;
use std::sync::Arc;
use tokio_postgres::types::Json;
use uuid::Uuid;

pub struct SaveGameForm {
    pub game_id: Uuid,
    pub room_id: Uuid,
    pub room_name: String,
    pub users: Vec<u32>,
    pub is_rank: bool,
    pub rule: Json<Rule>,
}

pub async fn make_game(form: SaveGameForm, pool: Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("INSERT INTO game (id, room_id, room_name, users, is_rank, rule) VALUES ($1, $2, $3, $4, $5, $6);")
        .await?;
    let _ = client
        .query(
            &stmt,
            &[
                &form.game_id,
                &form.room_id,
                &form.room_name,
                &form.users,
                &form.is_rank,
                &form.rule,
            ],
        )
        .await?;
    Ok(())
}

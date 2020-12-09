use crate::db::Result;
use deadpool_postgres::Pool;
use mighty::rule::Rule;
use mighty::State;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::Json;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct SaveStateForm {
    pub game_id: Uuid,
    pub room_id: Uuid,
    pub number: u32,
    pub state: State,
}

pub async fn save_state(form: SaveStateForm, pool: Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("INSERT INTO record (game_id, room_id, number, state) VALUES ($1, $2, $3, $4);")
        .await?;
    let _ = client
        .query(&stmt, &[&form.game_id, &form.room_id, &form.number, &form.state])
        .await?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct ChangeRatingForm {
    pub user_id: u32,
    pub game_id: Uuid,
    pub diff: u32,
    pub rating: u32,
}

pub async fn change_rating(form: ChangeRatingForm, pool: Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("INSERT INTO rating (user_id, game_id, diff, rating) VALUES ($1, $2, $3, $4);")
        .await?;
    let _ = client
        .query(&stmt, &[&form.user_id, &form.game_id, &form.diff, &form.rating])
        .await?;
    Ok(())
}

use crate::dev::*;
use actix::prelude::*;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_postgres::types::Json;
use uuid::Uuid;
use mighty::prelude::{State, Rule};

#[derive(Deserialize, Serialize, Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct ChangeRatingForm {
    pub user_no: u32,
    pub game_id: Uuid,
    pub diff: u32,
    pub rating: u32,
}

pub async fn change_rating(form: ChangeRatingForm, pool: Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("INSERT INTO rating (user_no, game_id, diff, rating) VALUES ($1, $2, $3, $4);")
        .await?;
    let _ = client
        .query(&stmt, &[&form.user_no, &form.game_id, &form.diff, &form.rating])
        .await?;

    let client = pool.get().await?;
    let stmt = client.prepare("UPDATE users SET rating=$1 WHERE no=$2;").await?;
    let _ = client.query(&stmt, &[&form.rating, &form.user_no]).await?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Message)]
#[rtype(result = "Result<Vec<Rating>>")]
pub struct GetRatingForm {
    pub user_no: u32,
    pub start: SystemTime,
    pub end: SystemTime,
}

#[derive(Deserialize, Serialize, Clone, MessageResponse)]
pub struct Rating {
    pub game_id: Uuid,
    pub diff: u32,
    pub rating: u32,
    pub time: SystemTime,
}

pub async fn get_rating(form: GetRatingForm, pool: Pool) -> Result<Vec<Rating>> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("SELECT game_id, diff, rating, time, FROM rating WHERE user_no=$1 AND $2<=time AND time<=$3 ORDER BY time ASC")
        .await?;
    let res = client.query(&stmt, &[&form.user_no, &form.start, &form.end]).await?;
    Ok(res
        .iter()
        .map(|r| Rating {
            game_id: r.get(0),
            diff: r.get(1),
            rating: r.get(2),
            time: r.get(3),
        })
        .collect())
}

#[derive(Deserialize, Serialize, Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct MakeGameForm {
    pub game_id: Uuid,
    pub room_id: Uuid,
    pub room_name: String,
    pub users: Vec<u32>,
    pub is_rank: bool,
    pub rule: Rule,
}

pub async fn make_game(form: MakeGameForm, pool: Pool) -> Result<()> {
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
                &Json(form.rule),
            ],
        )
        .await?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Message)]
#[rtype(result = "Result<()>")]
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
        .query(&stmt, &[&form.game_id, &form.room_id, &form.number, &Json(form.state)])
        .await?;
    Ok(())
}

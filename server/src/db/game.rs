use crate::dev::*;
use mighty::prelude::{Rule, State};
use postgres::types::Json;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct ChangeRatingForm {
    pub user_no: u32,
    pub game_id: Uuid,
    pub diff: u32,
    pub rating: u32,
}

pub fn change_rating(form: ChangeRatingForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("INSERT INTO rating (user_no, game_id, diff, rating) VALUES ($1, $2, $3, $4);")?;
    let _ = client.query(&stmt, &[&form.user_no, &form.game_id, &form.diff, &form.rating])?;

    let mut client = pool.get()?;
    let stmt = client.prepare("UPDATE users SET rating=$1 WHERE no=$2;")?;
    let _ = client.query(&stmt, &[&form.rating, &form.user_no])?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetRatingForm {
    pub user_no: u32,
    pub start: SystemTime,
    pub end: SystemTime,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Rating {
    pub game_id: Uuid,
    pub diff: u32,
    pub rating: u32,
    pub time: SystemTime,
}

pub fn get_rating(form: GetRatingForm, pool: Pool) -> Result<Vec<Rating>> {
    let mut client = pool.get()?;
    let stmt = client.prepare(
        "SELECT game_id, diff, rating, time, FROM rating WHERE user_no=$1 AND $2<=time AND time<=$3 ORDER BY time ASC",
    )?;
    let res = client.query(&stmt, &[&form.user_no, &form.start, &form.end])?;
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

#[derive(Deserialize, Serialize, Clone)]
pub struct MakeGameForm {
    pub game_id: Uuid,
    pub room_id: Uuid,
    pub room_name: String,
    pub users: Vec<u32>,
    pub is_rank: bool,
    pub rule: Rule,
}

pub fn make_game(form: MakeGameForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client
        .prepare("INSERT INTO game (id, room_id, room_name, users, is_rank, rule) VALUES ($1, $2, $3, $4, $5, $6);")?;
    let _ = client.query(
        &stmt,
        &[
            &form.game_id,
            &form.room_id,
            &form.room_name,
            &form.users,
            &form.is_rank,
            &Json(form.rule),
        ],
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SaveStateForm {
    pub game_id: Uuid,
    pub room_id: Uuid,
    pub number: u32,
    pub state: State,
}

pub fn save_state(form: SaveStateForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("INSERT INTO record (game_id, room_id, number, state) VALUES ($1, $2, $3, $4);")?;
    let _ = client.query(&stmt, &[&form.game_id, &form.room_id, &form.number, &Json(form.state)])?;
    Ok(())
}

use crate::dev::*;
use mighty::prelude::{Rule, State};
use postgres::types::Json;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Deserialize, Serialize, Clone)]
pub struct ChangeRatingForm {
    pub user_no: u32,
    pub game_id: GameId,
    pub diff: u32,
    pub rating: u32,
}

pub fn change_rating(form: ChangeRatingForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("INSERT INTO rating (user_no, game_id, diff, rating) VALUES ($1, $2, $3, $4);")?;
    let _ = client.query(
        &stmt,
        &[&form.user_no, &form.game_id.to_string(), &form.diff, &form.rating],
    )?;

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
    pub game_id: GameId,
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
            game_id: GameId::from_str(r.get(0)).unwrap(),
            diff: r.get(1),
            rating: r.get(2),
            time: r.get(3),
        })
        .collect())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MakeGameForm {
    pub game_id: GameId,
    pub room_id: RoomUid,
    pub room_name: String,
    pub users: Vec<u32>,
    pub is_rank: bool,
    pub rule: Rule,
}

pub fn make_game(form: MakeGameForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 is_gaming FROM curr_rooms WHERE uid=$1")?;
    let res = client.query(&stmt, &[&form.room_id.to_string()])?;
    let is_gaming: bool = res[0].get(0);
    ensure!(!is_gaming, "game is already going on the room");
    let mut client = pool.get()?;
    let stmt = client
        .prepare("INSERT INTO games (id, room_id, room_name, users, is_rank, rule) VALUES ($1, $2, $3, $4, $5, $6);")?;
    let _ = client.query(
        &stmt,
        &[
            &form.game_id.to_string(),
            &form.room_id.to_string(),
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
    pub game_id: GameId,
    pub room_id: RoomUid,
    pub number: u32,
    pub state: State,
}

pub fn save_state(form: SaveStateForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("INSERT INTO record (game_id, room_id, number, state) VALUES ($1, $2, $3, $4);")?;
    let _ = client.query(
        &stmt,
        &[
            &form.game_id.to_string(),
            &form.room_id.to_string(),
            &form.number,
            &Json(form.state),
        ],
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetRuleForm {
    pub rule_hash: RuleHash,
}

pub fn get_rule(form: GetRuleForm, pool: Pool) -> Result<Rule> {
    let mut client = pool.get()?;
    let stmt = client.prepare("")?; // todo
    let res = client.query(&stmt, &[&form.rule_hash.to_string()])?;
    ensure!(res.len() == 1, "no rule found");
    let rule: Json<Rule> = res[0].get(0);
    Ok(rule.0)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SaveRuleForm {
    pub rule: Rule,
}

pub fn save_rule(form: SaveRuleForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("")?; // todo
    let _ = client.query(&stmt, &[&Json(form.rule)]);
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MakeRoomForm {
    pub uid: RoomUid,
    pub id: RoomId,
    pub name: String,
    pub user_no: UserNo,
    pub rule: Rule,
}

pub fn make_room(form: MakeRoomForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client
        .prepare("INSERT INTO curr_rooms (uid, id, name, master, users, rule) VALUES ($1, $2, $3, $4, $5, $6);")?;
    let _ = client.query(
        &stmt,
        &[
            &form.uid.to_string(),
            &form.id.0,
            &form.name,
            &form.user_no.0,
            &vec![&form.user_no.0],
            &Json(form.rule),
        ],
    )?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetInRoomForm {
    pub room_id: RoomId,
}

pub fn get_into_room(form: GetInRoomForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 users_cnt FROM curr_rooms WHERE id=$1;")?;
    let res = client.query(&stmt, &[&form.room_id.0])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "no rooms exists"))?;
    let users_cnt: u32 = row.get(0);
    let mut client = pool.get()?;
    let stmt = client.prepare("UPDATE curr_rooms SET users_cnt=$1 id=$2;")?;
    let _ = client.query(&stmt, &[&users_cnt, &form.room_id.0])?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LeaveRoomForm {
    pub room_id: RoomId,
}

pub fn leave_room(form: LeaveRoomForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 users_cnt FROM curr_rooms WHERE id=$1;")?;
    let res = client.query(&stmt, &[&form.room_id.0])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "no rooms exists"))?;
    let users_cnt: u32 = row.get(0);
    let mut client = pool.get()?;
    let stmt = client.prepare("UPDATE curr_rooms SET users_cnt=$1 id=$2;")?;
    let _ = client.query(&stmt, &[&users_cnt, &form.room_id.0])?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChangeRoomInfoForm {
    pub room_id: RoomId,
    pub name: Option<String>,
    pub rule: Option<Rule>,
}

pub fn change_room_info(form: ChangeRoomInfoForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 name, rule FROM users WHERE id=$1;")?;
    let res = client.query(&stmt, &[&form.room_id.0])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "login failed"))?;
    let name = form.name.clone().unwrap_or_else(|| row.get(0));
    let rule = if form.rule == None {
        row.get(1)
    } else {
        Json(form.rule.clone().unwrap())
    };

    let mut client = pool.get()?;
    let stmt = client.prepare("UPDATE users SET name=$1, rule=$2 WHERE id=$3;")?;
    let _ = client.query(&stmt, &[&name, &rule, &form.room_id.0])?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetRoomListForm {
    pub room_id: RoomId,
}

pub fn get_room_list(_form: GetRoomListForm, _pool: Pool) -> Result<()> {
    Ok(())
}

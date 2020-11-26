use actix_web::{HttpResponse, ResponseError};
use bitflags::bitflags;
use deadpool_postgres::{Pool, PoolError};
use derive_more::Display;
use tokio_postgres::Error;

#[derive(Debug, Display)]
pub enum UserInfoError {
    PoolError(PoolError),
    InvalidForm,
    NoUser,
}

impl From<PoolError> for UserInfoError {
    fn from(e: PoolError) -> Self {
        UserInfoError::PoolError(e)
    }
}

impl From<tokio_postgres::Error> for UserInfoError {
    fn from(e: Error) -> Self {
        Self::from(PoolError::from(e))
    }
}

impl ResponseError for UserInfoError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserInfoError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            UserInfoError::InvalidForm => HttpResponse::BadRequest().body("invalid form"),
            UserInfoError::NoUser => HttpResponse::Unauthorized().body("no user found"),
        }
    }
}

bitflags! {
    pub struct UserInfoOption: u8 {
        const NO      = 0b000001;
        const ID      = 0b000010;
        const NAME    = 0b000100;
        const EMAIL   = 0b001000;
        const EMAIL_V = 0b010000;
        const RATING  = 0b100000;
    }
}

enum UserInfoInput {
    No(u32),
    Id(String),
    Email(String),
}

pub struct UserInfoForm {
    input: UserInfoInput,
    options: UserInfoOption,
}

impl UserInfoForm {
    pub fn from_no(no: u32, options: UserInfoOption) -> UserInfoForm {
        UserInfoForm {
            input: UserInfoInput::No(no),
            options,
        }
    }

    pub fn from_id(id: String, options: UserInfoOption) -> UserInfoForm {
        UserInfoForm {
            input: UserInfoInput::Id(id),
            options,
        }
    }

    pub fn from_email(email: String, options: UserInfoOption) -> UserInfoForm {
        UserInfoForm {
            input: UserInfoInput::Email(email),
            options,
        }
    }
}

pub struct UserInfo {
    pub no: Option<u32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_v: Option<bool>,
    pub rating: Option<u32>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl UserInfo {
    pub fn new() -> UserInfo {
        UserInfo {
            no: None,
            id: None,
            name: None,
            email: None,
            email_v: None,
            rating: None,
        }
    }
}

pub async fn user_info(form: &UserInfoForm, pool: &Pool) -> Result<UserInfo, UserInfoError> {
    let client = pool.get().await?;
    let res = match &form.input {
        UserInfoInput::No(no) => {
            let stmt = client.prepare("SELECT * FROM users WHERE no=$1").await?;
            client.query(&stmt, &[no]).await?
        }
        UserInfoInput::Id(id) => {
            let stmt = client.prepare("SELECT * FROM users WHERE id=$1").await?;
            client.query(&stmt, &[id]).await?
        }
        UserInfoInput::Email(email) => {
            let stmt = client.prepare("SELECT * FROM users WHERE email=$1").await?;
            client.query(&stmt, &[email]).await?
        }
    };

    if res.is_empty() {
        return Err(UserInfoError::NoUser);
    }

    let mut info = UserInfo::new();

    if form.options.contains(UserInfoOption::NO) {
        info.no = Some(res[0].get(0));
    }
    if form.options.contains(UserInfoOption::ID) {
        info.id = Some(res[0].get(1));
    }
    if form.options.contains(UserInfoOption::NAME) {
        info.name = Some(res[0].get(2));
    }
    if form.options.contains(UserInfoOption::EMAIL) {
        info.email = Some(res[0].get(3));
    }
    if form.options.contains(UserInfoOption::EMAIL_V) {
        info.email_v = Some(res[0].get(4));
    }
    if form.options.contains(UserInfoOption::RATING) {
        info.rating = Some(res[0].get(5));
    }

    Ok(info)
}

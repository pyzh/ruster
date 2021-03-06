use actix_web::{Error,actix::Message};
use utils::schema::users;
use chrono::{Utc, NaiveDateTime};
use model::response::{Msgs,MyError, SigninMsgs,UserIdMsgs, UserInfoMsgs,
                      UserThemesMsgs,UserCommentsMsgs,UserSavesMsgs,UserMessagesMsgs};

#[derive(Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize,Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct SigninUser {
    pub username: String,
    pub password: String,
    pub code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    pub user_id: i32,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct UserUpdate {
    pub user_id: i32,
    pub newname: String,
    pub newmail: String,
    pub newpassword: String,
    pub confirm_newpassword: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDelete {
    pub user_id: String,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserThemes {
    pub user_id: i32,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserComments {
    pub user_id: i32,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserSaves {
    pub user_id: i32,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserMessages {
    pub user_id: i32,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct UserMessagesReadall{
    pub user_id: i32,
    pub messages_unread_ids: Vec<i32>,
}

impl Message for SignupUser {
    type Result = Result<Msgs, Error>;
}

impl Message for SigninUser {
    type Result = Result<SigninMsgs, Error>;
}

impl Message for UserInfo {
    type Result = Result<UserInfoMsgs, Error>;
}
impl Message for UserId {
    type Result = Result<UserIdMsgs, Error>;
}

impl Message for UserUpdate {
    type Result = Result<Msgs, Error>;
}
impl Message for UserDelete {
    type Result = Result<Msgs, MyError>;
}
impl Message for UserThemes {
    type Result = Result<UserThemesMsgs, Error>;
}
impl Message for UserComments {
    type Result = Result<UserCommentsMsgs, Error>;
}
impl Message for UserSaves {
    type Result = Result<UserSavesMsgs, Error>;
}
impl Message for UserMessages {
    type Result = Result<UserMessagesMsgs, Error>;
}
impl Message for UserMessagesReadall {
    type Result = Result<Msgs, Error>;
}

impl User {
    pub fn new() -> User {
        User {
            id: 0,
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}
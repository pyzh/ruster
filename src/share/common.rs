pub const PAGE_SIZE: i32 = 33;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
}
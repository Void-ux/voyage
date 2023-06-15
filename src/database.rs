use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Command {
    pub guild_id: i64,
    pub author_id: i64,
    pub used: NaiveDateTime,
    pub prefix: String,
    pub command: String,
    pub slash: bool,
    pub failed: Option<bool>
}

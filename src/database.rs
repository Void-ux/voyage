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

#[derive(Debug, sqlx::FromRow)]
pub struct ExploreItem {
    pub id: i16,
    pub name: String,
    pub tier: String,
    pub sell_price: i64,
    pub health: Option<i16>,
    pub emoji_id: i64,
    pub emoji_name: String
}

impl ExploreItem {
    pub fn emoji(&self) -> String {
        format!("<:{}:{}>", &self.emoji_name, &self.emoji_id)
    }
}

use std::{collections::HashMap, sync::Mutex};

pub struct Data {
    pub pool: sqlx::postgres::PgPool,
    pub system_info: Mutex<sysinfo::System>
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name="bal_type", rename_all="lowercase")]
pub enum BalType {
    Wallet,
    Bank
}

impl Data {
    pub async fn fetch_balance(&self, user_id: i64, guild_id: i64, bal_type: BalType) -> i64 {
        sqlx::query!(
            "SELECT SUM(coins)::BIGINT FROM economy WHERE user_id=$1 AND guild_id=$2 AND bal_type=$3",
            user_id,
            guild_id,
            bal_type as BalType
        ).fetch_one(&self.pool).await.unwrap().sum.unwrap_or(0)
    }

    pub async fn edit_balance(&self, user_id: i64, guild_id: i64, amount: i64, bal_type: BalType) {
        sqlx::query!(
            "INSERT INTO economy (user_id, guild_id, coins, bal_type, time, msg, vc_session, daily, weekly, monthly)
            VALUES ($1, $2, $3, $4, NOW(), NULL, NULL, FALSE, FALSE, FALSE)",
            user_id,
            guild_id,
            amount,
            bal_type as BalType
        ).execute(&self.pool).await.unwrap();
    }
}

pub type Command = poise::Command<Data, CommandError>;
pub type CommandError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;

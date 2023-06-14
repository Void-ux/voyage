use std::{sync::Mutex, collections::HashMap};

pub struct Data {
    pub votes: Mutex<HashMap<String, u32>>,
    pub pool: sqlx::sqlite::SqlitePool
}

pub type Command = poise::Command<Data, CommandError>;
pub type CommandError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;

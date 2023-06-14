use sqlx::Row;

use crate::{Context, CommandError, utils::TabularData};
use crate::structs::Command;

#[poise::command(prefix_command, slash_command, subcommands("all"))]
pub async fn stats(
    _ctx: Context<'_>,
) -> Result<(), CommandError> { 
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn all(
    ctx: Context<'_>,
) -> Result<(), CommandError> {
    let data = sqlx::query("SELECT * FROM commands").fetch_all(&ctx.data().pool).await?;

    let mut table = TabularData::new();
    table.set_columns(vec!["command".into(), "author".into()]);
    for row in data {
        let command_name: String = row.try_get("command_name").unwrap();
        let author_id: i64 = row.try_get("author_id").unwrap();
        table.add_row(vec![command_name, author_id.to_string()]);
    }

    ctx.say(format!("```\n{}```", table.render())).await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [stats()]
}

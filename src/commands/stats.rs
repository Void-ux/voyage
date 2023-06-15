use crate::structs::Command;
use crate::{utils::TabularData, CommandError, Context};
use crate::database::Command as DBCommand;

/// Displays command usage metrics.
#[poise::command(prefix_command, slash_command, subcommands("all"), owners_only)]
pub async fn stats(_ctx: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

/// Displays global command usage metrics in an optional time period.
#[poise::command(prefix_command, slash_command)]
pub async fn all(ctx: Context<'_>) -> Result<(), CommandError> {
    let data: Vec<DBCommand> = sqlx::query_as("SELECT * FROM commands")
        .fetch_all(&ctx.data().pool)
        .await?;

    let mut table = TabularData::new();
    table.set_columns(vec!["command".into(), "author".into()]);
    for command_ in data {
        table.add_row(vec![command_.command, command_.author_id.to_string()]);
    }

    ctx.say(format!("```\n{}```", table.render())).await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [stats()]
}

use crate::structs::{Command, CommandError, Context};
use crate::utils::TabularData;
use sqlx::{Row, Column};

/// Executes a SQL query and returns any data if any
/// NOTE: All select arguments must be of type `TEXT`
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn sql(ctx: Context<'_>, #[rest] query: String) -> Result<(), CommandError> {
    let mut table = TabularData::new();
    let data = sqlx::query(query.as_str()).fetch_all(&ctx.data().pool).await;
    match data {
        Ok(rows) => {
            let columns: Vec<String> = rows[0].columns().iter().map(|c| c.name().to_owned()).collect();
            table.set_columns(columns);

            for row in rows {
                let mut row_data: Vec<_> = vec![];
                for i in 0..row.columns().len() {
                    row_data.push(row.get(i));
                }
                table.add_row(row_data)
            }

            ctx.say(format!("```\n{}```", table.render())).await?;
        },
        Err(e) => {
            ctx.say(e.to_string()).await?;
        }
    }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [sql()]
}

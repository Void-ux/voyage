use crate::structs::Command;
use crate::{utils::TabularData, CommandError, Context};
use crate::database::Command as DBCommand;

use sysinfo::{SystemExt, ProcessExt};
use num_format::{Locale, ToFormattedString};

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

/// Displays various metrics regarding bot performance
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn bothealth(ctx: Context<'_>) -> Result<(), CommandError> {
    let (ram_usage, cpu_usage) = {
        let mut system_info = ctx.data().system_info.lock().unwrap();
        system_info.refresh_specifics(sysinfo::RefreshKind::new()
            .with_cpu(sysinfo::CpuRefreshKind::new().with_cpu_usage())
            .with_processes(sysinfo::ProcessRefreshKind::new())
            .with_memory()
        );

        let pid = sysinfo::get_current_pid().unwrap();
        let process = system_info.process(pid).unwrap();
        (process.memory() / 1024 / 1024, process.cpu_usage())
    };
    let pool = &ctx.data().pool;

    ctx.send(|b| b.embed(|e| e
        .colour(0xC0C0C0)
        .title("Bot Health Report")
        .description(format!(
            "Connections: {}\nConnections Idle: {}\nPool Closed: {}",
            pool.size(),
            pool.num_idle(),
            pool.is_closed()
        ))
        .field(
            "Process",
            format!(
                "CPU: {}%\nMemory: {} MB",
                cpu_usage,
                ram_usage.to_formatted_string(&Locale::en),
            ),
            false
        )
    )).await?;

    Ok(())
}

pub fn commands() -> [Command; 2] {
    [stats(), bothealth()]
}

use crate::structs::Command;
use crate::{CommandError, Context};
use crate::utils::divmod;
use sysinfo::{SystemExt, ProcessExt};
use chrono::Utc;

#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), CommandError> {
    let now = Utc::now();
    let diff = now - ctx.data().launch_time;
    let (hours, minutes) = divmod(diff.num_seconds() as usize, 3600);
    let (minutes, seconds) = divmod(minutes, 60);

    let ram_usage = {
        let mut system_info = ctx.data().system_info.lock().unwrap();
        system_info.refresh_specifics(sysinfo::RefreshKind::new()
            .with_processes(sysinfo::ProcessRefreshKind::new())
            .with_memory()
        );

        let pid = sysinfo::get_current_pid().unwrap();
        let process = system_info.process(pid).unwrap();
        process.memory() / 1024 / 1024
    };

    ctx.send(|b| b.embed(|e| e
        .colour(0x2D936C)
        .field(
            "Information",
            format!(
                "
                **Developer**: void_ux
                **Uptime**: {} hours, {} minutes and {} seconds
                ",
                hours,
                minutes,
                seconds
            ),
            false
        )
        .field(
            "Statistics",
            format!(
                "
                **Servers**: {}
                **Users**: {}
                **RAM**: {} Mb
                **Latency**: N/A
                ",
                ctx.serenity_context().cache.guild_count(),
                ctx.serenity_context().cache.user_count(),
                ram_usage
            ),
            false
        )
    )).await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [about()]
}

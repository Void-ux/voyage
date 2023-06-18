mod commands;
mod database;
mod structs;
mod utils;

use std::{env::var, sync::Mutex, time::Duration};
use poise::serenity_prelude as serenity;
use sysinfo::SystemExt;
use structs::{CommandError, Context, Data};

async fn on_error(error: poise::FrameworkError<'_, Data, CommandError>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = sqlx::postgres::PgPoolOptions::new();
    let pool = options
        .connect_with(
            sqlx::postgres::PgConnectOptions::new()
                .host(&var("POSTGRES_HOST").unwrap())
                .username(&var("POSTGRES_USER").unwrap())
                .database(&var("POSTGRES_DATABASE").unwrap())
                .password(&var("POSTGRES_PASSWORD").unwrap()),
        )
        .await
        .unwrap();

    let data = Data {
        pool,
        system_info: Mutex::new(sysinfo::System::new())
    };

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: commands::commands(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        /// This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Invoked command {}", ctx.command().qualified_name)
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                sqlx::query!(
                    "INSERT INTO commands (guild_id, channel_id, author_id, used, prefix, command, slash, failed)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, NULL)",
                    *ctx.guild_id().unwrap().as_u64() as i64,
                    *ctx.channel_id().as_u64() as i64,
                    *ctx.author().id.as_u64() as i64,
                    ctx.created_at().naive_utc(),
                    ctx.prefix(),
                    &ctx.command().qualified_name,
                    ctx.prefix() == "/",
                )
                .execute(&ctx.data().pool)
                .await
                .unwrap();
            })
        },
        event_handler: |_ctx, _event, _framework, _data| {
            Box::pin(async move {
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var."))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .options(options)
        .intents(serenity::GatewayIntents::all())
        .run()
        .await
        .unwrap();
}

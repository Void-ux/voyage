#![warn(clippy::str_to_string)]

mod commands;
mod structs;
mod utils;

use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};
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
    sqlx::query("CREATE TABLE IF NOT EXISTS commands(command_name TEXT, author_id BIGINT)")
        .execute(&pool)
        .await
        .unwrap();

    let data = Data {
        votes: Mutex::new(HashMap::new()),
        pool: pool,
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
                sqlx::query("INSERT INTO commands VALUES ($1, $2)")
                    .bind(&ctx.command().qualified_name)
                    .bind(*ctx.author().id.as_u64() as i64)
                    .execute(&ctx.data().pool)
                    .await
                    .unwrap();
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}...", ctx.command().qualified_name);
            })
        },
        /// Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        /// Enforce command checks even for owners (enforced by default)
        /// Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
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

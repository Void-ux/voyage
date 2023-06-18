use crate::structs::{Command, CommandError, BalType};
use crate::Context;
use poise::serenity_prelude as serenity;
use chrono::{Utc, Duration, NaiveDateTime};

/// Displays your wallet and bank balance
#[poise::command(prefix_command, slash_command, guild_only, aliases("bal"))]
pub async fn balance(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<(), CommandError> {
    let member = member.unwrap_or(ctx.author_member().await.unwrap().to_mut().to_owned());
    let wallet = ctx.data().fetch_balance(
        *member.user.id.as_u64() as i64,
        *member.guild_id.as_u64() as i64,
        BalType::Wallet
    ).await;

    let bank = ctx.data().fetch_balance(
        *member.user.id.as_u64() as i64,
        *member.guild_id.as_u64() as i64,
        BalType::Bank
    ).await;

    ctx.send(|b| b.embed(|e| e
        .description(format!("\u{200b}\n**Total** - {} {}\n\u{200b}", wallet + bank, "<:coin:1119247275093413940>"))
        .colour(0x8BC34A)
        .field("Wallet", format!("{}", wallet), true)
        .field("Bank", format!("{}", bank), true)
        .author(|a| a
            .name(member.display_name())
            .icon_url(member.avatar_url().unwrap_or(member.user.avatar_url().unwrap_or(member.user.default_avatar_url())))
        )
    )).await?;

    Ok(())
}

/// Deposit some money from your wallet into your bank for safekeeping
#[poise::command(prefix_command, slash_command, guild_only, aliases("dep"))]
pub async fn deposit(ctx: Context<'_>, amount: String) -> Result<(), CommandError> {
    let wallet = ctx.data().fetch_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        BalType::Wallet
    ).await;

    let to_deposit;
    if amount == "all" {
        to_deposit = wallet;
    } else {
        let parsed_amount = amount.parse::<i64>();
        match parsed_amount {
            Ok(_) => to_deposit = parsed_amount.unwrap(),
            Err(_) => {
                ctx.say("You have to provide a numeric value, or 'all' to deposit everything in your wallet!").await?;
                return Ok(())
            }
        }
    }

    if to_deposit < 1 {
        ctx.say("You can only deposit a minimum of one coin!").await?;
        return Ok(())
    }
    if to_deposit > wallet {
        ctx.say("You can't deposit more coins than you already have in your wallet!").await?;
        return Ok(())
    }

    ctx.data().edit_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        -to_deposit,
        BalType::Wallet
    ).await;
    ctx.data().edit_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        to_deposit,
        BalType::Bank
    ).await;

    ctx.say(format!("Deposited {} <:coin:1119247275093413940> into your bank", to_deposit)).await?;
    Ok(())
}

/// [dev] Edits the wallet balance of a user
#[poise::command(prefix_command, owners_only, guild_only, aliases("pay", "send"))]
pub async fn give(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
    amount: i64
) -> Result<(), CommandError> {
    let member = member.unwrap_or(ctx.author_member().await.unwrap().to_mut().to_owned());
    ctx.data().edit_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        amount,
        BalType::Wallet
    ).await;

    ctx.say(format!("Gave {} {} <:coin:1119247275093413940>", member.display_name(), amount)).await?;
    Ok(())
}

/// Withdraw some money from your bank into your wallet
#[poise::command(prefix_command, slash_command, guild_only, aliases("with"))]
pub async fn withdraw(ctx: Context<'_>, amount: String) -> Result<(), CommandError> {
    let bank = ctx.data().fetch_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        BalType::Bank
    ).await;

    let to_withdraw;
    if amount == "all" {
        to_withdraw = bank;
    } else {
        let parsed_amount = amount.parse::<i64>();
        match parsed_amount {
            Ok(_) => to_withdraw = parsed_amount.unwrap(),
            Err(_) => {
                ctx.say("You have to provide a numeric value, or 'all' to withdraw everything from your bank!").await?;
                return Ok(())
            }
        }
    }

    if to_withdraw < 1 {
        ctx.say("You can only withdraw a minimum of one coin!").await?;
        return Ok(())
    }
    if to_withdraw > bank {
        ctx.say("You can't withdraw more coins than you already have in your bank!").await?;
        return Ok(())
    }

    ctx.data().edit_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        -to_withdraw,
        BalType::Wallet
    ).await;
    ctx.data().edit_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        to_withdraw,
        BalType::Bank
    ).await;

    ctx.say(format!("Withdrew {} <:coin:1119247275093413940> from your bank", to_withdraw)).await?;
    Ok(())
}

/// Returns the floored quotient and the remainder of a division operation
fn divmod(dividend: usize, divisor: usize) -> (usize, usize) {
    (dividend / divisor, dividend - divisor * (dividend / divisor))
}

async fn daily_cooldown(ctx: Context<'_>) -> Result<bool, CommandError> {
    let last_execution = sqlx::query!(
        "SELECT used FROM commands WHERE author_id=$1 ORDER BY used DESC LIMIT 1",
        *ctx.author().id.as_u64() as i64
    ).fetch_optional(&ctx.data().pool).await?;

    match last_execution {
        Some(rec) => {
            let now = Utc::now().naive_utc();
            let until = NaiveDateTime::from_timestamp_opt(rec.used.unwrap().timestamp(), 0).unwrap() + Duration::days(1);
            if now > until {
                Ok(true)
            } else {
                let diff = until - now;
                let (hours, minutes) = divmod(diff.num_seconds() as usize, 3600);
                let (minutes, seconds) = divmod(minutes, 60);
                ctx.send(|b| b.embed(|e| e
                    .title("Error!")
                    .description(format!("You can't use this command for another {} hours, {} minutes and {} seconds", hours, minutes, seconds))
                    .thumbnail("https://cdn.discordapp.com/attachments/927190003061256224/960178856843702322/unknown.png")
                )).await?;
                Ok(false)
            }
        }
        None => Ok(true)
    }
}

/// Claim your daily coins and XP!
#[poise::command(prefix_command, slash_command, guild_only, check = "daily_cooldown")]
pub async fn daily(ctx: Context<'_>) -> Result<(), CommandError> {
    ctx.data().edit_balance(
        *ctx.author().id.as_u64() as i64,
        *ctx.guild_id().unwrap().as_u64() as i64,
        250,
        BalType::Wallet
    ).await;

    ctx.send(|b| b.embed(|e| e
        .description("\u{200b}\nYou have claimed your daily reward.\n\u{200b}")
        .field("Coins", "250 <:coin:1119247275093413940>", true)
        .field("Experience", "250", true)
    )).await?;
    Ok(())
}

pub fn commands() -> [Command; 5] {
    [balance(), give(), deposit(), withdraw(), daily()]
}

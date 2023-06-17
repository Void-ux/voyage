use crate::structs::{Command, CommandError, BalType};
use crate::Context;
use poise::serenity_prelude as serenity;

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
        .description(format!("**Total** - {} {}", wallet + bank, "<:coin:1119247275093413940>"))
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

pub fn commands() -> [Command; 4] {
    [balance(), give(), deposit(), withdraw()]
}

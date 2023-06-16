use crate::structs::{Command, CommandError};
use crate::Context;
use poise::serenity_prelude as serenity;
use sqlx::types::BigDecimal;

#[poise::command(prefix_command, slash_command)]
pub async fn balance(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<(), CommandError> {
    let member = member.unwrap_or(ctx.author_member().await.unwrap().to_mut().to_owned());
    let wallet = sqlx::query!(
        "SELECT SUM(coins) FROM economy WHERE user_id=$1 AND guild_id=$2 AND bal_type='wallet'",
        *member.user.id.as_u64() as i64,
        *member.guild_id.as_u64() as i64
    ).fetch_one(&ctx.data().pool).await?.sum.unwrap_or(BigDecimal::from(0));

    let bank = sqlx::query!(
        "SELECT SUM(coins) FROM economy WHERE user_id=$1 AND guild_id=$2 AND bal_type='bank'",
        *member.user.id.as_u64() as i64,
        *member.guild_id.as_u64() as i64
    ).fetch_one(&ctx.data().pool).await?.sum.unwrap_or(BigDecimal::from(0));

    ctx.send(|b| b.embed(|e| e
        .description(format!("**Total** - {} {}", &wallet + &bank, "[emoji]"))
        .field("Wallet", format!("{}", wallet), true)
        .field("Bank", format!("{}", bank), true)
        .author(|a| a
            .name(member.display_name())
            .icon_url(member.avatar_url().unwrap_or(member.user.avatar_url().unwrap_or(member.user.default_avatar_url())))
        )
    )).await?;

    Ok(())
}


pub fn commands() -> [Command; 1] {
    [balance()]
}

use crate::structs::{Command, CommandError, Context, BalType};
use crate::utils::TabularData;
use sqlx::{Row, Column};
use titlecase::titlecase;
use poise::serenity_prelude as serenity;

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

/// [dev] adds an item to the inventory list which will be obtainable through exploring
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn additem(
    ctx: Context<'_>,
    emoji: serenity::Emoji,
    price: i64,
    tier: String,
    #[rest] name: String
) -> Result<(), CommandError> {
    sqlx::query!(
        "INSERT INTO explore_items (name, sell_price, tier, emoji_name, emoji_id) VALUES ($1, $2, $3, $4, $5)",
        name,
        price,
        tier,
        emoji.name,
        *emoji.id.as_u64() as i64
    ).execute(&ctx.data().pool).await?;

    ctx.send(|b| b.embed(|e| e 
        .colour(0x2D936C)
        .title("Item Added")
        .field("Name", titlecase(&name), true)
        .field("Tier", titlecase(&tier), true)
        .field("Emoji", emoji, true)
        .field("Price", price, true)
    )).await?;
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

pub fn commands() -> [Command; 3] {
    [sql(), additem(), give()]
}

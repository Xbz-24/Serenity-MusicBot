use serenity::all::{Context, Message};
use serenity::all::standard::CommandResult;
use serenity::all::standard::macros::command;
use crate::utils::utility::check_msg;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);

    Ok(())
}

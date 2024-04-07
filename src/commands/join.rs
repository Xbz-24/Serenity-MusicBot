use std::time::Duration;
use serenity::all::{Context, Message};
use serenity::all::standard::CommandResult;
use serenity::all::standard::macros::command;
use serenity::prelude::Mentionable;
use songbird::{Event, TrackEvent};
use crate::handlers::channel_duration_notifier::ChannelDurationNotifier;
use crate::handlers::track_end_notifier::TrackEndNotifier;
use crate::utils::utility::check_msg;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = msg.guild(&ctx.cache).unwrap();
        let channel_id = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handle_lock) = manager.join(guild_id, connect_to).await {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

        let chan_id = msg.channel_id;

        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndNotifier {
                chan_id,
                http: send_http,
            },
        );

        let send_http = ctx.http.clone();

        handle.add_global_event(
            Event::Periodic(Duration::from_secs(60), None),
            ChannelDurationNotifier {
                chan_id,
                count: Default::default(),
                http: send_http,
            },
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }
    Ok(())
}

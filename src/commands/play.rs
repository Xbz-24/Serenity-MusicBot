use serenity::all::{Context, Message};
use serenity::all::standard::{Args, CommandResult};
use serenity::all::standard::macros::command;
use songbird::{Event, TrackEvent};
use songbird::input::YoutubeDl;
use crate::handlers::song_end_notifier::SongEndNotifier;
use crate::utils::http_client::get_http_client;
use crate::utils::utility::check_msg;

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();

    let http_client = get_http_client(ctx).await;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = YoutubeDl::new(http_client, url);

        let song = handler.play_input(src.into());
        let send_http = ctx.http.clone();
        let chan_id = msg.channel_id;

        // let _ = song.add_event(
        //     Event::Periodic(Duration::from_secs(5), Some(Duration::from_secs(7))),
        //     SongFader {
        //         chan_id,
        //         http: send_http,
        //     },
        // );

        let send_http = ctx.http.clone();

        let _ = song.add_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                chan_id,
                http: send_http,
            },
        );

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

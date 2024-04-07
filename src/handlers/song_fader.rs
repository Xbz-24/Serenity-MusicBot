use songbird::EventHandler as VoiceEventHandler;
use std::sync::Arc;
use poise::async_trait;
use serenity::all::{ChannelId, Http};
use songbird::{Event, EventContext};
use crate::utils::utility::check_msg;

struct SongFader {
    chan_id: ChannelId,
    http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for SongFader {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(&[(state, track)]) = ctx {
            let _ = track.set_volume(state.volume / 2.0);

            if state.volume < 1e-2 {
                let _ = track.stop();
                check_msg(self.chan_id.say(&self.http, "Stopping song...").await);
                Some(Event::Cancel)
            } else {
                check_msg(self.chan_id.say(&self.http, "Volume reduced.").await);
                None
            }
        } else {
            None
        }
    }
}

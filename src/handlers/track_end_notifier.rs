use songbird::EventHandler as VoiceEventHandler;
use std::sync::Arc;
use poise::async_trait;
use serenity::all::{ChannelId, Http};
use songbird::{Event, EventContext};
use crate::utils::utility::check_msg;

pub(crate) struct TrackEndNotifier {
    pub(crate) chan_id: ChannelId,
    pub(crate) http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            check_msg(
                self.chan_id
                    .say(&self.http, &format!("Tracks ended: {}.", track_list.len()))
                    .await,
            );
        }
        None
    }
}

use songbird::EventHandler as VoiceEventHandler;
use std::sync::Arc;
use poise::async_trait;
use serenity::all::{ChannelId, Http};
use songbird::{Event, EventContext};
use crate::utils::utility::check_msg;

pub(crate) struct SongEndNotifier {
    pub(crate) chan_id: ChannelId,
    pub(crate) http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        check_msg(
            self.chan_id
                .say(&self.http, "Song faded out completely!")
                .await,
        );
        None
    }
}

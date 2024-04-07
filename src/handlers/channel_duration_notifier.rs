use songbird::EventHandler as VoiceEventHandler;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use poise::async_trait;
use serenity::all::{ChannelId, Http};
use songbird::{Event, EventContext};
use crate::utils::check_msg;

pub(crate) struct ChannelDurationNotifier {
    pub(crate) chan_id: ChannelId,
    pub(crate) count: Arc<AtomicUsize>,
    pub(crate) http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for ChannelDurationNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let count_before = self.count.fetch_add(1, Ordering::Relaxed);
        check_msg(
            self.chan_id
                .say(
                    &self.http,
                    &format!(
                        "I've been in this channel for {} minutes!",
                        count_before + 1
                    ),
                )
                .await,
        );

        None
    }
}

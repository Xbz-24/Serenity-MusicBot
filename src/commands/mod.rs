use serenity::all::standard::macros::group;

pub(crate) mod deafen;
pub(crate) mod join;
pub(crate) mod leave;
pub(crate) mod unmute;
pub(crate) mod undeafen;
pub(crate) mod stop;
pub(crate) mod skip;
pub(crate) mod queue;
pub(crate) mod play;
pub(crate) mod ping;
pub(crate) mod mute;

use crate::commands::deafen::DEAFEN_COMMAND;
use crate::commands::join::JOIN_COMMAND;
use crate::commands::leave::LEAVE_COMMAND;
use crate::commands::mute::MUTE_COMMAND;
use crate::commands::play::PLAY_COMMAND;
use crate::commands::queue::QUEUE_COMMAND;
use crate::commands::skip::SKIP_COMMAND;
use crate::commands::stop::STOP_COMMAND;
use crate::commands::ping::PING_COMMAND;
use crate::commands::undeafen::UNDEAFEN_COMMAND;
use crate::commands::unmute::UNMUTE_COMMAND;
#[group]
#[commands(
deafen, join, leave, mute , play, queue, skip, stop, ping, undeafen, unmute
)]
pub(crate) struct General;

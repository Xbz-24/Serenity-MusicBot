use reqwest::Client as HttpClient;
use serenity::prelude::{TypeMapKey, Context};

pub(crate) struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub(crate) async fn get_http_client(ctx: &Context) -> HttpClient {
    let data = ctx.data.read().await;
    data.get::<HttpKey>()
        .cloned()
        .expect("Guaranteed to exist in the typemap.")
}

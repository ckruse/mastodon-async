#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use futures_util::TryStreamExt;
use log::{as_serde, info};
use mastodon_async::Result;

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
    use log::warn;

    femme::with_level(log::LevelFilter::Info);
    let mastodon = register::get_mastodon_data().await?;
    let stream = mastodon.stream_user().await?;
    info!("watching mastodon for events. This will run forever, press Ctrl+C to kill the program.");
    stream
        .try_for_each(|event| async move {
            match event {
                // fill in how you want to handle events here.
                _ => warn!(event = as_serde!(event); "unrecognized event received"),
            }
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example log_events --features toml\n"
    );
}

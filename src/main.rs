mod error;
mod handlers;
mod routing;
mod state;
mod telegram;
mod utility;

use std::net::SocketAddr;
use tokio::{signal, sync::oneshot};

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .long("port")
                .help("http server port")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("token")
                .short("t")
                .long("token")
                .help("telegram bot token")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("chat_id")
                .short("c")
                .long("chat")
                .help("telegram chat id")
                .takes_value(true),
        )
        .get_matches()
}

#[tokio::main]
async fn main() {
    let matches = get_matches();

    let port: u16 = match matches.value_of("port") {
        Some(p) => p.parse::<u16>().unwrap(),
        _ => 3030,
    };

    let token = matches
        .value_of("token")
        .expect("Telegram bot token missing!");

    telegram::status(token)
        .await
        .expect("Invalid Telegram bot token!");

    let chat_id = matches
        .value_of("chat_id")
        .expect("Telegram chat id missing!");

    let state = state::State {
        token: token.to_string(),
        chat_id: chat_id.to_string(),
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let (tx, rx) = oneshot::channel();
    let (addr, server) =
        warp::serve(routing::routing(state)).bind_with_graceful_shutdown(addr, async {
            rx.await.ok();
        });
    tokio::task::spawn(server);
    println!("Listening on http://{}", addr);

    signal::ctrl_c().await.unwrap();
    let _ = tx.send(());
}

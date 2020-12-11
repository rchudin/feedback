mod handlers;
mod routing;

use std::net::SocketAddr;

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
        .get_matches()
}

#[tokio::main]
async fn main() {
    let matches = get_matches();

    let port: u16 = match matches.value_of("port") {
        Some(p) => p.parse::<u16>().unwrap(),
        _ => 3030,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server = warp::serve(routing::routing()).run(addr);
    println!("Listening on http://{}", addr);
    server.await;
}

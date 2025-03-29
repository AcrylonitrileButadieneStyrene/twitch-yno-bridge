#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod command;
mod parser;

use command::Command;
use tokio::sync::{
    broadcast::{self, Sender},
    mpsc::UnboundedReceiver,
};

#[derive(Clone)]
struct AppState {
    channel: Sender<Command>,
}

#[tokio::main]
async fn main() {
    let config = twitch_irc::ClientConfig::default();
    let (message_stream, client) = twitch_irc::TwitchIRCClient::<
        twitch_irc::SecureTCPTransport,
        twitch_irc::login::StaticLoginCredentials,
    >::new(config);
    client.join("fluoret".to_owned()).unwrap();

    let (sender, _) = broadcast::channel::<Command>(8);
    tokio::spawn(handle_twitch_chat(message_stream, sender.clone()));

    let router = axum::Router::new()
        .route("/", axum::routing::get(get_root))
        .route("/redirect", axum::routing::get(get_redirect))
        .route("/count", axum::routing::get(get_count))
        .with_state(AppState { channel: sender });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6248")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[axum::debug_handler]
async fn get_root(
    axum::extract::State(state): axum::extract::State<AppState>,
    wsu: axum::extract::WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    wsu.on_upgrade(|ws| async { websocket(ws, state).await })
}

#[axum::debug_handler]
async fn get_count(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    state.channel.receiver_count().to_string()
}

#[axum::debug_handler]
async fn get_redirect() -> impl axum::response::IntoResponse {
    axum::response::Html::from(
        r#"
        <h1>Loading. Please wait...</h1>
        <script>
            setTimeout(() => document.location.href = "https://ynoproject.net/2kki", 5000);
        </script>
    "#,
    )
}

async fn websocket(mut ws: axum::extract::ws::WebSocket, state: AppState) {
    let mut recv = state.channel.subscribe();

    loop {
        tokio::select! {
            biased;
            req = ws.recv() => {
                if on_socket_event(req, &mut ws).await.is_break() {
                    return;
                }
            },
            msg = recv.recv() => {
                match msg {
                    Ok(command) => drop(ws.send(axum::extract::ws::Message::text(serde_json::to_string(&command).unwrap())).await),
                    Err(broadcast::error::RecvError::Lagged(count)) => println!("Lagged behind by {count} commands"),
                    Err(broadcast::error::RecvError::Closed) => panic!("Broadcast channel has closed"),
                }
            },
        };
    }
}

async fn on_socket_event(
    req: Option<Result<axum::extract::ws::Message, axum::Error>>,
    ws: &mut axum::extract::ws::WebSocket,
) -> std::ops::ControlFlow<()> {
    match req {
        None => return std::ops::ControlFlow::Break(()),
        Some(Ok(axum::extract::ws::Message::Binary(bytes))) => {
            if !bytes.is_empty() && bytes[0] == 0x7F {
                let mut vec = vec![0x7F];
                vec.extend_from_slice(
                    tokio::fs::read_to_string("./script.js")
                        .await
                        .unwrap()
                        .as_bytes(),
                );
                ws.send(axum::extract::ws::Message::Binary(vec.into()))
                    .await
                    .unwrap();
            }
        }
        _ => println!("{req:?}"),
    };

    std::ops::ControlFlow::Continue(())
}

async fn handle_twitch_chat(
    mut message_stream: UnboundedReceiver<twitch_irc::message::ServerMessage>,
    sender: Sender<Command>,
) {
    while let Some(message) = message_stream.recv().await {
        match message {
            twitch_irc::message::ServerMessage::Privmsg(privmsg)
                if privmsg.source.command == "PRIVMSG" =>
            {
                if let [user, message] = &privmsg.source.params[..] {
                    if let Ok((_, cmd)) = parser::parse(message) {
                        if let Command::Bang(_) = cmd {
                            match user.as_str() {
                                "#fluoret" | "#starlitehi" | "#goobertgum" => (),
                                _ => continue,
                            }
                        }

                        drop(sender.send(cmd));
                    }
                }
            }
            _ => (),
        }
    }
}

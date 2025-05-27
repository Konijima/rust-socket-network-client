use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey};
use rsa::pkcs1v15::Pkcs1v15Sign;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::StreamExt;
use futures::SinkExt;

#[derive(Deserialize)]
struct Challenge {
    challenge: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Outgoing<'a> {
    #[serde(rename = "auth")]
    Auth {
        client_id: &'a str,
        signature: String,
    },
    #[serde(rename = "event")]
    Event {
        data: &'a str,
    },
}

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8081";
    let (ws_stream, _) = connect_async(url).await.expect("Can't connect");
    println!("Connected to {url}");

    let (mut write, mut read) = ws_stream.split();

    // 1. Wait for the server's challenge
    let msg = read.next().await.expect("No challenge").expect("WebSocket error");
    let challenge_json = match msg {
        Message::Text(txt) => txt,
        _ => panic!("Unexpected message"),
    };
    let challenge: Challenge = serde_json::from_str(&challenge_json).expect("Bad challenge");
    let challenge_bytes = general_purpose::STANDARD.decode(&challenge.challenge).expect("b64");

    // 2. Load private key and sign the challenge
    let pem = fs::read_to_string("keys/device123.pem").expect("missing private key");
    let privkey = RsaPrivateKey::from_pkcs1_pem(&pem).expect("bad private key");
    let signature = privkey.sign(
        Pkcs1v15Sign::new_unprefixed(),
        &challenge_bytes
    ).expect("sign error");
    let signature_b64 = general_purpose::STANDARD.encode(signature);

    // 3. Send auth message
    let auth = Outgoing::Auth {
        client_id: "device123",
        signature: signature_b64,
    };
    let auth_msg = serde_json::to_string(&auth).unwrap();
    write.send(Message::Text(auth_msg.into())).await.expect("send failed");
    println!("Auth sent");

    // 4. Spawn a task to read incoming messages
    tokio::spawn(async move {
        let mut read = read;
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(txt) = msg {
                println!("Received: {txt}");
            }
        }
    });

    // 5. Main loop: read stdin, send as event messages
    println!("You can now type messages. Ctrl+C to exit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let event = Outgoing::Event { data: &line };
        let event_msg = serde_json::to_string(&event).unwrap();
        write.send(Message::Text(event_msg.into())).await.expect("send failed");
    }
}

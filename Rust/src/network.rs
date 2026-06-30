use std::time::Duration;

use async_stream::stream;
use iced::Subscription;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

use crate::{Message, PixelSender};

fn scorit_stream(host: String, port: u16) -> impl futures::Stream<Item = Message> + Send + 'static {
    stream! {
        loop {
            match TcpStream::connect((host.as_str(), port)).await {
                Ok(tcp) => {
                    yield Message::ScoritConnected;
                    let reader = BufReader::new(tcp);
                    let mut lines = reader.lines();
                    loop {
                        match lines.next_line().await {
                            Ok(Some(line)) if !line.trim().is_empty() => {
                                yield Message::ScoritData(line);
                            }
                            Ok(Some(_)) => continue,
                            _ => break,
                        }
                    }
                    yield Message::ScoritDisconnected;
                }
                Err(_) => {}
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

fn pixel_stream(host: String, port: u16) -> impl futures::Stream<Item = Message> + Send + 'static {
    stream! {
        loop {
            let listener = loop {
                match TcpListener::bind((host.as_str(), port)).await {
                    Ok(l) => break l,
                    Err(_) => tokio::time::sleep(Duration::from_secs(5)).await,
                }
            };

            loop {
                match listener.accept().await {
                    Ok((tcp, _addr)) => {
                        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
                        yield Message::PixelConnected(PixelSender(tx));

                        let (_, mut writer) = tcp.into_split();
                        while let Some(data) = rx.recv().await {
                            if writer.write_all(data.as_bytes()).await.is_err() {
                                break;
                            }
                        }
                        yield Message::PixelDisconnected;
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }
    }
}

/// Subscription that connects to the Scorit scoring system.
/// Changing `host` or `port` restarts the subscription automatically.
pub fn scorit_subscription(host: String, port: u16) -> Subscription<Message> {
    let id = format!("scorit:{}:{}", host, port);
    Subscription::run_with_id(id, scorit_stream(host, port))
}

/// Subscription that binds the PixelCom TCP server.
/// Changing `host` or `port` restarts the subscription automatically.
pub fn pixel_subscription(host: String, port: u16) -> Subscription<Message> {
    let id = format!("pixel:{}:{}", host, port);
    Subscription::run_with_id(id, pixel_stream(host, port))
}

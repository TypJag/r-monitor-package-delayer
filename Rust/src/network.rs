use std::any::TypeId;
use std::time::Duration;

use async_stream::stream;
use iced::Subscription;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

use crate::{Message, PixelSender};

const SCORIT_HOST: &str = "192.168.10.127";
const SCORIT_PORT: u16 = 50000;
const PIXEL_HOST: &str = "192.168.10.134";
const PIXEL_PORT: u16 = 50001;

struct ScoritWorker;
struct PixelWorker;

fn scorit_stream() -> impl futures::Stream<Item = Message> + Send + 'static {
    stream! {
        loop {
            match TcpStream::connect((SCORIT_HOST, SCORIT_PORT)).await {
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

fn pixel_stream() -> impl futures::Stream<Item = Message> + Send + 'static {
    stream! {
        loop {
            let listener = loop {
                match TcpListener::bind((PIXEL_HOST, PIXEL_PORT)).await {
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

pub fn scorit_subscription() -> Subscription<Message> {
    Subscription::run_with_id(TypeId::of::<ScoritWorker>(), scorit_stream())
}

pub fn pixel_subscription() -> Subscription<Message> {
    Subscription::run_with_id(TypeId::of::<PixelWorker>(), pixel_stream())
}

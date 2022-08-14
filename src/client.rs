use std::time::Duration;

use cursive::{
    reexports::log::debug,
    views::{Dialog, ListView, TextView},
};
use northstar_rcon_client::{self, connect};
use tokio::sync::mpsc::{error::TryRecvError, Receiver, Sender};

use crate::{model::Msg, ui};

// const EXIT_PHRASES: [&'static str; 3] = ["q", "exit", "quit"];
// const MAX_NETCON_INPUT_LEN: usize = 2048usize;

pub async fn start(sink: cursive::CbSink, tx: Sender<Msg>, mut rx: Receiver<Msg>) {
    let mut wr = None;
    let mut handle = None;
    loop {
        // debug!("Loop");
        match rx.try_recv() {
            Ok(m) => match m {
                Msg::Start { target, password } => {
                    debug!("Connecting to server...");
                    if let Ok(client) = connect(format!("{}:{}", target.url, target.port)).await {
                        debug!("Connected to server");
                        debug!("Trying to authenticate...");
                        match client.authenticate(&password).await {
                            Ok((mut reader, mut writer)) => {
                                debug!("We're in!");
                                sink.send(Box::new(|s| {
                                    s.pop_layer();
                                    s.add_layer(Dialog::info("Connected!"));
                                    s.pop_layer();
                                }))
                                .unwrap();
                                let t = tx.clone();
                                sink.send(Box::new(move |s| {
                                    s.add_active_screen();
                                    s.add_fullscreen_layer(ui::main_view::build(t));
                                }))
                                .unwrap();
                                writer.enable_console_logs().await.unwrap();
                                wr = Some(writer);
                                let s = sink.clone();
                                let h = tokio::spawn(async move {
                                    loop {
                                        if let Ok(msg) = reader.receive_console_log().await {
                                            s.send(Box::new(move |s| {
                                                s.call_on_name("output", |view: &mut ListView| {
                                                    view.add_child("[SERVER]", TextView::new(&msg));
                                                });
                                            }))
                                            .unwrap();
                                        } else {
                                            break;
                                        }
                                    }
                                });
                                handle = Some(h);
                            }
                            Err(_) => {}
                        }
                    } else {
                        sink.send(Box::new(|s| {
                            s.pop_layer();
                            s.add_layer(Dialog::info("Error connecting to remote server"))
                        }))
                        .unwrap();
                    }
                }
                Msg::Send { message } => {
                    if let Some(w) = wr.as_mut() {
                        w.exec_command(&message).await.unwrap();
                    }
                }
                Msg::Stop => {
                    if let Some(h) = handle {
                        h.abort();
                    }
                    break;
                }
            },
            Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {
                // debug!("Sleeping");
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    }
}

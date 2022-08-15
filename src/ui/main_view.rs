use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, ListView, Panel, ResizedView, TextView};
use tokio::sync::mpsc::Sender;

use crate::model::Msg;

pub fn build(tx: Sender<Msg>) -> ResizedView<Dialog> {
    let output = ListView::new()
        .with_name("output")
        .scrollable()
        .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
        .full_width()
        .fixed_height(20);
    let layout = LinearLayout::vertical()
        .child(Panel::new(output))
        .child(Panel::new(
            EditView::new()
                .on_submit(move |s, msg| {
                    s.call_on_name("input", |v: &mut EditView| {
                        v.set_content("");
                    });
                    s.call_on_name("output", |v: &mut ListView| {
                        v.add_child("[CLIENT]", TextView::new(msg));
                    });
                    if msg.starts_with("/") {
                        let mut parts = msg.split(' ');
                        match parts.next() {
                            Some("/leave") | Some("/quit") => {
                                tx.blocking_send(Msg::Stop).unwrap();
                            }
                            Some("/set") => {
                                if let Some(var) = parts.next() {
                                    let val = parts.collect::<Vec<&str>>().join(" ");
                                    tx.blocking_send(Msg::Set {
                                        var: var.to_string(),
                                        val: val.to_string(),
                                    })
                                    .unwrap();
                                }
                            }
                            Some(_) => {
                                s.add_layer(Dialog::info("Unsupported command"));
                            }
                            None => {
                                s.add_layer(Dialog::info("Unknown error handling command"));
                            }
                        }
                    } else {
                        tx.blocking_send(Msg::Send {
                            message: msg.to_string(),
                        })
                        .unwrap();
                    }
                })
                .with_name("input")
                .min_height(2)
                .full_width(),
        ));

    Dialog::around(layout).title("RetCon").full_screen()
}

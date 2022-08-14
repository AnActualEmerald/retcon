use cursive::{
    align::HAlign,
    reexports::log::debug,
    views::{Dialog, EditView, ListView, TextView},
    Cursive,
};

use cursive::traits::*;
use tokio::sync::mpsc::Sender;

use crate::model::{Msg, Server};

pub fn show(s: &mut Cursive, server: Server, channel: Sender<Msg>) {
    fn start(s: &mut Cursive, channel: Sender<Msg>, target: Server) {
        if let Some(password) = s.call_on_name("password", |v: &mut EditView| v.get_content()) {
            if *password == String::new() {
                s.add_layer(Dialog::info("Please enter a password"));
                return;
            }
            s.pop_layer();
            s.add_layer(Dialog::around(TextView::new("Connecting...")));
            channel
                .blocking_send(Msg::Start {
                    target,
                    password: password.to_string(),
                })
                .map_err(|e| {
                    debug!("Error sending message through channel: {}", e);
                })
                .unwrap();
        } else {
            s.add_layer(Dialog::info("Please enter a password").button("OK", |s| {
                s.call_on_name("password", |v: &mut EditView| v.set_content(String::new()));
                s.pop_layer();
            }))
        }
    }
    let serv = server.clone();
    let chan = channel.clone();
    let layout = ListView::new().child(
        "Password:",
        EditView::new()
            .on_submit(move |s, _| {
                start(s, chan.clone(), serv.clone());
            })
            .with_name("password")
            .fixed_width(20),
    );

    let chan = channel;
    let serv = server;
    s.add_layer(
        Dialog::around(layout)
            .title("Enter server password")
            .h_align(HAlign::Center)
            .button("Cancel", |s| {
                s.pop_layer();
            })
            .button("Submit", move |s| start(s, chan.clone(), serv.clone())),
    );
}

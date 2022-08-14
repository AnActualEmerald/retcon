use std::cell::RefCell;
use std::rc::Rc;

use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView};
use cursive::{traits::*, Cursive};
use tokio::sync::mpsc::Sender;

use crate::model::{Config, Msg};

use super::{add_popup, password_popup};

pub struct ServerView {
    inner: Dialog,
}

impl ServerView {
    pub fn new(cfg: Rc<RefCell<Config>>, channel: Sender<Msg>) -> Self {
        let c = cfg.clone();
        let add_server = move |s: &mut Cursive| {
            add_popup::show(s, c.clone());
        };
        let c = cfg.clone();
        let remove_server = move |s: &mut Cursive| {
            //okay to unwrap here becaus this should always exist
            let mut select = s.find_name::<SelectView<String>>("servers").unwrap();
            match select.selected_id() {
                None => s.add_layer(Dialog::info("Please select the server to remove")),
                Some(f) => {
                    if let Some((name, _)) = select.get_item(f) {
                        let mut c = c.borrow_mut();
                        if c.servers.contains_key(name) {
                            c.servers.remove(name);
                        }
                    }

                    select.remove_item(f);
                }
            }
        };

        let c = cfg.clone();
        let chan = channel;
        let select = SelectView::<String>::new()
            .autojump()
            .on_submit(move |s, name: &str| {
                let c = c.borrow();
                if c.servers.contains_key(name) {
                    let server = c.servers.get(name).unwrap();
                    password_popup::show(s, server.clone(), chan.clone());
                } else {
                    s.add_layer(Dialog::info("No server of that name was found!").button(
                        "OK",
                        |s| {
                            s.pop_layer();
                        },
                    ));
                }
            })
            .with_all_str(cfg.borrow_mut().servers.keys())
            .with_name("servers")
            .fixed_size((10i32, 20i32));

        let dialog = LinearLayout::vertical()
            .child(Button::new("Add new", add_server))
            .child(Button::new("Remove", remove_server))
            .child(DummyView)
            .child(Button::new("Exit", Cursive::quit));

        let layout = LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(dialog);

        let root = Dialog::around(layout).title("Select A Server");

        Self { inner: root }
    }

    pub fn inner(self) -> Dialog {
        self.inner
    }
}

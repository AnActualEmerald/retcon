use std::sync::{Arc, Mutex};

use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView};
use cursive::{traits::*, Cursive};

use crate::model::{Config, Server};

use super::add_popup;

pub struct ServerView {
    inner: Dialog,
}

impl ServerView {
    pub fn new(cfg: Arc<Mutex<Config>>) -> Self {
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
                        let mut c = c.lock().unwrap();
                        if c.servers.contains_key(name) {
                            c.servers.remove(name);
                        }
                    }

                    select.remove_item(f);
                }
            }
        };

        let select = SelectView::<String>::new()
            .with_all_str(cfg.lock().unwrap().servers.keys())
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

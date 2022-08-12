use std::sync::{Arc, Mutex};

use cursive::{
    traits::*,
    views::{Dialog, EditView, LinearLayout, SelectView, TextView},
    Cursive,
};

use crate::model::{Config, Server};

pub fn show(s: &mut Cursive, cfg: Arc<Mutex<Config>>) {
    let c = cfg.clone();
    let name = LinearLayout::horizontal()
        .child(TextView::new("Server name: "))
        .child(EditView::new().with_name("name").fixed_width(20));

    let url = LinearLayout::horizontal()
        .child(TextView::new("URL:PORT: "))
        .child(EditView::new().with_name("url").fixed_width(20));

    let root = LinearLayout::vertical().child(name).child(url);
    s.add_layer(
        Dialog::around(root)
            .button("Cancel", |s| {
                s.pop_layer();
            })
            .button("Ok", move |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| view.get_content())
                    .unwrap();
                let url = s
                    .call_on_name("url", |view: &mut EditView| view.get_content())
                    .unwrap();

                let (url, port) = if let Some((ip, port)) = url.split_once(':') {
                    (ip.to_string(), port.to_string())
                } else {
                    (url.to_string(), "37015".to_string())
                };

                c.lock()
                    .unwrap()
                    .servers
                    .insert(name.to_string(), Server { url, port });

                s.call_on_name("servers", |view: &mut SelectView<String>| {
                    view.add_item_str(name.as_str());
                });

                s.pop_layer();
            })
            .title("Enter server info"),
    );
}

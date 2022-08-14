use cursive::{
    traits::*,
    views::{Dialog, EditView, ListView, SelectView},
    Cursive,
};
use std::{cell::RefCell, rc::Rc};

use crate::model::{Config, Server};

pub fn show(s: &mut Cursive, cfg: Rc<RefCell<Config>>) {
    let c = cfg;
    let root = ListView::new()
        .child(
            "Server Name:",
            EditView::new().with_name("name").fixed_width(20),
        )
        .child("URL:", EditView::new().with_name("url").fixed_width(20))
        .child(
            "PORT:",
            EditView::new()
                .content("37015")
                .with_name("port")
                .fixed_width(10),
        );

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
                    .unwrap()
                    .to_string();

                let port = s
                    .call_on_name("port", |view: &mut EditView| view.get_content())
                    .unwrap()
                    .to_string();

                c.borrow_mut()
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

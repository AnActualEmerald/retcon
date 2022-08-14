use std::{cell::RefCell, fs, rc::Rc};

use clap::Parser;
use directories::ProjectDirs;
use model::{Config, Msg};
use tokio::sync::mpsc::channel;

mod client;
mod model;
mod ui;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {}

fn main() {
    let theme = include_str!("../theme.toml");
    cursive::logger::init();
    Cli::parse();
    let dirs = ProjectDirs::from("me", "greenboi", "retcon").unwrap();
    ensure_dirs(&dirs);
    let cfg = Rc::new(RefCell::new(
        Config::load(dirs.config_dir().join("config.ron")).unwrap(),
    ));

    let (tx, rx) = channel::<Msg>(1);

    let mut s = cursive::default();
    s.load_toml(theme).unwrap();

    let sink = s.cb_sink().clone();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let t = tx.clone();
    runtime.spawn(async move {
        client::start(sink, t, rx).await;
    });

    let initial = ui::server_select::ServerView::new(cfg.clone(), tx);
    s.add_layer(initial.inner());
    s.add_global_callback('+', cursive::Cursive::toggle_debug_console);

    s.run();

    cfg.borrow().save().unwrap();
}

fn ensure_dirs(dirs: &ProjectDirs) {
    fs::create_dir_all(dirs.config_dir()).expect("Unable to create configuration directory");
}

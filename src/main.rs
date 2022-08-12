use std::{
    fs,
    sync::{Arc, Mutex},
};

use clap::Parser;
use directories::ProjectDirs;
use model::Config;

mod model;
mod ui;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {}

#[tokio::main]
async fn main() {
    Cli::parse();
    let dirs = ProjectDirs::from("me", "greenboi", "retcon").unwrap();
    ensure_dirs(&dirs);
    let cfg = Arc::new(Mutex::new(
        Config::load(dirs.config_dir().join("config.ron")).unwrap(),
    ));

    let initial = ui::server_select::ServerView::new(cfg.clone());

    let mut s = cursive::default();

    s.add_layer(initial.inner());

    s.run();

    cfg.lock().unwrap().save().unwrap();
}

fn ensure_dirs(dirs: &ProjectDirs) {
    fs::create_dir_all(dirs.config_dir()).expect("Unable to create configuration directory");
}

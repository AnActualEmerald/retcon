// use std::io::{self, Write};

// use anyhow::Result;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::Dialog;
use cursive::Cursive;

// mod client;
// mod proto;

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();

    siv.add_fullscreen_layer(
        Dialog::text("This is a survey!\nPress <Next> when you're ready.")
            .title("Important survey")
            .button("Next", show_next)
            .resized(SizeConstraint::Full, SizeConstraint::Full),
    );

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Did you do the thing?")
            .title("Question 1")
            .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
            .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
            .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))),
    );
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit()),
    );
}

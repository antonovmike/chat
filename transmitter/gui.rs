use gtk::{
    prelude::*, ScrolledWindow, ApplicationWindow, Box, 
    Entry, Orientation, TextView, Adjustment,
};

use crate::db::read_from_chat_db;

pub fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("chatapp")
        .default_width(350)
        .default_height(200)
        .build();
    
    let vbox = Box::new(Orientation::Vertical, 10);
    window.add(&vbox);
    
    let scrolled_window = ScrolledWindow::new(
        Some(&Adjustment::new(0.0, 0.0, 100.0, 1.0, 10.0, 10.0)), 
        Some(&Adjustment::new(0.0, 0.0, 100.0, 1.0, 10.0, 10.0))
    );

    let text_view = TextView::new();
    scrolled_window.add(&text_view);
    vbox.pack_start(&scrolled_window, true, true, 0);

    let entry = Entry::new();
    vbox.pack_start(&entry, false, true, 0);

    let text_db = read_from_chat_db().unwrap();

    for item in text_db.clone() {
        let buffer = text_view.buffer().expect("Couldn't get text buffer");
        let mut end_iter = buffer.end_iter();
        let text = format!("{}:\t{}\n", item.0, item.1);
        buffer.insert(&mut end_iter, &text);
    }

    entry.connect_activate(move |entry| {
        let buffer = text_view.buffer().expect("Couldn't get text buffer");
        let mut end_iter = buffer.end_iter();
        let text = entry.text();
        buffer.insert(&mut end_iter, &format!("{}\n", text));
        entry.set_text("");
    });

    window.show_all();
}

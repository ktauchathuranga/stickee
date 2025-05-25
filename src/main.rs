use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ScrolledWindow, TextView};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct NoteData {
    note: String,
}

fn main() {
    let app = Application::builder()
        .application_id("io.github.ktauchathuranga.stickee")
        .build();

    app.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Stickee")
            .default_width(300)
            .default_height(200)
            .build();

        // Create a text view
        let text_view = TextView::new();
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        let buffer = text_view.buffer();

        // Create a scrolled window
        let scrolled = ScrolledWindow::builder()
            .child(&text_view)
            .build();

        window.set_child(Some(&scrolled));

        // Load saved notes
        let note_file = PathBuf::from(std::env::var("HOME").unwrap()).join(".stickee.json");
        if note_file.exists() {
            if let Ok(data) = fs::read_to_string(&note_file) {
                if let Ok(note_data) = serde_json::from_str::<NoteData>(&data) {
                    buffer.set_text(&note_data.note);
                }
            }
        }

        // Save notes when text changes
        buffer.connect_changed(move |buffer| {
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let text = buffer.text(&start, &end, false);
            let note_data = NoteData { note: text.to_string() };
            if let Ok(file) = File::create(&note_file) {
                let _ = serde_json::to_writer(file, &note_data);
            }
        });

        window.present();
    });

    app.run();
}

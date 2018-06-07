extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, DrawingArea};

fn main() {
    // Initialize gtk
    gtk::init().unwrap();

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello world");
    window.set_default_size(400, 400);
    
    // Create the drawing area
    let area = DrawingArea::new();
    area.connect_draw(move |_, cr| {
        // white background
        cr.set_source_rgb(0., 0., 0.);
        cr.paint();

        // green rectangle
        cr.set_source_rgb(0., 1., 0.);
        cr.rectangle(100., 100., 200., 200.);
        cr.fill();

        Inhibit(false)
    });
    window.add(&area);

    // Make all widgets visivle
    window.show_all();

    // Handle closing of the window
    window.connect_delete_event(|_, _| {
        gtk::main_quit(); // Stop the main loop
        Inhibit(false)   // Let the default handler destroy the window
    });

    // Run the main loop
    gtk::main();
}

extern crate gio;
extern crate gtk;

use gtk::prelude::*;

fn main() {
	let argc: i32 = 0;
	let argv: &[&str] = &[];

	let app = gtk::Application::new(None, gio::APPLICATION_FLAGS_NONE).unwrap();
	app.connect_activate(|_| {});
	app.run(argc, argv);
}

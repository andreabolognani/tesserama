extern crate gio;
extern crate gtk;

use gtk::prelude::*;

struct Tesserama {
	app: gtk::Application,
}

impl Tesserama {
	fn new() -> Self {
		let flags = gio::ApplicationFlags::empty();
		let app = gtk::Application::new(None, flags).unwrap();
		app.connect_activate(|_| {});
		Tesserama { app: app }
	}

	fn run(&self) -> i32 {
		let argc: i32 = 0;
		let argv: &[&str] = &[];
		self.app.run(argc, argv)
	}
}

fn main() {
	Tesserama::new().run();
}

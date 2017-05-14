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
		let args: Vec<String> = std::env::args().collect();
		let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

		let argc: i32 = args.len() as i32;
		let argv: &[&str] = &args;

		self.app.run(argc, argv)
	}
}

fn main() {
	Tesserama::new().run();
}

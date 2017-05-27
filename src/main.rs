extern crate gio;
extern crate gtk;

use gtk::prelude::*;

struct Tesserama {
	app: gtk::Application,
}

impl Tesserama {
	fn new() -> Result<Self, ()> {
		let flags = gio::ApplicationFlags::empty();
		match gtk::Application::new(None, flags) {
			Ok(app) => {
				app.connect_activate(|_| {});
				Ok(Tesserama { app: app })
			}
			Err(_) => Err(())
		}
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
	match Tesserama::new() {
		Ok(value) => {
			value.run();
		}
		Err(_) => {
			println!("GTK+ initialization error");
		}
	}
}

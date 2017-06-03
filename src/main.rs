extern crate gio;
extern crate gtk;

use gtk::prelude::*;

#[derive(Clone)]
struct Application {
	parent: gtk::Application,
}

impl Application {
	fn new() -> Self {
		let flags = gio::ApplicationFlags::empty();
		let ret = Application {
			parent: gtk::Application::new(None, flags)
			        .expect("GTK+ initialization error"),
		};
		ret.setup();
		ret
	}

	fn setup(&self) {
		let _self = self.clone();
		self.parent.connect_activate(move |_| {
			_self.activate();
		});
	}

	fn run(&self) {
		let args: Vec<String> = std::env::args().collect();
		let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

		let argc: i32 = args.len() as i32;
		let argv: &[&str] = &args;

		self.parent.run(argc, argv);
	}

	fn create_window(&self) -> gtk::ApplicationWindow {
		gtk::ApplicationWindow::new(&self.parent)
	}

	fn activate(&self) {
		Window::new(&self).show_all();
	}
}

#[derive(Clone)]
struct Window {
	parent: gtk::ApplicationWindow,
	headerbar: gtk::HeaderBar,
}

impl Window {
	fn new(app: &Application) -> Self {
		let ret = Window {
			parent: app.create_window(),
			headerbar: gtk::HeaderBar::new(),
		};
		ret.setup();
		ret
	}

	fn setup(&self) {
		self.parent.set_title("Tesserama");
		self.parent.set_default_size(800, 600);

		self.headerbar.set_show_close_button(true);
		self.parent.set_titlebar(&self.headerbar);
	}

	fn show_all(&self) {
		self.parent.show_all();
	}
}

fn main() {
	Application::new().run();
}

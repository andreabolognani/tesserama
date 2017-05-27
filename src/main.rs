extern crate gio;
extern crate gtk;

use gtk::prelude::*;

fn app_new() -> gtk::Application {
	let flags = gio::ApplicationFlags::empty();
	match gtk::Application::new(None, flags) {
		Ok(app) => {
			let app_clone = app.clone();
			app.connect_activate(move |_| {
				app_activate(&app_clone);
			});
			app
		}
		Err(_) => {
			panic!("GTK+ initialization error");
		}
	}
}

fn app_activate(app: &gtk::Application) {
		let win = win_new(app);
		win_show(&win);
}

fn app_run(app: &gtk::Application) {
	let args: Vec<String> = std::env::args().collect();
	let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

	let argc: i32 = args.len() as i32;
	let argv: &[&str] = &args;

	app.run(argc, argv);
}

fn win_new(app: &gtk::Application) -> gtk::ApplicationWindow {
	let win = gtk::ApplicationWindow::new(app);
	win.set_title("Tesserama");
	win
}

fn win_show(win: &gtk::ApplicationWindow) {
	win.show_all();
}

fn main() {
	let app = app_new();
	app_run(&app);
}

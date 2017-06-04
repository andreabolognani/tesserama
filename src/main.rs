extern crate glib;
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
		self.parent.set_accels_for_action("win.search", &["<Ctrl>f"]);
		self.parent.set_accels_for_action("win.insert", &["<Ctrl>i"]);
		self.parent.set_accels_for_action("win.open", &["<Ctrl>o"]);
		self.parent.set_accels_for_action("win.save", &["<Ctrl>s"]);

		let _self = self.clone();
		self.parent.connect_activate(move |_| {
			_self.activate_action();
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

	fn activate_action(&self) {
		Window::new(&self).show_all();
	}
}

#[derive(Clone)]
struct Window {
	parent: gtk::ApplicationWindow,
	headerbar: gtk::HeaderBar,
	searchbutton: gtk::ToggleButton,
	insertbutton: gtk::Button,
	menubutton: gtk::ToggleButton,
	searchaction: gio::SimpleAction,
	insertaction: gio::SimpleAction,
	menuaction: gio::SimpleAction,
	openaction: gio::SimpleAction,
	saveaction: gio::SimpleAction,
}

impl Window {
	fn new(app: &Application) -> Self {
		let ret = Window {
			parent: app.create_window(),
			headerbar: gtk::HeaderBar::new(),
			searchbutton: gtk::ToggleButton::new(),
			insertbutton: gtk::Button::new_with_label("Insert"),
			menubutton: gtk::ToggleButton::new(),
			searchaction: gio::SimpleAction::new_stateful(
				"search",
				None,
				&false.to_variant(),
			),
			insertaction: gio::SimpleAction::new("insert", None),
			menuaction: gio::SimpleAction::new_stateful(
				"menu",
				None,
				&false.to_variant(),
			),
			openaction: gio::SimpleAction::new("open", None),
			saveaction: gio::SimpleAction::new("save", None),
		};
		ret.setup();
		ret
	}

	fn setup(&self) {
		self.parent.set_title("Tesserama");
		self.parent.set_default_size(800, 600);

		/* Actions */

		let _self = self.clone();
		self.searchaction.connect_activate(move |_,_| {
			_self.search_action_activated();
		});
		self.searchaction.set_enabled(false);
		self.parent.add_action(&self.searchaction);

		let _self = self.clone();
		self.insertaction.connect_activate(move |_,_| {
			_self.insert_action_activated();
		});
		self.insertaction.set_enabled(false);
		self.parent.add_action(&self.insertaction);

		let _self = self.clone();
		self.menuaction.connect_activate(move |_,_| {
			_self.menu_action_activated();
		});
		self.parent.add_action(&self.menuaction);

		let _self = self.clone();
		self.openaction.connect_activate(move |_,_| {
			_self.open_action_activated();
		});
		self.parent.add_action(&self.openaction);

		let _self = self.clone();
		self.saveaction.connect_activate(move |_,_| {
			_self.save_action_activated();
		});
		self.saveaction.set_enabled(false);
		self.parent.add_action(&self.saveaction);

		/* Header bar */

		self.headerbar.set_show_close_button(true);
		self.parent.set_titlebar(&self.headerbar);

		let image = gtk::Image::new_from_icon_name(
			"edit-find-symbolic",
			gtk::IconSize::Button.into(),
		);
		self.searchbutton.set_image(&image);
		self.searchbutton.set_tooltip_text("Search");
		self.searchbutton.set_action_name("win.search");
		self.headerbar.pack_start(&self.searchbutton);

		self.insertbutton.set_action_name("win.insert");
		self.headerbar.pack_start(&self.insertbutton);

		let image = gtk::Image::new_from_icon_name(
			"open-menu-symbolic",
			gtk::IconSize::Button.into(),
		);
		self.menubutton.set_image(&image);
		self.menubutton.set_tooltip_text("Menu");
		self.menubutton.set_action_name("win.menu");
		self.headerbar.pack_end(&self.menubutton);
	}

	fn show_all(&self) {
		self.parent.show_all();
	}

	fn search_action_activated(&self) {
	}

	fn insert_action_activated(&self) {
	}

	fn menu_action_activated(&self) {
	}

	fn open_action_activated(&self) {
	}

	fn save_action_activated(&self) {
	}
}

fn main() {
	Application::new().run();
}

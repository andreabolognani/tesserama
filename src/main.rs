extern crate glib;
extern crate gio;
extern crate gtk;
extern crate csv;

use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

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

const RECORD_TYPES: [gtk::Type; 5] = [
	gtk::Type::String,
	gtk::Type::String,
	gtk::Type::String,
	gtk::Type::String,
	gtk::Type::String,
];

#[derive(Clone)]
struct Window {
	parent: gtk::ApplicationWindow,
	headerbar: gtk::HeaderBar,
	searchbutton: gtk::ToggleButton,
	insertbutton: gtk::Button,
	menubutton: gtk::ToggleButton,
	stack: gtk::Stack,
	treeview: gtk::TreeView,
	searchaction: gio::SimpleAction,
	insertaction: gio::SimpleAction,
	menuaction: gio::SimpleAction,
	openaction: gio::SimpleAction,
	saveaction: gio::SimpleAction,
	source_filename: Rc<RefCell<PathBuf>>,
	source_uri: Rc<RefCell<String>>,
	data: Rc<RefCell<gtk::ListStore>>,
}

impl Window {
	fn new(app: &Application) -> Self {
		let ret = Window {
			parent: app.create_window(),
			headerbar: gtk::HeaderBar::new(),
			searchbutton: gtk::ToggleButton::new(),
			insertbutton: gtk::Button::new_with_label("Insert"),
			menubutton: gtk::ToggleButton::new(),
			stack: gtk::Stack::new(),
			treeview: gtk::TreeView::new(),
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
			source_filename: Rc::new(RefCell::new(PathBuf::new())),
			source_uri: Rc::new(RefCell::new(String::new())),
			data: Rc::new(RefCell::new(gtk::ListStore::new(&RECORD_TYPES))),
		};
		ret.setup();
		ret
	}

	fn setup(&self) {
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

		/* Empty application */

		let empty = gtk::Label::new("");

		/* Application contents */

		let contents = gtk::Box::new(gtk::Orientation::Vertical, 0);

		let renderer = gtk::CellRendererText::new();
		let column = gtk::TreeViewColumn::new();
		column.pack_start(&renderer, false);
		column.add_attribute(&renderer, "text", 1);
		self.treeview.append_column(&column);

		let renderer = gtk::CellRendererText::new();
		let column = gtk::TreeViewColumn::new();
		column.set_title("People");
		column.set_expand(true);
		column.pack_start(&renderer, false);
		column.add_attribute(&renderer, "text", 2);
		self.treeview.append_column(&column);

		let renderer = gtk::CellRendererText::new();
		let column = gtk::TreeViewColumn::new();
		column.set_title("Signature");
		column.pack_start(&renderer, false);
		column.add_attribute(&renderer, "text", 3);
		self.treeview.append_column(&column);

		let renderer = gtk::CellRendererText::new();
		let column = gtk::TreeViewColumn::new();
		column.pack_start(&renderer, false);
		column.add_attribute(&renderer, "text", 4);
		self.treeview.append_column(&column);

		let renderer = gtk::CellRendererText::new();
		let column = gtk::TreeViewColumn::new();
		column.set_title("Date");
		column.pack_start(&renderer, false);
		column.add_attribute(&renderer, "text", 0);
		self.treeview.append_column(&column);

		let scrolled = gtk::ScrolledWindow::new(None, None);
		scrolled.add(&self.treeview);

		contents.pack_start(&scrolled, true, true, 0);

		self.stack.add_named(&empty, "empty");
		self.stack.add_named(&contents, "contents");
		self.parent.add(&self.stack);
	}

	fn show_all(&self) {
		self.parent.show_all();
	}

	fn update_title(&self) {
		let source_filename: &PathBuf = &*self.source_filename.borrow();

		let title: &OsStr = source_filename.file_name().unwrap();
		let title: Option<&str> = title.to_str();

		let subtitle: &Path = source_filename.parent().unwrap();
		let subtitle: Option<&str> = subtitle.to_str();

		self.headerbar.set_title(title);
		self.headerbar.set_subtitle(subtitle);
	}

	fn set_data_source(&self, filename: PathBuf, uri: String) {
		{
			let mut source_filename = self.source_filename.borrow_mut();
			*source_filename = filename;
		}
		{
			let mut source_uri = self.source_uri.borrow_mut();
			*source_uri = uri;
		}

		self.update_title();
	}

	fn load_data(&self) {
		{
			let mut data = self.data.borrow_mut();
			*data = gtk::ListStore::new(&RECORD_TYPES);
		}

		let data: &gtk::ListStore = &*self.data.borrow();
		let path: &PathBuf = &*self.source_filename.borrow();

		let mut reader = csv::ReaderBuilder::new()
		                 .has_headers(false)
		                 .from_path(path)
		                 .expect("Failed to open file");

		for record in reader.records() {
			if record.is_ok() {
				let record: csv::StringRecord = record.unwrap();

				// Convert the record to a format gtk::ListStore likes
				let record: [&glib::ToValue; 5] = [
					&String::from(&record[0]),
					&String::from(&record[1]),
					&String::from(&record[2]),
					&String::from(&record[3]),
					&String::from(&record[4]),
				];

				let iter = data.append();
				data.set(&iter, &[0, 1, 2, 3, 4], &record);
			}
		}

		self.treeview.set_model(data);

		self.stack.set_visible_child_name("contents");
	}

	fn search_action_activated(&self) {
	}

	fn insert_action_activated(&self) {
	}

	fn menu_action_activated(&self) {
	}

	fn open_action_activated(&self) {
		let dialog = gtk::FileChooserDialog::new(
			Some("Choose a file"),
			Some(&self.parent),
			gtk::FileChooserAction::Open,
		);
		dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
		dialog.add_button("Open", gtk::ResponseType::Ok.into());

		if dialog.run() == gtk::ResponseType::Ok.into() {
			let filename = dialog.get_filename();
			let uri = dialog.get_uri();

			if filename.is_some() && uri.is_some() {
				self.set_data_source(filename.unwrap(), uri.unwrap());
				self.load_data();
			}
		}

		dialog.destroy();
	}

	fn save_action_activated(&self) {
	}
}

fn main() {
	Application::new().run();
}

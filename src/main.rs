// Tesserama - Simple membership cards manager
// Copyright (C) 2017  Andrea Bolognani <eof@kiyuko.org>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

extern crate glib;
extern crate gio;
extern crate pango;
extern crate gtk;
extern crate csv;

use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use gio::prelude::*;
use gio::MenuExt;
use gtk::prelude::*;

#[derive(Clone)]
struct Application {
    parent: gtk::Application,
}

impl Application {
    fn new() -> Self {
        let flags = gio::ApplicationFlags::empty();
        let ret = Self {
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
        self.parent.run(&args);
    }

    fn create_window(&self) -> gtk::ApplicationWindow {
        gtk::ApplicationWindow::new(&self.parent)
    }

    fn activate_action(&self) {
        ApplicationWindow::new(&self).show_all();
    }
}

#[derive(Clone)]
struct ApplicationWindow {
    parent: gtk::ApplicationWindow,
    headerbar: gtk::HeaderBar,
    searchbutton: gtk::ToggleButton,
    insertbutton: gtk::Button,
    menubutton: gtk::ToggleButton,
    menupopover: gtk::Popover,
    stack: gtk::Stack,
    searchentry: gtk::SearchEntry,
    searchbar: gtk::SearchBar,
    treeview: gtk::TreeView,
    searchaction: gio::SimpleAction,
    insertaction: gio::SimpleAction,
    menuaction: gio::SimpleAction,
    openaction: gio::SimpleAction,
    saveaction: gio::SimpleAction,
    source_filename: Rc<RefCell<PathBuf>>,
    source_uri: Rc<RefCell<String>>,
    dirty: Rc<RefCell<bool>>,
    data: Rc<RefCell<gtk::ListStore>>,
    filtered_data: Rc<RefCell<gtk::TreeModelFilter>>,
    filter_needle: Rc<RefCell<String>>,
}

impl ApplicationWindow {
    const COLUMN_DATE: u32 = 0;
    const COLUMN_NUMBER: u32 = 1;
    const COLUMN_PEOPLE: u32 = 2;
    const COLUMN_SIGNATURE: u32 = 3;
    const COLUMN_FLAGS: u32 = 4;

    const RECORD_TYPES: [gtk::Type; 5] = [
        gtk::Type::String, // COLUMN_DATE
        gtk::Type::String, // COLUMN_NUMBER
        gtk::Type::String, // COLUMN_PEOPLE
        gtk::Type::String, // COLUMN_SIGNATURE
        gtk::Type::String, // COLUMN_FLAGS
    ];

    fn new(app: &Application) -> Self {
        let menubutton = gtk::ToggleButton::new();
        let menupopover = gtk::Popover::new(&menubutton);
        let data = gtk::ListStore::new(&Self::RECORD_TYPES);
        let filtered_data = gtk::TreeModelFilter::new(&data, None);
        let ret = Self {
            parent: app.create_window(),
            headerbar: gtk::HeaderBar::new(),
            searchbutton: gtk::ToggleButton::new(),
            insertbutton: gtk::Button::new_with_label("Insert"),
            menubutton: menubutton,
            menupopover: menupopover,
            stack: gtk::Stack::new(),
            searchentry: gtk::SearchEntry::new(),
            searchbar: gtk::SearchBar::new(),
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
            dirty: Rc::new(RefCell::new(false)),
            data: Rc::new(RefCell::new(data)),
            filtered_data: Rc::new(RefCell::new(filtered_data)),
            filter_needle: Rc::new(RefCell::new(String::new())),
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

        let menu = gio::Menu::new();
        menu.append("Open", "win.open");
        menu.append("Save", "win.save");
        self.menupopover.bind_model(&menu, None);

        let _self = self.clone();
        self.menupopover.connect_closed(move |_| {
            _self.menu_popover_closed();
        });

        /* Empty application */

        let empty = gtk::Label::new("");

        /* Application contents */

        let contents = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let _self = self.clone();
        self.searchentry.connect_search_changed(move |_| {
            _self.search_changed();
        });

        let _self = self.clone();
        self.searchentry.connect_stop_search(move |_| {
            _self.stop_search();
        });

        self.searchbar.connect_entry(&self.searchentry);
        self.searchbar.add(&self.searchentry);

        self.treeview.set_enable_search(false);

        let number_renderer = gtk::CellRendererText::new();
        number_renderer.set_alignment(1.0, 0.5);
        number_renderer.set_property_editable(true);
        let _self = self.clone();
        number_renderer.connect_edited(move |_, path, text| {
            _self.number_cell_edited(path, text);
        });
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&number_renderer, false);
        column.add_attribute(&number_renderer, "text", Self::COLUMN_NUMBER as i32);
        self.treeview.append_column(&column);

        let people_renderer = gtk::CellRendererText::new();
        people_renderer.set_property_ellipsize(pango::EllipsizeMode::End);
        people_renderer.set_property_editable(true);
        let _self = self.clone();
        people_renderer.connect_edited(move |_, path, text| {
            _self.people_cell_edited(path, text);
        });
        let column = gtk::TreeViewColumn::new();
        column.set_title("People");
        column.set_expand(true);
        column.pack_start(&people_renderer, false);
        column.add_attribute(&people_renderer, "text", Self::COLUMN_PEOPLE as i32);
        self.treeview.append_column(&column);

        let signature_renderer = gtk::CellRendererText::new();
        signature_renderer.set_property_editable(true);
        let _self = self.clone();
        signature_renderer.connect_edited(move |_, path, text| {
            _self.signature_cell_edited(path, text);
        });
        let column = gtk::TreeViewColumn::new();
        column.set_title("Signature");
        column.set_sizing(gtk::TreeViewColumnSizing::GrowOnly);
        column.pack_start(&signature_renderer, false);
        column.add_attribute(&signature_renderer, "text", Self::COLUMN_SIGNATURE as i32);
        self.treeview.append_column(&column);

        let flags_renderer = gtk::CellRendererText::new();
        flags_renderer.set_alignment(1.0, 0.5);
        flags_renderer.set_property_editable(true);
        let _self = self.clone();
        flags_renderer.connect_edited(move |_, path, text| {
            _self.flags_cell_edited(path, text);
        });
        let column = gtk::TreeViewColumn::new();
        column.set_sizing(gtk::TreeViewColumnSizing::GrowOnly);
        column.pack_start(&flags_renderer, false);
        column.add_attribute(&flags_renderer, "text", Self::COLUMN_FLAGS as i32);
        self.treeview.append_column(&column);

        let date_renderer = gtk::CellRendererText::new();
        date_renderer.set_property_editable(true);
        let _self = self.clone();
        date_renderer.connect_edited(move |_, path, text| {
            _self.date_cell_edited(path, text);
        });
        let column = gtk::TreeViewColumn::new();
        column.set_title("Date");
        column.pack_start(&date_renderer, false);
        column.add_attribute(&date_renderer, "text", Self::COLUMN_DATE as i32);
        self.treeview.append_column(&column);

        let scrolled = gtk::ScrolledWindow::new(None, None);
        scrolled.add(&self.treeview);

        contents.pack_start(&self.searchbar, false, false, 0);
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

        let file_name: &OsStr = source_filename.file_name().unwrap();
        let parent: &Path = source_filename.parent().unwrap();

        let mut tmp = String::from("");
        let title: Option<&str> = file_name.to_str().and_then(|s| {
            if self.is_dirty() {
                tmp.push('*');
            }
            tmp.push_str(s);
            Some(tmp.as_str())
        });
        let subtitle: Option<&str> = parent.to_str();

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

    fn set_dirty(&self, dirty: bool) {
        *self.dirty.borrow_mut() = dirty;
        self.saveaction.set_enabled(dirty);

        self.update_title()
    }

    fn is_dirty(&self) -> bool {
        *self.dirty.borrow()
    }

    fn search(&self) {
        {
            let mut filter_needle = self.filter_needle.borrow_mut();
            *filter_needle = self.searchentry.get_text().unwrap().to_lowercase();
        }

        let filtered_data: &gtk::TreeModelFilter = &*self.filtered_data.borrow();
        filtered_data.refilter();
    }

    fn load_data(&self) {
        {
            let mut data = self.data.borrow_mut();
            let mut filtered_data = self.filtered_data.borrow_mut();
            let mut filter_needle = self.filter_needle.borrow_mut();

            *data = gtk::ListStore::new(&Self::RECORD_TYPES);
            *filtered_data = gtk::TreeModelFilter::new(&*data, None);
            *filter_needle = String::new();
        }

        let data: &gtk::ListStore = &*self.data.borrow();
        let filtered_data: &gtk::TreeModelFilter = &*self.filtered_data.borrow();
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

        self.set_dirty(false);
        self.searchaction.set_enabled(true);
        self.insertaction.set_enabled(true);

        let _self = self.clone();
        filtered_data.set_visible_func(move |_, iter| {
            _self.filter_func(iter)
        });
        self.treeview.set_model(filtered_data);

        self.stack.set_visible_child_name("contents");
    }

    fn filter_func(&self, iter: &gtk::TreeIter) -> bool {
        let data: &gtk::ListStore = &*self.data.borrow();
        let filter_needle: &String = &*self.filter_needle.borrow();

        if filter_needle.parse::<i32>().is_ok() {
            // If the needle can be converted to a number, we look up
            // the corresponding record
            let value: glib::Value = data.get_value(iter, Self::COLUMN_NUMBER as i32);
            let number: &String = &value.get::<String>().unwrap();

            number == filter_needle
        } else {
            // In all other cases, we perform a case-insensitive substring
            // search among people's names
            let value: glib::Value = data.get_value(iter, Self::COLUMN_PEOPLE as i32);
            let people: String = value.get::<String>().unwrap().to_lowercase();

            people.contains(filter_needle)
        }
    }

    fn convert_path(&self, path: gtk::TreePath) -> gtk::TreePath {
        let filtered_data: &gtk::TreeModelFilter = &*self.filtered_data.borrow();

        // Since we use filtering on the data displayed in the
        // treeview, we have to convert paths from the filtered
        // model to the actual model before using them
        filtered_data.convert_path_to_child_path(&path).unwrap()
    }

    fn update_number(&self, path: gtk::TreePath, text: &str) {
        let data: &gtk::ListStore = &*self.data.borrow();
        let path: gtk::TreePath = self.convert_path(path);
        let iter: gtk::TreeIter = data.get_iter(&path).unwrap();
        let value: glib::Value = data.get_value(&iter, Self::COLUMN_NUMBER as i32);
        let number: &String = &value.get::<String>().unwrap();

        if text != number {
            let record: [&glib::ToValue; 1] = [
                &String::from(text),
            ];

            data.set(&iter, &[Self::COLUMN_NUMBER], &record);
            self.set_dirty(true);
        }
    }

    fn update_people(&self, path: gtk::TreePath, text: &str) {
        let data: &gtk::ListStore = &*self.data.borrow();
        let path: gtk::TreePath = self.convert_path(path);
        let iter: gtk::TreeIter = data.get_iter(&path).unwrap();
        let value: glib::Value = data.get_value(&iter, Self::COLUMN_PEOPLE as i32);
        let people: &String = &value.get::<String>().unwrap();

        if text != people {
            let record: [&glib::ToValue; 1] = [
                &String::from(text),
            ];

            data.set(&iter, &[Self::COLUMN_PEOPLE], &record);
            self.set_dirty(true);
        }
    }

    fn update_signature(&self, path: gtk::TreePath, text: &str) {
        let data: &gtk::ListStore = &*self.data.borrow();
        let path: gtk::TreePath = self.convert_path(path);
        let iter: gtk::TreeIter = data.get_iter(&path).unwrap();
        let value: glib::Value = data.get_value(&iter, Self::COLUMN_SIGNATURE as i32);
        let signature: &String = &value.get::<String>().unwrap();

        if text != signature {
            let record: [&glib::ToValue; 1] = [
                &String::from(text),
            ];

            data.set(&iter, &[Self::COLUMN_SIGNATURE], &record);
            self.set_dirty(true);
        }
    }

    fn update_flags(&self, path: gtk::TreePath, text: &str) {
        let data: &gtk::ListStore = &*self.data.borrow();
        let path: gtk::TreePath = self.convert_path(path);
        let iter: gtk::TreeIter = data.get_iter(&path).unwrap();
        let value: glib::Value = data.get_value(&iter, Self::COLUMN_FLAGS as i32);
        let flags: &String = &value.get::<String>().unwrap();

        if text != flags {
            let record: [&glib::ToValue; 1] = [
                &String::from(text),
            ];

            data.set(&iter, &[Self::COLUMN_FLAGS], &record);
            self.set_dirty(true);
        }
    }

    fn update_date(&self, path: gtk::TreePath, text: &str) {
        let data: &gtk::ListStore = &*self.data.borrow();
        let path: gtk::TreePath = self.convert_path(path);
        let iter: gtk::TreeIter = data.get_iter(&path).unwrap();
        let value: glib::Value = data.get_value(&iter, Self::COLUMN_DATE as i32);
        let date: &String = &value.get::<String>().unwrap();

        if text != date {
            let record: [&glib::ToValue; 1] = [
                &String::from(text),
            ];

            data.set(&iter, &[Self::COLUMN_DATE], &record);
            self.set_dirty(true);
        }
    }

    // High-level actions

    fn start_search_action(&self) {
        self.searchbar.set_search_mode(true);
        self.insertaction.set_enabled(false);
    }

    fn stop_search_action(&self) {
        self.searchbar.set_search_mode(false);
        self.insertaction.set_enabled(true);
    }

    fn start_menu_action(&self) {
        self.menupopover.show();
    }

    fn stop_menu_action(&self) {
        self.menupopover.hide();
    }

    fn open_action(&self) {
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

    // Signal handlers

    fn search_action_activated(&self) {
        let variant: glib::Variant = self.searchaction.get_state().unwrap();
        let state: bool = variant.get().unwrap();
        let state = !state;

        self.searchaction.set_state(&state.to_variant());

        if state {
            self.start_search_action();
        } else {
            self.stop_search_action();
        }
    }

    fn search_changed(&self) {
        self.search();
    }

    fn stop_search(&self) {
        self.searchaction.set_state(&false.to_variant());

        // A bit redundant, but guarantees we perform the same teardown
        // steps regardless of how the search has been interrupted
        self.stop_search_action();
    }

    fn insert_action_activated(&self) {
    }

    fn menu_action_activated(&self) {
        let variant: glib::Variant = self.menuaction.get_state().unwrap();
        let state: bool = variant.get().unwrap();
        let state = !state;

        self.menuaction.set_state(&state.to_variant());

        if state {
            self.start_menu_action();
        } else {
            self.stop_menu_action();
        }
    }

    fn menu_popover_closed(&self) {
        self.menuaction.set_state(&false.to_variant());
    }

    fn open_action_activated(&self) {
        self.open_action();
    }

    fn save_action_activated(&self) {
    }

    fn number_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_number(path, text);
    }

    fn people_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_people(path, text);
    }

    fn signature_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_signature(path, text);
    }

    fn flags_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_flags(path, text);
    }

    fn date_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_date(path, text);
    }
}

fn main() {
    Application::new().run();
}

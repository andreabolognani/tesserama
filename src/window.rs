// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2018  Andrea Bolognani <eof@kiyuko.org>
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
extern crate time;

use std::cell::RefCell;
use std::cmp;
use std::ffi::OsStr;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use self::gio::prelude::*;
use self::gio::MenuExt;
use self::gtk::prelude::*;

use super::column::Column;
use super::simpleaction::SimpleAction;
use super::liststore::ListStore;
use super::application::Application;

#[derive(Clone)]
pub struct Window {
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
    peoplecolumn: gtk::TreeViewColumn,
    searchaction: SimpleAction,
    insertaction: SimpleAction,
    menuaction: SimpleAction,
    openaction: SimpleAction,
    saveaction: SimpleAction,
    source_filename: Rc<RefCell<PathBuf>>,
    source_uri: Rc<RefCell<String>>,
    dirty: Rc<RefCell<bool>>,
    data: Rc<RefCell<ListStore>>,
    filtered_data: Rc<RefCell<gtk::TreeModelFilter>>,
    filter_needle: Rc<RefCell<String>>,
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let menubutton = gtk::ToggleButton::new();
        let menupopover = gtk::Popover::new(&menubutton);
        let data = ListStore::new();
        let filtered_data = data.create_filter();
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
            peoplecolumn: gtk::TreeViewColumn::new(),
            searchaction: SimpleAction::new_stateful("search", false),
            insertaction: SimpleAction::new("insert"),
            menuaction: SimpleAction::new_stateful("menu", false),
            openaction: SimpleAction::new("open"),
            saveaction: SimpleAction::new("save"),
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
        // Though formally deprecated, set_wmclass() seems to be the only way
        // to reliably convince GNOME Shell to display the proper application
        // name in the topbar, so it's staying :)
        self.parent.set_wmclass("Tesserama", "Tesserama");
        self.parent.set_title("Tesserama");

        self.parent.set_default_size(800, 600);

        let _self = self.clone();
        self.parent.connect_delete_event(move |_,_| {
            _self.delete_event()
        });

        /* Actions */

        let _self = self.clone();
        self.searchaction.as_parent().connect_activate(move |_,_| {
            _self.search_action_activated();
        });
        self.searchaction.set_enabled(false);
        self.parent.add_action(self.searchaction.as_parent());

        let _self = self.clone();
        self.insertaction.as_parent().connect_activate(move |_,_| {
            _self.insert_action_activated();
        });
        self.insertaction.set_enabled(false);
        self.parent.add_action(self.insertaction.as_parent());

        let _self = self.clone();
        self.menuaction.as_parent().connect_activate(move |_,_| {
            _self.menu_action_activated();
        });
        self.parent.add_action(self.menuaction.as_parent());

        let _self = self.clone();
        self.openaction.as_parent().connect_activate(move |_,_| {
            _self.open_action_activated();
        });
        self.parent.add_action(self.openaction.as_parent());

        let _self = self.clone();
        self.saveaction.as_parent().connect_activate(move |_,_| {
            _self.save_action_activated();
        });
        self.saveaction.set_enabled(false);
        self.parent.add_action(self.saveaction.as_parent());

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
        self.menupopover.set_relative_to(&self.menubutton);

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
        column.add_attribute(&number_renderer, "text", Column::Number.into());
        self.treeview.append_column(&column);

        let people_renderer = gtk::CellRendererText::new();
        people_renderer.set_property_ellipsize(pango::EllipsizeMode::End);
        people_renderer.set_property_editable(true);
        let _self = self.clone();
        people_renderer.connect_edited(move |_, path, text| {
            _self.people_cell_edited(path, text);
        });
        self.peoplecolumn.set_title("People");
        self.peoplecolumn.set_expand(true);
        self.peoplecolumn.pack_start(&people_renderer, false);
        self.peoplecolumn.add_attribute(&people_renderer, "text", Column::People.into());
        self.treeview.append_column(&self.peoplecolumn);

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
        column.add_attribute(&signature_renderer, "text", Column::Signature.into());
        self.treeview.append_column(&column);

        let id_renderer = gtk::CellRendererText::new();
        id_renderer.set_property_editable(true);
        let _self = self.clone();
        id_renderer.connect_edited(move |_, path, text| {
            _self.id_cell_edited(path, text);
        });
        let column = gtk::TreeViewColumn::new();
        column.set_title("ID");
        column.set_sizing(gtk::TreeViewColumnSizing::GrowOnly);
        column.pack_start(&id_renderer, false);
        column.add_attribute(&id_renderer, "text", Column::ID.into());
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
        column.add_attribute(&flags_renderer, "text", Column::Flags.into());
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
        column.add_attribute(&date_renderer, "text", Column::Date.into());
        self.treeview.append_column(&column);

        let scrolled = gtk::ScrolledWindow::new(None, None);
        scrolled.add(&self.treeview);

        contents.pack_start(&self.searchbar, false, false, 0);
        contents.pack_start(&scrolled, true, true, 0);

        self.stack.add_named(&empty, "empty");
        self.stack.add_named(&contents, "contents");
        self.parent.add(&self.stack);
    }

    pub fn show_all(&self) {
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

            *data = ListStore::new();
            *filtered_data = data.create_filter();
            *filter_needle = String::new();
        }

        let data: &ListStore = &*self.data.borrow();
        let filtered_data: &gtk::TreeModelFilter = &*self.filtered_data.borrow();
        let path: &PathBuf = &*self.source_filename.borrow();

        let mut reader = csv::ReaderBuilder::new()
                         .has_headers(false)
                         .from_path(path)
                         .expect("Failed to open file");

        for record in reader.records() {
            if record.is_ok() {
                let record: csv::StringRecord = record.unwrap();

                let mut values = ListStore::new_row();

                // Extract values from the record. Missing fields default to
                // the empty string, so that it's possible to load files
                // created using older versions of the application
                for i in 0..record.len() {
                    values[i] = String::from(&record[i]);
                }

                let iter = data.append();
                data.set_all_values(&iter, &values);
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

        match gtk::RecentManager::get_default() {
            Some(recents) => {
                recents.add_item(&*self.source_uri.borrow());
            },
            None => {},
        }
    }

    fn save_data(&self) {

        let path: &PathBuf = &*self.source_filename.borrow();
        let mut writer = csv::WriterBuilder::new()
                         .has_headers(false)
                         .from_path(path)
                         .expect("Failed to open output file");

        let data: &ListStore = &*self.data.borrow();
        let iter: gtk::TreeIter = data.get_iter_first().unwrap();

        loop {
            let mut record = csv::StringRecord::new();

            for x in 0..Column::SIZE {
                let value: Option<String> = data.get_value(&iter, &Column::from(x));

                match value {
                    Some(ref field) => record.push_field(field),
                    None => break,
                }
            }

            if &record[Column::People.into()] != "" {
                writer.write_record(&record).expect("Failed to write output file");
            }

            if !data.iter_next(&iter) { break; }
        }

        writer.flush().expect("Failed to write output file");

        self.set_dirty(false);
    }

    fn filter_func(&self, iter: &gtk::TreeIter) -> bool {
        let filter_needle: &String = &*self.filter_needle.borrow();

        if filter_needle.parse::<i32>().is_ok() {
            // If the needle can be converted to a number, we look up
            // the corresponding record
            self.value_matches(iter, &Column::Number, filter_needle)
        } else {
            // In all other cases, we perform a case-insensitive substring
            // search among people's names and signatures
            (self.value_contains(iter, &Column::People, filter_needle) ||
             self.value_contains(iter, &Column::Signature, filter_needle))
        }
    }

    fn value_matches(&self, iter: &gtk::TreeIter, column: &Column, needle: &str) -> bool {
        let data: &ListStore = &*self.data.borrow();

        data.get_value(iter, column).map_or(false, |value| {
            &value == needle
        })
    }

    fn value_contains(&self, iter: &gtk::TreeIter, column: &Column, needle: &str) -> bool {
        let data: &ListStore = &*self.data.borrow();

        data.get_value(iter, column).map_or(false, |value| {
            let value = value.to_lowercase();

            // Most entries are in the form
            //
            //   LastName FirstName, OtherFirstName
            //
            // to save on typing.
            //
            // We want such an entry to match when searching for
            // "LastName OtherFirstName", and in order to do that we
            // have to split the needle into chunks and check whether
            // all of them are contained in the entry
            needle.split_whitespace().all(|chunk| {
                value.contains(chunk)
            })
        })
    }

    // Returns true if it's okay to discard changes in the current
    // document, either because the user has confirmed by clicking
    // the relative button or because there are none
    fn discard_changes_okay(&self) -> bool {

        let mut ret = false;

        if self.is_dirty() {
            let dialog = gtk::MessageDialog::new(
                Some(&self.parent),
                gtk::DialogFlags::empty(),
                gtk::MessageType::Question,
                gtk::ButtonsType::OkCancel,
                "Discard unsaved changes?",
            );

            // Only proceed if the user has explicitly selected the
            // corresponding option; pressing Cancel or dismissing
            // the dialog by pressing ESC cancels the close operation
            if dialog.run() == gtk::ResponseType::Ok.into() {
                ret = true;
            }

            dialog.destroy()
        } else {
            ret = true;
        }

        ret
    }

    fn convert_path(&self, path: gtk::TreePath) -> gtk::TreePath {
        let filtered_data: &gtk::TreeModelFilter = &*self.filtered_data.borrow();

        // Since we use filtering on the data displayed in the
        // treeview, we have to convert paths from the filtered
        // model to the actual model before using them
        filtered_data.convert_path_to_child_path(&path).unwrap()
    }

    fn update_column(&self, path: gtk::TreePath, column: &Column, text: &str) {
        let data: &ListStore = &*self.data.borrow();
        let path: gtk::TreePath = self.convert_path(path);
        let iter: gtk::TreeIter = data.get_iter(&path).unwrap();
        let value: Option<String> = data.get_value(&iter, column);

        if value.is_some() {
            let current: &String = &value.unwrap();

            if text != current {
                data.set_value(&iter, column, &String::from(text));
                self.set_dirty(true);
            }
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

    fn insert_action(&self) {
        let data: &ListStore = &*self.data.borrow();

        let mut number: i32 = 1;
        let iter: Option<gtk::TreeIter> = data.get_iter_first();

        if iter.is_some() {
            let iter: gtk::TreeIter = iter.unwrap();

            loop {
                let value: Option<String> = data.get_value(&iter, &Column::Number);

                if value.is_some() {
                    number = match value.unwrap().parse::<i32>() {
                        Ok(value) => cmp::max(number, value + 1),
                        Err(_) => number,
                    }
                }

                if !data.iter_next(&iter) { break; }
            }
        }

        let number: String = fmt::format(format_args!("{}", number));

        let date: String = match time::now().strftime("%d/%m/%y") {
            Ok(now) => { fmt::format(format_args!("{}", now)) },
            Err(_) => { String::new() },
        };

        // Create an empty record
        let mut values = ListStore::new_row();

        // Fill in some sensible data: the next number in the
        // sequence and today's date
        values[usize::from(Column::Number)] = number;
        values[usize::from(Column::Date)] = date;

        let iter: gtk::TreeIter = data.append();
        let path: gtk::TreePath = data.get_path(&iter).unwrap();

        // Insert the fresh data
        data.set_all_values(&iter, &values);

        // Scroll to it and start editing right away
        self.treeview.scroll_to_cell(&path, None, false, 0.0, 0.0);
        self.treeview.set_cursor(&path, &self.peoplecolumn, true);
    }

    fn start_menu_action(&self) {
        self.menupopover.show();
    }

    fn stop_menu_action(&self) {
        self.menupopover.hide();
    }

    fn open_action(&self) {
        // Don't overwrite changes unless the user is okay with that
        if !self.discard_changes_okay() {
            return
        }

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

    fn save_action(&self) {
        self.save_data();
    }

    fn close_action(&self) -> glib::signal::Inhibit {
        // false means we want to close the window, true means
        // we don't, so we have to flip the result here
        glib::signal::Inhibit(!self.discard_changes_okay())
    }

    // Signal handlers

    fn search_action_activated(&self) {
        let state = !self.searchaction.get_state().unwrap();

        self.searchaction.change_state(state);

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
        self.searchaction.change_state(false);

        // A bit redundant, but guarantees we perform the same teardown
        // steps regardless of how the search has been interrupted
        self.stop_search_action();
    }

    fn insert_action_activated(&self) {
        self.insert_action();
    }

    fn menu_action_activated(&self) {
        let state = !self.menuaction.get_state().unwrap();

        self.menuaction.change_state(state);

        if state {
            self.start_menu_action();
        } else {
            self.stop_menu_action();
        }
    }

    fn menu_popover_closed(&self) {
        self.menuaction.change_state(false);
    }

    fn open_action_activated(&self) {
        self.open_action();
    }

    fn save_action_activated(&self) {
        self.save_action();
    }

    fn number_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_column(path, &Column::Number, text);
    }

    fn people_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_column(path, &Column::People, text);
    }

    fn signature_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_column(path, &Column::Signature, text);
    }

    fn id_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_column(path, &Column::ID, text);
    }

    fn flags_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_column(path, &Column::Flags, text);
    }

    fn date_cell_edited(&self, path: gtk::TreePath, text: &str) {
        self.update_column(path, &Column::Date, text);
    }

    fn delete_event(&self) -> glib::signal::Inhibit {
        self.close_action()
    }
}

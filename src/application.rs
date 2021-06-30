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
extern crate gtk;

use self::gio::prelude::*;
use self::gtk::prelude::*;

use super::window::Window;

#[derive(Clone)]
pub struct Application {
    parent: gtk::Application,
}

impl Application {
    pub fn new() -> Self {
        let flags = gio::ApplicationFlags::empty();
        let ret = Self {
            parent: gtk::Application::new(None, flags),
        };
        ret.setup();
        ret
    }

    fn setup(&self) {
        glib::set_application_name("Tesserama");

        self.parent.set_accels_for_action("win.search", &["<Ctrl>f"]);
        self.parent.set_accels_for_action("win.insert", &["<Ctrl>i"]);
        self.parent.set_accels_for_action("win.open", &["<Ctrl>o"]);
        self.parent.set_accels_for_action("win.save", &["<Ctrl>s"]);

        let _self = self.clone();
        self.parent.connect_activate(move |_| {
            _self.activate_action();
        });
    }

    pub fn run(&self) {
        self.parent.run();
    }

    pub fn create_window(&self) -> gtk::ApplicationWindow {
        gtk::ApplicationWindow::new(&self.parent)
    }

    fn activate_action(&self) {
        Window::new(&self).show_all();
    }
}

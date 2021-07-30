// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2021  Andrea Bolognani <eof@kiyuko.org>
// SPDX-License-Identifier: GPL-2.0-or-later

use ::gio::prelude::*;
use ::gtk::prelude::*;

use crate::window::Window;

#[derive(Clone)]
pub struct Application {
    parent: gtk::Application,
}

impl Application {
    pub fn new() -> Self {
        let flags = gio::ApplicationFlags::empty();
        let ret = Self {
            parent: gtk::Application::new(Some("org.kiyuko.Tesserama"), flags),
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

// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2021  Andrea Bolognani <eof@kiyuko.org>
// SPDX-License-Identifier: GPL-2.0-or-later

#[derive(Clone)]
pub struct SimpleAction {
    parent: gio::SimpleAction,
}

impl SimpleAction {
    pub fn new(name: &str) -> Self {
        Self {
            parent: gio::SimpleAction::new(name, None),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.parent.set_enabled(enabled);
    }

    pub fn as_parent(&self) -> &gio::SimpleAction {
        &self.parent
    }
}

// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2021  Andrea Bolognani <eof@kiyuko.org>
// SPDX-License-Identifier: GPL-2.0-or-later

use ::gio::prelude::*;

#[derive(Clone)]
pub struct SimpleActionStateful {
    parent: gio::SimpleAction,
}

impl SimpleActionStateful {
    pub fn new(name: &str, state: bool) -> Self {
        Self {
            parent: gio::SimpleAction::new_stateful(
                name,
                None,
                &state.to_variant(),
            )
        }
    }

    pub fn state(&self) -> bool {
        self.parent.state().unwrap().get().unwrap()
    }

    pub fn change_state(&self, state: bool) {
        self.parent.change_state(&state.to_variant());
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.parent.set_enabled(enabled);
    }

    pub fn as_parent(&self) -> &gio::SimpleAction {
        &self.parent
    }
}

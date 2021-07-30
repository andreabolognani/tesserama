// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2021  Andrea Bolognani <eof@kiyuko.org>
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

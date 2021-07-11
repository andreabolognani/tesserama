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

use self::gtk::prelude::*;

use super::column::Column;

pub struct ListStore {
    parent: gtk::ListStore,
}

impl ListStore {
    pub fn new() -> Self {
        Self {
            parent: gtk::ListStore::new(&[glib::Type::STRING; Column::SIZE]),
        }
    }

    pub fn new_row() -> [String; Column::SIZE] {
        [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]
    }

    pub fn append(&self) -> gtk::TreeIter {
        self.parent.append()
    }

    pub fn value(&self, iter: &gtk::TreeIter, column: &Column) -> Option<String> {
        let variant = self.parent.value(iter, i32::from(column.clone())).get::<String>();

        match variant {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    pub fn set_value(&self, iter: &gtk::TreeIter, column: &Column, value: &String) {
        let record: [(u32, &dyn glib::ToValue); 1] = [
            (u32::from(column.clone()), value),
        ];

        self.parent.set(iter, &record);
    }

    pub fn set_all_values(&self, iter: &gtk::TreeIter, values: &[String]) {
        let record: [(u32, &dyn glib::ToValue); Column::SIZE] = [
            (0, &values[0]),
            (1, &values[1]),
            (2, &values[2]),
            (3, &values[3]),
            (4, &values[4]),
            (5, &values[5]),
        ];

        self.parent.set(iter, &record);
    }

    pub fn path(&self, iter: &gtk::TreeIter) -> Option<gtk::TreePath> {
        self.parent.path(iter)
    }

    pub fn iter(&self, path: &gtk::TreePath) -> Option<gtk::TreeIter> {
        self.parent.iter(path)
    }

    pub fn iter_first(&self) -> Option<gtk::TreeIter> {
        self.parent.iter_first()
    }

    pub fn iter_next(&self, iter: &gtk::TreeIter) -> bool {
        self.parent.iter_next(iter)
    }

    pub fn create_filter(&self) -> gtk::TreeModelFilter {
        gtk::TreeModelFilter::new(&self.parent, None)
    }
}

// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2021  Andrea Bolognani <eof@kiyuko.org>
// SPDX-License-Identifier: GPL-2.0-or-later

use ::gtk::prelude::*;

use crate::column::Column;

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

// Tesserama - Simple membership cards manager
// Copyright (C) 2017-2021  Andrea Bolognani <eof@kiyuko.org>
// SPDX-License-Identifier: GPL-2.0-or-later

#[derive(Clone)]
pub enum Column {
    Date,
    Number,
    People,
    Signature,
    Flags,
    ID,
}

impl Column {
    pub const SIZE: usize = 6;
}

impl From<Column> for u8 {
    fn from(c: Column) -> u8 {
        match c {
            Column::Date => 0,
            Column::Number => 1,
            Column::People => 2,
            Column::Signature => 3,
            Column::Flags => 4,
            Column::ID => 5,
        }
    }
}

impl From<u8> for Column {
    fn from(n: u8) -> Column {
        match n {
            0 => Column::Date,
            1 => Column::Number,
            2 => Column::People,
            3 => Column::Signature,
            4 => Column::Flags,
            5 => Column::ID,
            _ => panic!("Numeric value {} can't be converted to column", n),
        }
    }
}

impl From<Column> for u32 {
    fn from(c: Column) -> u32 {
        let c: u8 = c.into();
        c as u32
    }
}

impl From<u32> for Column {
    fn from(n: u32) -> Column {
        Column::from(n as u8)
    }
}

impl From<Column> for i32 {
    fn from(c: Column) -> i32 {
        let c: u8 = c.into();
        c as i32
    }
}

impl From<i32> for Column {
    fn from(n: i32) -> Column {
        Column::from(n as u8)
    }
}

impl From<Column> for usize {
    fn from(c: Column) -> usize {
        let c: u8 = c.into();
        c as usize
    }
}

impl From<usize> for Column {
    fn from(n: usize) -> Column {
        Column::from(n as u8)
    }
}

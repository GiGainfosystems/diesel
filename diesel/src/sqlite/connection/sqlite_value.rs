extern crate libsqlite3_sys as ffi;

use std::marker::PhantomData;
use std::os::raw as libc;
use std::ptr::NonNull;
use std::{slice, str};

use crate::row::*;
use crate::sqlite::{Sqlite, SqliteType};

/// Raw sqlite value as received from the database
///
/// Use existing `FromSql` implementations to convert this into
/// rust values:
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct SqliteValue<'a> {
    value: NonNull<ffi::sqlite3_value>,
    p: PhantomData<&'a ()>,
}

#[derive(Clone)]
pub struct SqliteRow<'a> {
    stmt: NonNull<ffi::sqlite3_stmt>,
    next_col_index: libc::c_int,
    p: PhantomData<&'a ()>,
}

impl<'a> SqliteValue<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) unsafe fn new(inner: *mut ffi::sqlite3_value) -> Option<Self> {
        NonNull::new(inner)
            .map(|value| SqliteValue {
                value,
                p: PhantomData,
            })
            .and_then(|value| {
                // We check here that the actual value represented by the inner
                // `sqlite3_value` is not `NULL` (is sql meaning, not ptr meaning)
                if value.is_null() {
                    None
                } else {
                    Some(value)
                }
            })
    }

    pub(crate) fn read_text(&self) -> &str {
        unsafe {
            let ptr = ffi::sqlite3_value_text(self.value.as_ptr());
            let len = ffi::sqlite3_value_bytes(self.value.as_ptr());
            let bytes = slice::from_raw_parts(ptr as *const u8, len as usize);
            // The string is guaranteed to be utf8 according to
            // https://www.sqlite.org/c3ref/value_blob.html
            str::from_utf8_unchecked(bytes)
        }
    }

    pub(crate) fn read_blob(&self) -> &[u8] {
        unsafe {
            let ptr = ffi::sqlite3_value_blob(self.value.as_ptr());
            let len = ffi::sqlite3_value_bytes(self.value.as_ptr());
            slice::from_raw_parts(ptr as *const u8, len as usize)
        }
    }

    pub(crate) fn read_integer(&self) -> i32 {
        unsafe { ffi::sqlite3_value_int(self.value.as_ptr()) as i32 }
    }

    pub(crate) fn read_long(&self) -> i64 {
        unsafe { ffi::sqlite3_value_int64(self.value.as_ptr()) as i64 }
    }

    pub(crate) fn read_double(&self) -> f64 {
        unsafe { ffi::sqlite3_value_double(self.value.as_ptr()) as f64 }
    }

    /// Get the type of the value as returned by sqlite
    pub fn value_type(&self) -> Option<SqliteType> {
        let tpe = unsafe { ffi::sqlite3_value_type(self.value.as_ptr()) };
        match tpe {
            ffi::SQLITE_TEXT => Some(SqliteType::Text),
            ffi::SQLITE_INTEGER => Some(SqliteType::Long),
            ffi::SQLITE_FLOAT => Some(SqliteType::Double),
            ffi::SQLITE_BLOB => Some(SqliteType::Binary),
            ffi::SQLITE_NULL => None,
            _ => unreachable!("Sqlite docs saying this is not reachable"),
        }
    }

    pub(crate) fn is_null(&self) -> bool {
        self.value_type().is_none()
    }
}

impl<'a> SqliteRow<'a> {
    pub(crate) unsafe fn new(inner_statement: NonNull<ffi::sqlite3_stmt>) -> Self {
        SqliteRow {
            stmt: inner_statement,
            next_col_index: 0,
            p: PhantomData,
        }
    }
}

impl<'a> ExactSizeIterator for SqliteRow<'a> {}

impl<'a> Iterator for SqliteRow<'a> {
    type Item = SqliteField<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let col_index = self.next_col_index;
        if (col_index as usize) < self.len() {
            self.next_col_index += 1;

            Some(SqliteField {
                stmt: self.stmt,
                col_idx: col_index,
                p: PhantomData,
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = column_count(self.stmt) as usize;
        (size, Some(size))
    }
}

pub struct SqliteField<'a> {
    stmt: NonNull<ffi::sqlite3_stmt>,
    col_idx: i32,
    p: PhantomData<&'a ()>,
}

impl<'a> Field<'a, Sqlite> for SqliteField<'a> {
    fn column_name(&self) -> Option<&str> {
        column_name(self.stmt, self.col_idx)
    }

    fn is_null(&self) -> bool {
        self.value().is_none()
    }

    fn value(&self) -> Option<crate::backend::RawValue<'a, Sqlite>> {
        unsafe {
            let ptr = ffi::sqlite3_column_value(self.stmt.as_ptr(), self.col_idx);
            SqliteValue::new(ptr)
        }
    }
}

// pub struct SqliteNamedRow<'a> {
//     stmt: NonNull<ffi::sqlite3_stmt>,
//     column_indices: &'a HashMap<&'a str, usize>,
// }

// impl<'a> NamedRow<Sqlite> for SqliteNamedRow<'a> {
//     fn index_of(&self, column_name: &str) -> Option<usize> {
//         self.column_indices.get(column_name).cloned()
//     }

//     fn get_raw_value(&self, idx: usize) -> Option<SqliteValue<'_>> {
//         unsafe {
//             let ptr = ffi::sqlite3_column_value(self.stmt.as_ptr(), idx as libc::c_int);
//             SqliteValue::new(ptr)
//         }
//     }

//     fn field_names(&self) -> Vec<&str> {
//         (0..column_count(self.stmt))
//             .filter_map(|c| column_name(self.stmt, c))
//             .collect()
//     }
// }

fn column_name<'a>(stmt: NonNull<ffi::sqlite3_stmt>, field_number: i32) -> Option<&'a str> {
    unsafe {
        let ptr = ffi::sqlite3_column_name(stmt.as_ptr(), field_number);
        if ptr.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(ptr).to_str().expect(
                "The Sqlite documentation states that this is UTF8. \
                 If you see this error message something has gone \
                 horribliy wrong. Please open an issue at the \
                 diesel repository.",
            ))
        }
    }
}

fn column_count(stmt: NonNull<ffi::sqlite3_stmt>) -> i32 {
    unsafe { ffi::sqlite3_column_count(stmt.as_ptr()) }
}

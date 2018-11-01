#![feature(crate_visibility_modifier)]

mod database;

use self::database::{Array, ArrayId, ConcatDatabase, DatabaseStruct, InputString, StringId};

use salsa::Database;
use std::sync::Arc;

fn main() {
    let db = DatabaseStruct::default();

    // "Hello" + ", " + "world" + "!"
    let strings = vec![StringId(0), StringId(1), StringId(2), StringId(3)];

    db.query(InputString).set(strings[0], string("Hello"));
    db.query(InputString).set(strings[1], string(", "));
    db.query(InputString).set(strings[2], string("world"));
    db.query(InputString).set(strings[3], string("!"));
    db.query(Array).set(ArrayId(0), Arc::new(strings));

    println!("{:?}", db.output_string(ArrayId(0)));

    db.query(InputString).set(StringId(3), string("?"));
    println!("{:?}", db.output_string(ArrayId(0)));

    db.query(InputString).set(StringId(4), string(" "));
    db.query(Array).set(
        ArrayId(0),
        Arc::new(vec![StringId(0), StringId(4), StringId(2)]),
    );

    println!("{:?}", db.output_string(ArrayId(0)));
}

fn string(v: impl Into<String>) -> Arc<String> {
    Arc::new(v.into())
}

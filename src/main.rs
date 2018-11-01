#![feature(crate_visibility_modifier)]

mod database;

use self::database::{
    ArrayDatabase, ArrayId, DatabaseStruct, InternedArray, InternedString, StringId,
};

use salsa::Database;
use std::sync::Arc;

fn main() {
    let db = DatabaseStruct::default();

    // "Hello" + ", " + "world" + "!"
    let strings = vec![StringId(0), StringId(1), StringId(2), StringId(3)];

    db.query(InternedString).set(strings[0], string("Hello"));
    db.query(InternedString).set(strings[1], string(", "));
    db.query(InternedString).set(strings[2], string("world"));
    db.query(InternedString).set(strings[3], string("!"));
    db.query(InternedArray).set(ArrayId(0), Arc::new(strings));

    println!("{:?}", db.concat(ArrayId(0)));

    db.query(InternedString).set(StringId(3), string("?"));
    println!("{:?}", db.concat(ArrayId(0)));

    db.query(InternedString).set(StringId(4), string(" "));
    db.query(InternedArray).set(
        ArrayId(0),
        Arc::new(vec![StringId(0), StringId(4), StringId(2)]),
    );

    println!("{:?}", db.concat(ArrayId(0)));
}

fn string(v: impl Into<String>) -> Arc<String> {
    Arc::new(v.into())
}

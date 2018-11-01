use std::sync::Arc;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ArrayId(pub usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StringId(pub usize);

salsa::query_group! {
    pub trait ConcatDatabase: salsa::Database {
        fn input_string(key: StringId) -> Arc<String> {
            type InputString;
            storage input;
        }

        fn input_array(key: ArrayId) -> Arc<Vec<StringId>> {
            type Array;
            storage input;
        }

        fn output_string(key: ArrayId) -> Arc<String> {
            type OutputString;
        }
    }
}

fn output_string(db: &impl ConcatDatabase, key: ArrayId) -> Arc<String> {
    let array = db.input_array(key);
    let mut out = String::new();

    for &i in array.iter() {
        out.push_str(&db.input_string(i));
    }

    Arc::new(out)
}

#[derive(Default)]
crate struct DatabaseStruct {
    runtime: salsa::Runtime<DatabaseStruct>,
}

impl DatabaseStruct {}

impl salsa::Database for DatabaseStruct {
    fn salsa_runtime(&self) -> &salsa::Runtime<DatabaseStruct> {
        &self.runtime
    }
}

salsa::database_storage! {
    pub struct DatabaseStorage for DatabaseStruct {
        impl ConcatDatabase {
            fn input_string() for InputString;
            fn input_array() for Array;
            fn output_string() for OutputString;
        }
    }
}

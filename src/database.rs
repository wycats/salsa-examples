use std::sync::Arc;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ArrayId(pub usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StringId(pub usize);

salsa::query_group! {
    pub trait StringDatabase: salsa::Database {
        fn string_by_id(key: StringId) -> Arc<String> {
            type InternedString;
            storage input;
        }
    }
}

salsa::query_group! {
    pub trait ArrayDatabase: StringDatabase {
        fn array_by_id(key: ArrayId) -> Arc<Vec<StringId>> {
            type InternedArray;
            storage input;
        }

        fn concat(key: ArrayId) -> Arc<String> {
            type ConcatString;
        }
    }
}

fn concat(db: &impl ArrayDatabase, key: ArrayId) -> Arc<String> {
    let array = db.array_by_id(key);
    let mut out = String::new();

    for &i in array.iter() {
        out.push_str(&db.string_by_id(i));
    }

    Arc::new(out)
}

#[derive(Default)]
crate struct DatabaseStruct {
    runtime: salsa::Runtime<DatabaseStruct>,
}

impl salsa::Database for DatabaseStruct {
    fn salsa_runtime(&self) -> &salsa::Runtime<DatabaseStruct> {
        &self.runtime
    }
}

salsa::database_storage! {
    pub struct DatabaseStorage for DatabaseStruct {
        impl StringDatabase {
            fn string_by_id() for InternedString;
        }

        impl ArrayDatabase {
            fn array_by_id() for InternedArray;
            fn concat() for ConcatString;
        }
    }
}

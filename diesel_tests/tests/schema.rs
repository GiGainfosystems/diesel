use diesel::*;
use dotenv::dotenv;
use std::env;

use diesel_oci::oracle::connection::OciConnection;

#[cfg(all(feature = "postgres", feature = "backend_specific_database_url"))]
infer_schema!("dotenv:PG_DATABASE_URL");
#[cfg(all(feature = "sqlite", feature = "backend_specific_database_url"))]
infer_schema!("dotenv:SQLITE_DATABASE_URL");
#[cfg(all(feature = "mysql", feature = "backend_specific_database_url"))]
infer_schema!("dotenv:MYSQL_DATABASE_URL");
#[cfg(all(feature = "oracle", feature = "backend_specific_database_url"))]
infer_schema!("dotenv:OCI_DATABASE_URL");
#[cfg(not(any(feature = "backend_specific_database_url", feature="oracle")))]
infer_schema!("dotenv:DATABASE_URL");


#[cfg(feature = "oracle")]

    table! {
        comments (id) {
            id -> Integer,
            post_id -> Integer,
            text -> Text,
        }
    }

    table! {
        composite_fk (id) {
            id -> Nullable<Integer>,
            post_id -> Integer,
            user_id -> Integer,
        }
    }

    table! {
        cyclic_fk_1 (id) {
            id -> Nullable<Integer>,
            cyclic_fk_2_id -> Nullable<Binary>,
        }
    }

    table! {
        cyclic_fk_2 (id) {
            id -> Nullable<Integer>,
            cyclic_fk_1_id -> Nullable<Binary>,
        }
    }

    table! {
        fk_doesnt_reference_pk (id) {
            id -> Nullable<Integer>,
            random -> Nullable<Text>,
        }
    }

    table! {
        fk_inits (id) {
            id -> Nullable<Integer>,
        }
    }

    table! {
        fk_tests (id) {
            id -> Nullable<Integer>,
            fk_id -> Integer,
        }
    }

    table! {
        followings (user_id, post_id) {
            user_id -> Integer,
            post_id -> Integer,
            email_notifications -> Bool,
        }
    }

    table! {
        infer_all_the_bools (col1) {
            col1 -> Bool,
            col2 -> Bool,
            col3 -> Bool,
            col4 -> Bool,
        }
    }

    table! {
        infer_all_the_datetime_types (dt) {
            dt -> Timestamp,
            date -> Date,
            time -> Time,
            timestamp -> Timestamp,
        }
    }

    table! {
        infer_all_the_floats (col1) {
            col1 -> Float,
            col2 -> Float,
            col3 -> Double,
            col4 -> Double,
            col5 -> Double,
            col6 -> Double,
        }
    }

    table! {
        infer_all_the_ints (col1) {
            col1 -> Integer,
            col2 -> Integer,
            col3 -> Integer,
            col4 -> Integer,
            col5 -> SmallInt,
            col6 -> SmallInt,
            col7 -> SmallInt,
            col8 -> BigInt,
            col9 -> BigInt,
            col10 -> BigInt,
            col11 -> SmallInt,
            col12 -> Integer,
            col13 -> BigInt,
        }
    }

    table! {
        infer_all_the_strings (col1) {
            col1 -> Text,
            col2 -> Text,
            col3 -> Text,
            col4 -> Text,
            col5 -> Text,
            col6 -> Text,
            col7 -> Text,
            col8 -> Text,
            col9 -> Binary,
            col10 -> Binary,
        }
    }

    table! {
        likes (comment_id, user_id) {
            comment_id -> Integer,
            user_id -> Integer,
        }
    }

    table! {
        multiple_fks_to_same_table (id) {
            id -> Nullable<Integer>,
            post_id_1 -> Nullable<Binary>,
            post_id_2 -> Nullable<Binary>,
        }
    }

    table! {
        nullable_doubles (id) {
            id -> Nullable<Integer>,
            n -> Nullable<Double>,
        }
    }

    table! {
        nullable_table (id) {
            id -> Integer,
            value -> Nullable<Integer>,
        }
    }

    table! {
        numbers (n) {
            n -> Nullable<Integer>,
        }
    }

    table! {
        points (x, y) {
            x -> Integer,
            y -> Integer,
        }
    }

    table! {
        posts (id) {
            id -> Integer,
            user_id -> Integer,
            title -> Text,
            body -> Nullable<Text>,
        }
    }

    table! {
        precision_numbers (n) {
            n -> Double,
        }
    }

    table! {
        self_referential_fk (id) {
            id -> Nullable<Integer>,
            parent_id -> Integer,
        }
    }

    table! {
        special_comments (id) {
            id -> Nullable<Integer>,
            special_post_id -> Integer,
        }
    }

    table! {
        special_posts (id) {
            id -> Nullable<Integer>,
            user_id -> Integer,
            title -> Text,
        }
    }

    table! {
        trees (id) {
            id -> Integer,
            parent_id -> Nullable<Integer>,
        }
    }

    table! {
        users (id) {
            id -> Integer,
            name -> Text,
            hair_color -> Nullable<Text>,
        }
    }

    table! {
        users_with_name_pk (name) {
            name -> Nullable<Text>,
        }
    }

    table! {
        with_keywords (fn_) {
            #[sql_name = "fn"]
            fn_ -> Integer,
            #[sql_name = "let"]
            let_ -> Integer,
            #[sql_name = "extern"]
            extern_ -> Integer,
        }
    }

    joinable!(comments -> posts (post_id));
    joinable!(fk_tests -> fk_inits (fk_id));
    joinable!(followings -> posts (post_id));
    joinable!(followings -> users (user_id));
    joinable!(likes -> comments (comment_id));
    joinable!(likes -> users (user_id));
    joinable!(posts -> users (user_id));

    allow_tables_to_appear_in_same_query!(
        comments,
        composite_fk,
        cyclic_fk_1,
        cyclic_fk_2,
        fk_doesnt_reference_pk,
        fk_inits,
        fk_tests,
        followings,
        infer_all_the_bools,
        infer_all_the_datetime_types,
        infer_all_the_floats,
        infer_all_the_ints,
        infer_all_the_strings,
        likes,
        multiple_fks_to_same_table,
        nullable_doubles,
        nullable_table,
        numbers,
        points,
        posts,
        precision_numbers,
        self_referential_fk,
        special_comments,
        special_posts,
        trees,
        users,
        users_with_name_pk,
        with_keywords,
    );


#[cfg(feature = "sqlite")]
mod test_infer_schema_works_on_empty_database {
    infer_schema!(":memory:");
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset,
         QueryableByName)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

impl User {
    pub fn new(id: i32, name: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            hair_color: None,
        }
    }

    pub fn with_hair_color(id: i32, name: &str, hair_color: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            hair_color: Some(hair_color.to_string()),
        }
    }

    pub fn new_post(&self, title: &str, body: Option<&str>) -> NewPost {
        NewPost::new(self.id, title, body)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(Post)]
pub struct Comment {
    id: i32,
    post_id: i32,
    text: String,
}

impl Comment {
    pub fn new(id: i32, post_id: i32, text: &str) -> Self {
        Comment {
            id: id,
            post_id: post_id,
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[table_name = "followings"]
#[primary_key(user_id, post_id)]
pub struct Following {
    pub user_id: i32,
    pub post_id: i32,
    pub email_notifications: bool,
}

#[cfg_attr(feature = "postgres", path = "postgres_specific_schema.rs")]
#[cfg_attr(not(feature = "postgres"), path = "backend_specifics.rs")]
mod backend_specifics;

pub use self::backend_specifics::*;

#[derive(Debug, PartialEq, Eq, Queryable, Clone, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub hair_color: Option<String>,
}

impl NewUser {
    pub fn new(name: &str, hair_color: Option<&str>) -> Self {
        NewUser {
            name: name.to_string(),
            hair_color: hair_color.map(|s| s.to_string()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    user_id: i32,
    title: String,
    body: Option<String>,
}

impl NewPost {
    pub fn new(user_id: i32, title: &str, body: Option<&str>) -> Self {
        NewPost {
            user_id: user_id,
            title: title.into(),
            body: body.map(|b| b.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a>(
    #[column_name(post_id)] pub i32,
    #[column_name(text)] pub &'a str,
);

#[derive(PartialEq, Eq, Debug, Clone, Insertable)]
#[table_name = "fk_tests"]
pub struct FkTest {
    id: i32,
    fk_id: i32,
}

impl FkTest {
    pub fn new(id: i32, fk_id: i32) -> Self {
        FkTest {
            id: id,
            fk_id: fk_id,
        }
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "nullable_table"]
pub struct NullableColumn {
    id: i32,
    value: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Queryable, Insertable, Identifiable, Associations)]
#[table_name = "likes"]
#[primary_key(user_id, comment_id)]
#[belongs_to(User)]
#[belongs_to(Comment)]
pub struct Like {
    pub user_id: i32,
    pub comment_id: i32,
}

#[cfg(feature = "postgres")]
pub type TestConnection = PgConnection;
#[cfg(feature = "sqlite")]
pub type TestConnection = SqliteConnection;
#[cfg(feature = "mysql")]
pub type TestConnection = MysqlConnection;
#[cfg(feature= "oracle")]
pub type TestConnection = OciConnection;

pub type TestBackend = <TestConnection as Connection>::Backend;

//Used to ensure cleanup of one-off tables, e.g. for a table created for a single test
pub struct DropTable<'a> {
    pub connection: &'a TestConnection,
    pub table_name: &'static str,
}

impl<'a> Drop for DropTable<'a> {
    fn drop(&mut self) {
        self.connection
            .execute(&format!("DROP TABLE {}", self.table_name))
            .unwrap();
    }
}

pub fn connection() -> TestConnection {
    let result = connection_without_transaction();
    #[cfg(feature = "sqlite")]
    result.execute("PRAGMA foreign_keys = ON").unwrap();
    result.begin_test_transaction().unwrap();
    result
}

#[cfg(feature = "postgres")]
pub fn connection_without_transaction() -> TestConnection {
    dotenv().ok();
    let connection_url = env::var("PG_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests");
    PgConnection::establish(&connection_url).unwrap()
}

#[cfg(feature = "oracle")]
pub fn connection_without_transaction() -> TestConnection {
    dotenv().ok();
    let connection_url = env::var("OCI_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests");
    OciConnection::establish(&connection_url).unwrap()
}

#[cfg(feature = "sqlite")]
embed_migrations!("../migrations/sqlite");

#[cfg(feature = "sqlite")]
pub fn connection_without_transaction() -> TestConnection {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    embedded_migrations::run(&connection).unwrap();
    connection
}

#[cfg(feature = "mysql")]
pub fn connection_without_transaction() -> TestConnection {
    dotenv().ok();
    let connection_url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests");
    MysqlConnection::establish(&connection_url).unwrap()
}

#[cfg(feature = "postgres")]
pub fn disable_foreign_keys(connection: &TestConnection) {
    connection.execute("SET CONSTRAINTS ALL DEFERRED").unwrap();
}

#[cfg(feature = "oracle")]
pub fn disable_foreign_keys(connection: &TestConnection) {
    connection.execute("alter session SET CONSTRAINTS ALL DEFERRED").unwrap();
}

#[cfg(feature = "mysql")]
pub fn disable_foreign_keys(connection: &TestConnection) {
    connection.execute("SET FOREIGN_KEY_CHECKS = 0").unwrap();
}

#[cfg(feature = "sqlite")]
pub fn disable_foreign_keys(connection: &TestConnection) {
    connection
        .execute("PRAGMA defer_foreign_keys = ON")
        .unwrap();
}

#[cfg(feature = "sqlite")]
pub fn drop_table_cascade(connection: &TestConnection, table: &str) {
    connection
        .execute(&format!("DROP TABLE {}", table))
        .unwrap();
}

#[cfg(not(feature = "sqlite"))]
pub fn drop_table_cascade(connection: &TestConnection, table: &str) {
    connection
        .execute(&format!("DROP TABLE {} CASCADE", table))
        .unwrap();
}

sql_function!(fn nextval(a: sql_types::VarChar) -> sql_types::BigInt);

pub fn connection_with_sean_and_tess_in_users_table() -> TestConnection {
    let connection = connection();
    insert_sean_and_tess_into_users_table(&connection);
    connection
}

pub fn insert_sean_and_tess_into_users_table(connection: &TestConnection) {
    connection
        .execute("INSERT INTO users (id, name) VALUES (1, 'Sean'), (2, 'Tess')")
        .unwrap();
    ensure_primary_key_seq_greater_than(2, &connection);
}

pub fn connection_with_nullable_table_data() -> TestConnection {
    let connection = connection();

    let test_data = vec![
        NullableColumn { id: 1, value: None },
        NullableColumn { id: 2, value: None },
        NullableColumn {
            id: 3,
            value: Some(1),
        },
        NullableColumn {
            id: 4,
            value: Some(2),
        },
        NullableColumn {
            id: 5,
            value: Some(1),
        },
    ];
    insert_into(nullable_table::table)
        .values(&test_data)
        .execute(&connection)
        .unwrap();

    connection
}

fn ensure_primary_key_seq_greater_than(x: i64, connection: &TestConnection) {
    if cfg!(feature = "postgres") {
        for _ in 0..x {
            select(nextval("users_id_seq")).execute(connection).unwrap();
        }
    }
}

pub fn find_user_by_name(name: &str, connection: &TestConnection) -> User {
    users::table
        .filter(users::name.eq(name))
        .first(connection)
        .unwrap()
}

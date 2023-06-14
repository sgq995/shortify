use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Url {
    pub id: i32,
    pub hash: String,
    pub url: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CreateUrl<'a> {
    pub hash: &'a String,
    pub url: &'a String,
}

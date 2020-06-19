use crate::schema::books;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
}
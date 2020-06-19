use diesel_db::{establish_connection, create_book, get_books};

fn main() {
    println!("Hello world");
    // Postgres connection
    let connection = establish_connection();
    let title = String::from("NAME OF THE WIND");
    let author = String::from("Patrick Rothfus");

    let post = create_book(&connection,&title,&author);
    println!("Book({}, {}) has been saved",post.title,post.author);
    let books = get_books(&connection, 10,0);
    for book in books.iter() {
        println!("title:{}, author: {}", book.title, book.author);
    }
}
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

struct Library<'a> {
    books: Vec<Book<'a>>,
}

impl<'a> Library<'a> {
    fn new() -> Self {
        Library {
            books: Vec::new()
        }
    }

    fn add_book(&mut self, title: &'a str, author: &'a str) {
        let book = Book { title, author};
        self.books.push(book)
    }

    fn find_longest_title(&self) -> Option<&Book<'a>> {
        self.books.iter().max_by_key(|book| book.title.len())
    }

    fn find_author<'b>(&self, author: &'b str) -> Vec<&Book<'a>> {
        self.books.iter().filter(|&book|book.author == author).collect()
    }
}

fn main() {
    let author1 = "George Orwell";
    let author2 = "Aldous Huxley";
    let title1 = "1984";
    let title2 = "Brave New World";

    let mut library = Library::new();
    library.add_book(author1, title1);
    library.add_book(author2, title2);

    if let Some(longest) = library.find_longest_title() {
        println!("The book with the longest title is: {}", longest.title);
    }
    else {
        println!("No books in the library.");
    }

    let search_author = "George Orwell";
    let found_books = library.find_author(search_author);
    println!("Books by {}: {:?}", search_author, found_books.iter().map(|book| book.title).collect::<Vec<_>>());
}

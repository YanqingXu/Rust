struct Wold<'a> {
    word: &'a str,
}

impl<'a> Wold<'a> {
    fn new(word: &'a str) -> Self {
        Wold { word }
    }

    fn longest<'b>(words: &'b [&'a str]) -> Option<Wold<'a>> {
        let mut longest_word: Option<&str> = None;
        for &word in words {
            if longest_word.is_none() || word.len() > longest_word.as_ref().unwrap().len() {
                longest_word = Some(word);
            }
        }

        longest_word.map(|w| Wold::new(w))
    }
}

fn main() {
    let words = vec!["apple", "banana", "pear", "orange"];
    if let Some(longest_word) = Wold::longest(&words) {
        print!("The longest word is {}", longest_word.word);
    }
    else {
        print!("No found!");
    }
}

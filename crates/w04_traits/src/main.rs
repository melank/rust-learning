trait Summary {
    fn summarize(&self) -> String;
}

struct News {
    title: String,
    author: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let news = News {
        title: String::from("Rust hits production"),
        author: String::from("team"),
    };

    println!("{}", news.summarize());
}

#[cfg(test)]
mod tests {
    use super::{News, Summary};

    #[test]
    fn summarizes_news() {
        let news = News {
            title: String::from("A"),
            author: String::from("B"),
        };

        assert_eq!(news.summarize(), "A by B");
    }
}

/// Writing a specific instance of a function and making it generic ...

fn largest_u32(arr : &[u32]) -> u32 {
    let mut large = arr[0];

    for i in arr.iter() {
        if i > &large {
            large = *i;
        }
    }

    large
}

/// above using generics
/// PartialOrd + Copy --> PartialOrd is for sortablity and Copy is to limit to stack only types
fn largest<T : PartialOrd + Copy>(arr : &[T]) -> T {
    let mut large = arr[0];

    for i in arr.iter() {
        if i > &large {
            large = *i;
        }
    }

    large
}

/// Traits - a way of grouping types with similar behavior

trait Summary {
    fn summarize(&self) -> String; 
}

#[derive(Debug)]
struct Tweet {
    author : String,
    text : String,
}

impl Summary for Tweet {

    fn summarize(&self) -> String {
        format!("{} by {}", self.text, self.author)
    }
}

#[derive(Debug)]
struct Article {
    author: String,
    title: String,
    synopsis: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 2, 3, 4];
    println!("Largest of {:?} is {}.", arr, largest_u32(&arr));
    println!("Largest of {:?} is {}.", arr, largest(&arr));

    let tweet = Tweet {
        author: String::from("Bala"),
        text: String::from("Hello world"),
    };

    println!("Summarized tweet: {:?} - {}.", tweet, tweet.summarize());

    // traits can be passed as parameters, returned from functions
}

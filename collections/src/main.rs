use std::collections::HashMap;

fn main() {

    let text = "alice in wonderland; hello world - your alice";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let c = word_count.entry(word).or_insert(0);
        *c += 1;
    }

    println!("{:?}", word_count);

    let mut mc = -1;
    let mut mw = String::new();
    for (w, c) in word_count.iter() {
        if *c > mc {
            mc = *c;
            mw = String::from(*w); // need to do this because we are using string slices above
        }
    }

    println!("Most frequently occurring word: {}", mw);

    let arr = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", arr);
}

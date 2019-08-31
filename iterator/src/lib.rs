#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}


// here shoes should be moved to this function as filter needs to take
// ownership of the iterable
fn filter_shoes_by_size(shoes : Vec<Shoe>, size : u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|shoe| shoe.size == size)
        .collect()
}

struct Counter {
    count : u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

// implementing trait Iterator for Counter
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 5 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn it_works() {
        let shoes = vec![
            Shoe{size: 8, style: String::from("boot")},
            Shoe{size: 9, style: String::from("sneaker")},
            Shoe{size: 10, style: String::from("sandal")},
        ];

        assert_eq!(
            filter_shoes_by_size(shoes, 8),
            vec![
                Shoe{size: 8, style: String::from("boot")},
            ]
        );
    }

    #[test]
    fn iterator_test() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn iterator_test_1() {

        // here, skip, zip, map, filter and sum work because there are defined
        // in terms of Iterator.next function
        let result : u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(x, y)| x * y)
            .filter(|x| x%3 == 0)
            .sum();

        assert_eq!(result, 18);
    }
}

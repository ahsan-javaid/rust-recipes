struct Fibonacci {
    curr: u32,
    next: u32
}

// impl Iterator trait for Fibonacci struct
impl Iterator for Fibonacci {
    type Item = u32; // Self:: Item

    fn next(&mut self) -> Option<Self::Item> {
        // println!("called {}", self.curr);
        let current = self.curr;

        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        curr: 0,
        next: 1
    }
}

fn main() {
    let sequence = fibonacci();
    let mut iter = sequence.into_iter();

    // using next
    while let Some(item) = iter.next() {
        println!("value: {}", item);
        if item > 10_000 {
            break;
        }
    }
    // using take : Creates an iterator that yields the first n elements, or fewer if the underlying iterator ends sooner.
    for i in fibonacci().take(10) {
        println!("{i}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_10() {
        let sequence = fibonacci();
        let mut iter = sequence.into_iter();
        let expected = [0,1,1,2,3,5,8,13,21,34];
       
        for i in expected {
            let val = iter.next().unwrap();
            assert_eq!(val, i);
        }
    }
}

struct Filter<I, F> {
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            // Access fn stored in predicate and call it by passing item ref
            if (self.predicate)(&item) {
                return Some(item);
            }
        }

        None
    }
}

fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    Filter {
        iter: iter.into_iter(),
        predicate,
    }
}

fn main() {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elitm";

    let v = filter(text.split_whitespace(), |word| {
        if word.contains("m") {
            true
        } else {
            false
        }
    });

    let got: Vec<&str> = v.collect();

    println!("val {:?}", got);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_words() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elitm";

        let v = filter(text.split_whitespace(), |word| {
            if word.contains("m") {
                true
            } else {
                false
            }
        });

        let got: Vec<&str> = v.collect();

        let expected = vec!["Lorem", "ipsum", "amet,", "elitm"];

        assert_eq!(got, expected);
    }
}

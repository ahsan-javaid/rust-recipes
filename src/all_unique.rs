use std::collections::HashSet;

fn all_unique(input: &str) -> bool {
    let mut hashset: HashSet<char> = HashSet::new();

    for c in input.chars() {
        if hashset.contains(&c) {
            return false;
        }

        hashset.insert(c);
    }

    true
}

fn main() {
    let name = "abcdefga";

    let ans = all_unique(name);

    println!("output: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_true() {
        let name = "abcdefg";

        let got = all_unique(name);

        assert_eq!(got, true);
    }


    #[test]
    fn unique_false() {
        let name = "abcdefga";

        let got = all_unique(name);

        assert_eq!(got, false);
    }
}

fn main() {}

#[allow(dead_code)]
fn find_first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    let tt = bytes.iter().enumerate();

    println!("---> {:#?}", tt);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let s = String::from("Silav u rrez");
        assert_eq!(find_first_word(&s), 5);
    }
}

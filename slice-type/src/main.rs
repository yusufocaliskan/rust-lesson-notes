fn main() {
    let t = String::from("Silav u re ");

    find_first_word(&t);
}
fn find_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

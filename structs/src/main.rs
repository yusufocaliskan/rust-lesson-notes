use std::fmt;

// #[derive(Debug, PartialEq)]
#[derive(PartialEq)]
#[allow(dead_code)]
struct Book {
    name: String,
    is_active: bool,
    price: i64,
    lang: String,
}

//Custom debugger
impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.lang.as_str() {
            "ku" => write!(
                f,
                "Pirtuk:  {{ Nav: {:?} Rews: {:?} Buha: {:?} Ziman: {:?}}}",
                self.name, self.is_active, self.price, self.lang
            ),
            "tr" => write!(
                f,
                "Kitap:  {{ Ad: {:?} Durum: {:?} Fiyat: {:?} Dil: {:?} }}",
                self.name, self.is_active, self.price, self.lang
            ),
            _ => write!(
                f,
                "Book:  {{ Name: {:?} Status: {:?} Price: {:?} Language: {:?} }}",
                self.name, self.is_active, self.price, self.lang
            ),
        }
    }
}

#[allow(dead_code)]
fn create_book(_name: String, _is_active: bool, _price: i64, _lang: String) -> Book {
    Book {
        name: _name,
        is_active: _is_active,
        price: _price,
        lang: _lang,
    }
}

fn main() {
    let book = create_book(String::from("Tu ne hati!"), false, 30, String::from("ku"));
    let book_tr = create_book(
        String::from("Saf Aklin Elestirisi"),
        false,
        30,
        String::from("tr"),
    );
    println!("{:#?}", book);
    println!("{:#?}", book_tr);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn book_test() {
        let book = create_book(
            String::from("Hasretinden Prangalar Eskittim"),
            true,
            40,
            String::from("tr"),
        );
        let expected_book = Book {
            name: String::from("Hasretinden Prangalar Eskittim"),
            is_active: true,
            price: 50,
            lang: String::from("tr"),
        };
        assert_eq!(book, expected_book)
    }
}

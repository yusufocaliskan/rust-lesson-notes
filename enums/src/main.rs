#[derive(Debug)]
enum Status {
    Published,
    Draft,
}

#[derive(Debug)]
struct Post {
    title: String,
    content: String,
    date: String,
    create_at: String,
}

#[derive(Debug)]
struct Author {
    name: String,
    create_at: String,
    age: Option<u32>,
}

#[derive(Debug)]
struct Blog {
    post: Post,
    author: Author,
    status: Status,
}

impl Blog {
    fn new(post: Post, author: Author, status: Status) -> Blog {
        Blog {
            post,
            author,
            status,
        }
    }
}

fn main() {
    let new_blog = Blog::new(
        Post {
            title: String::from("Ji Xelke meye bindest re"),
            content: String::from("/ Enum for author category/ Enum for author category/"),

            date: String::from("05/09/2024"),
            create_at: String::from("03/09/2024"),
        },
        Author {
            name: String::from("Bahox X"),
            create_at: String::from("03/09/2024"),
            age: None,
            // age: Some(32),
        },
        Status::Published,
    );

    println!("--> Blog : {:#?}", new_blog);
    // println!("--> Title : {}", new_blog.post.title);
    // println!("--> Content : {}", new_blog.post.title);
}

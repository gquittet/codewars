fn main() {
    println!("{}", reverse_words("Hello World!"));
}

fn reverse_words(str: &str) -> String {
    let mut text = str.split(" ").collect::<Vec<&str>>();
    text.reverse();
    text.join(" ")
}

// Best solution
// fn reverse_words(str: &str) -> String {
//     str.split_whitespace().rev().collect::<Vec<_>>().join(" ")
// }

#[test]
fn returns_expected() {
    assert_eq!(reverse_words("hello world!"), "world! hello");
}

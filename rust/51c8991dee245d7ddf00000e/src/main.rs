fn main() {
    println!("{}", reverse_words("Hello World!"));
}

fn reverse_words(str: &str) -> String {
    let mut text = str.split(" ").collect::<Vec<&str>>();
    text.reverse();
    return text.join(" ");
}

#[test]
fn returns_expected() {
    assert_eq!(reverse_words("hello world!"), "world! hello");
}

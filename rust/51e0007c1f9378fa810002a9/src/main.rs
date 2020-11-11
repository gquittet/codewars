fn main() {
    println!("{:?}", parse("iiisdoso"));
}

fn parse(code: &str) -> Vec<i32> {
    let mut n: i32 = 0;
    let mut result: Vec<i32> = vec![];
    for c in code.chars().into_iter() {
        match c {
            'i' => n += 1,
            'd' => n -= 1,
            's' => n *= n,
            'o' => result.push(n),
            _ => {}
        }
    }
    result
}

#[test]
fn sample_tests() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}

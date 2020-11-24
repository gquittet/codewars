fn main() {
    println!("Hello, world!");
}


fn range_extraction(a: &[i32]) -> String {
    if a.len() == 0 {
        return "".to_string();
    }

    let mut result = format!("{}", a[0]);
    let mut temp: Vec<i32> = Vec::new();

    for (index, &n) in a[1..].iter().enumerate() {
        if n == a[index] + 1 {
            temp.push(n);
        } else {
            if temp.len() >= 2 {
                result = format!("{}-{},{}", result, temp.last().unwrap(), n);
            } else if temp.len() > 0 && temp.len() < 2 {
                result = format!("{},{},{}", result, temp.last().unwrap(), n);
            } else {
                result = format!("{},{}", result, n);
            }
            temp = Vec::new();
        }
    }

    if temp.len() >= 2 {
        result = format!("{}-{}", result, temp.last().unwrap());
    } else if temp.len() > 0 && temp.len() < 2 {
        result = format!("{},{}", result, temp.last().unwrap());
    }

    result
}


#[test]
fn should_range_extraction() {
    assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));
    assert_eq!("-3--1,2,10,15,16,18-20", range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
}
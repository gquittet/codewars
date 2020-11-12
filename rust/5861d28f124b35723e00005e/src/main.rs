fn main() {
    println!("Hello, world!");
}

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    gallons * mpg >= distance_to_pump
}

#[test]
fn sample_tests() {
    assert_eq!(zero_fuel(50, 25, 2), true);
    assert_eq!(zero_fuel(100, 50, 1), false);
    assert_eq!(zero_fuel(60, 30, 3), true);
    assert_eq!(zero_fuel(70, 25, 1), false);
    assert_eq!(zero_fuel(100, 25, 3), false);
    assert_eq!(zero_fuel(93, 24, 3), false);
}
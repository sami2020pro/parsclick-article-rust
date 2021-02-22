fn main() {
    let string = String::from("Hello World from sami2020pro");
    let length = calculate_length(&string);

    println!("The length of '{}' is {}", string, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
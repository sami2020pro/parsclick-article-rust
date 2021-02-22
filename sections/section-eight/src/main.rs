fn main() {
    let string = String::from("Hello World from sami2020pro");
    let length = calculate_length(&string);

    println!("The length of '{}' is {}", string, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let x: usize = 1; // usize can hold to zero or more than zero
    let y: usize = 0; // usize can hold to zero or more than zero
    let z: isize = -10; // isize can hold to zero or more than zero OR less than zero

    println!("x {} | y {} | z {}", x, y, z);
}

fn main() {
    let x: usize = 1; // usize can hold to zero or more than zero
    let y: usize = 0; // usize can hold to zero or more than zero
    let z: usize = -10; // Error, We need ISIZE !!!

    println!("x {} | y {} | z {}", x, y, z);
}
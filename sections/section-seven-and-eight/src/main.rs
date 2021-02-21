fn main() {
    {
        let x = String::from("Hello World"); // از اینجا به بعد متغییر x در دسترس هست 
        println!("{}", x); // هرکاری که میخواید رو با متغیر x انجام میدید
    } // اینجا اسکوپی که ایجاد کرده بودیم تموم میشه و متغیر x از بین میره و دیگه در دسترس نیست
}

fn main() {
    {
        let x = String::from("Hello World");
    }
    println!("{}", x); // Error
}

fn main() {
    let string1 = String::from("Hello World from sami2020pro");
    let string2 = string1;

    println!("{}", string2);
}

fn main() {
    let string1 = String::from("Hello World from sami2020pro");
    let string2 = string1;

    println!("{}", string2);
    println!("{}", string1); // Error
}
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

fn main() {
    let s = String::from("Hello World");  // متغییر اس وارد اسکوپ میشه و از اینجا به بعد در دسترس هست

    takes_ownership(s);             // متغییر اس انتقال داده شده به این فانکشن و دیگه در دسترس نیست

    let x = 5;                      // متغییر ایکس وارد اسکوپ میشه و از اینجا به بعد در دسترس هست

    makes_copy(x);                  // متغییر ایکس میبایست به این فانکشن انتقال داده میشد اما خوده تایپ `i32` کپی هست
                                    // و هنوز ایکس در دسترس هست
                                    // در ادامه کپی رو توضیح خواهیم داد

}  // در اینجا ایکس بیرون میره و بعد اس هم از اسکوپ بیرون میره ولی اس انتقال داده شده و اتفاقی نمیوفته

fn takes_ownership(some_string: String) { // اینجا متغییرمون وارد این اسکوپ شده
    println!("{}", some_string);
} // اینجا متغییرمون از اسکوپ خارج شده و `drop` صدا زده شده و از حافظه یا همون مموری متغییرمون رو حذف میکنه 

fn makes_copy(some_integer: i32) { // اینجا متغییرمون وارد این اسکوپ شده
    println!("{}", some_integer);
} // اینجا متغییرمون از اسکوپ خارج میشه ولی اتفاق خاصی نمیوفته

fn main() {
	let s = String::from("Hello World");
	take_ownership(s);

	println!("{}", s); // Error
}

fn take_ownership(some_string: String) {
	println!("{}", some_string);
}

fn main() {
    let s1 = gives_ownership(); // این فانکشن آنرشیپ رو به متغییرمون میده
    let s2 = String::from("hello"); // در اینجا مقداری رو به متغییری دادیم در واقع یه متغییر تعریف کردیم
    let s3 = takes_and_gives_back(s2); // اینجا آنرشیپ رو گرفتیم از متغییر `اس تو` و به متغییر `اس تری` دادیم
}

// این فانکشن آنرشیپی رو به کسی میده
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

// این فانکشن آنرشیپ رو از کسی میگیره و به کسه دیگه ای میده
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
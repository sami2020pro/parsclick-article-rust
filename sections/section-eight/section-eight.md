# بخش هشت

توی یکی دیگه از مقالات راست میخوایم بخش هشت رو با هم بخوونیم.

توی بخش هشت میخوایم مفاهیم `references` و `borrowing` رو توی راست یادبگیریم.

![مقاله ی آموزشی زبان راست در پارس کلیک](https://images.pexels.com/photos/1194713/pexels-photo-1194713.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940)

در بخش قبلی یعنی بخش هفت پیش نیاز این بخش رو یادگرفتیم و الان مشکلی برای این بخش نداریم اگه هنوز بخش قبل رو متوجه نشدید پیشنهاد میکنم دوباره بخوونید.

در بخش قبل که مفهوم `ownership` رو یاد گرفتیم دیدیم وقتی مثلا یک استرینگ رو به یه فانکشن میدیم اون فانکشن `ownership` رو تغییر میده و صاحب اون مقدار میشه اما فرض بگیرید میخوایم یه استرینگی رو به یه فانکشن بفرستیم که طول اون استرینگ رو برگردونه و بعد خوده استرینگ رو چاپ کنیم با طولش اینجا ما نیاز داریم که `ownership` اون استرینگ براش اتفاقی نیوفته اما چطور ؟

برای همین موضوع مفهوم `reference` و `borrowing` مطرح شد.

به مثال زیر توجه کنید

```rust
fn main() {
    let string = String::from("Hello World from sami2020pro");
    let length = calculate_length(&string);

    println!("The length of '{}' is {}", string, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

به خروجی کد بالا هم توجه کنید

```
The length of 'Hello World from sami2020pro' is 28
```



برای درک بهتر این موضوع بیاید با موارد زیر آشنا بشیم

- References
- Raw pointers
- Smart pointers
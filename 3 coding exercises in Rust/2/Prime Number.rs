use std::io;

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false; // اعداد کمتر از ۲ اول نیستند
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false; // اگر عددی مقسوم‌علیه داشته باشد، اول نیست
        }
    }
    true // اگر هیچ مقسوم‌علیهی پیدا نشد، عدد اول است
}

fn main() {
    println!("لطفاً یک عدد صحیح مثبت وارد کنید:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("خطا در خواندن ورودی");

    let num: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ورودی نامعتبر! لطفاً یک عدد صحیح مثبت وارد کنید.");
            return;
        }
    };

    if is_prime(num) {
        println!("عدد {} یک عدد اول است.", num);
    } else {
        println!("عدد {} یک عدد اول نیست.", num);
    }
}

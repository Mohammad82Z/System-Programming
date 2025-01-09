use std::io;

// تابعی برای محاسبه عدد nام در دنباله فیبوناچی به صورت بازگشتی
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("لطفاً یک عدد صحیح مثبت وارد کنید:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("خطا در خواندن ورودی");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("لطفاً یک عدد صحیح معتبر وارد کنید.");
            return;
        }
    };

    let result = fibonacci(n);
    println!("عدد {}ام در دنباله فیبوناچی برابر است با: {}", n, result);
}

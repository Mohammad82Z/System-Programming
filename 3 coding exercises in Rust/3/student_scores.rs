use std::io;

fn main() {
    let mut students: Vec<(String, f64)> = Vec::new(); // لیست دانشجویان و نمراتشان

    println!("تعداد دانشجویان را وارد کنید:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("خطا در خواندن ورودی");
    let student_count: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("لطفاً یک عدد معتبر وارد کنید.");
            return;
        }
    };

    // گرفتن اطلاعات دانشجویان
    for i in 1..=student_count {
        println!("نام دانشجوی شماره {} را وارد کنید:", i);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("خطا در خواندن ورودی");

        println!("نمره {} را وارد کنید:", name.trim());
        let mut score_input = String::new();
        io::stdin()
            .read_line(&mut score_input)
            .expect("خطا در خواندن ورودی");
        let score: f64 = match score_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("نمره نامعتبر! دوباره تلاش کنید.");
                return;
            }
        };

        students.push((name.trim().to_string(), score));
    }

    // محاسبه میانگین نمرات
    let total_score: f64 = students.iter().map(|(_, score)| score).sum();
    let average = total_score / student_count as f64;
    println!("میانگین نمرات: {:.2}", average);

    // نمایش دانشجویان با نمره کمتر از میانگین
    println!("دانشجویانی که نمره آنها کمتر از میانگین است:");
    for (name, score) in &students {
        if *score < average {
            println!("{} با نمره {:.2}", name, score);
        }
    }
}

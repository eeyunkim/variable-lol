use std::io;

struct Heart {
    radius: f64,
}

impl Heart {
    fn new(radius: f64) -> Self {
        Heart { radius }
    }

    fn area(&self) -> f64 {
        (4.0 * std::f64::consts::PI * self.radius.powi(2) - std::f64::consts::PI * self.radius.powi(2)) / 2.0
    }
}

fn main() {
    let heart = get_user_input("하트의 반지름을 입력하세요 : ");
    let area = heart.area();

    println!("하트의 넓이는 {}입니다.", area);
}

fn get_user_input(prompt: &str) -> Heart {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

        match input.trim().parse() {
            Ok(radius) => return Heart::new(radius),
            Err(_) => println!("올바른 숫자를 입력하세요."),
        }
    }
}

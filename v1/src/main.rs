fn main() {
    let number;
    let name = input_text();
    loop {
       let text = input_text();
        let parse_result = text.trim().parse::<i32>();

        match parse_result {
            Ok(num) => {
               number = calculate_double(num);
               break;
            },
            Err(e) => {
                println!("String NOT ALLOWED")
            }
        }
    }

    println!("hello, {}!", name);
    println!("double num: {}", number);
}


fn input_text() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn calculate_double(num: i32) -> i32 {
    num * 2
}


fn main() {
    println!("Добро пожаловать в Rust!");
    greet("Art", 10);
}

fn greet(name: &str, age: u8) {
    let age2 = age as i32; // этот код превращяет u8 В i32 
    let mut x = age as i32; // тоже самое только мутабельное говно
    println!("Hi bro: {}, your age: {}", name, age);
    println!("Hello its test!: {}", add(age2, 2));

    testik(&mut x);

    println!("test function testik() -> {}", x);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn testik(age: &mut i32) {
    *age +=1;
}
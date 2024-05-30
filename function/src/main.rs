enum Color {
    Red,
    Green,
    Blue,
}

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let x = 5;
    println!("Değiştirilemez x: {}", x);

    let mut y = 10;
    println!("Başlangıç y: {}", y);
    y = 20;
    println!("Değiştirilmiş y: {}", y);
    /*

    */
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("İsim: {}, Yaş: {}", person.name, person.age);
    /*

    */
    let color = Color::Green;

    match color {
        Color::Red => println!("Renk Kırmızı"),
        Color::Green => println!("Renk Yeşil"),
        Color::Blue => println!("Renk Mavi"),
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn data_type(){
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';

    println!("Tamsayı: {}", integer);
    println!("Ondalık sayı: {}", float);
    println!("Boolean: {}", boolean);
    println!("Karakter: {}", character);
}

fn control_blocks(){
    let number = 7;

    if number < 5 {
        println!("Sayı 5'ten küçüktür");
    } else {
        println!("Sayı 5'ten büyük veya eşittir");
    }

    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }
        println!("Sayaç: {}", counter);
        counter += 1;
    }

    for i in 0..5 {
        println!("For döngüsü i: {}", i);
    }

    while counter > 0 {
        println!("While döngüsü sayaç: {}", counter);
        counter -= 1;
    }
}
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Sayaç: {}", counter);

        if counter == 5 {
            break;
        }
    }
}

fn while_loop(){
    let mut number = 3;

    while number != 0 {
        println!("Sayı: {}", number);
        number -= 1;
    }

    println!("Bitti!");
}

fn for_loop(){
    for i in 1..5 {
        println!("i: {}", i);
    }

    for i in 1..=5 {
        println!("i (dahil): {}", i);
    }

}

fn for_loop_arr(){
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("Element: {}", element);
    }
}

fn condition_loop(){
    for number in 1..10 {
        if number % 2 == 0 {
            continue; // Çift sayıları atla
        }

        if number == 7 {
            break; // Döngüyü 7'ye ulaştığında sonlandır
        }

        println!("Tek sayı: {}", number);
    }
}

fn label(){
    'outer: for i in 1..5 {
        for j in 1..5 {
            if i == 3 {
                break 'outer;
            }
            println!("i: {}, j: {}", i, j);
        }
    }
}
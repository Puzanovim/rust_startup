use std::io;


fn main() {
    let mut number = String::new();
    println!("Enter your number: ");
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please type a number");

    if number < 3 {
        println!("Number is smaller then 3");
    } else if number > 3 {
        println!("Number is greater then 3");
    } else {
        println!("Number is equal 3");
    }

    let my_number = 10;
    let message = if number == my_number { "We're the same" } else { "We're not the same" };
    println!("Your number={number}. My number={my_number}. {message}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End loop. Count = {count}");

    println!("Start count = {count}");
    while count > 0 {
        println!("{count}!");
        count -= 1;
    }
    println!("End count!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }
    for number in (1..4).rev() {
        println!("Number is {number}");
    }
}

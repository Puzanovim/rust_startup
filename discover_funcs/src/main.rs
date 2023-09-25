fn main() {
    println!("Hello, world!");
    another_func();
    let x = get_five();
    let y = plus_one(x);
    let sum = sum_numbers(x, y);
    println!("x={x}, y={y}, their sum = {sum}");
}

fn another_func() {
    println!("another function.");
}

fn get_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn sum_numbers(x: i32, y: i32) -> i32 {
    x + y
}

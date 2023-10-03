use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error while reading n");
    let n: i32 = n.trim().parse().expect("Error while parse n");

    print_fibonacci(n);
}

fn print_fibonacci(n: i32) {
    let mut last = 1;
    let mut current = 0;
    
    for _i in 0..n {
        let new = last + current;
        print!("{new} ");
        *&mut last = current;
        *&mut current = new;
    }
}

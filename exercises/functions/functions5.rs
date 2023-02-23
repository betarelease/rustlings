// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.



fn main() {
    let input = 5;
    let answer = square(input);
    println!("The square of {} is {}", input, answer);
}

fn square(num: i32) -> i32 {
    num * num
}

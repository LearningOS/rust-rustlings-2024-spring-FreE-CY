// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3"; // don't rename this variable
    let num :i32 = number.parse().expect("Not a number!");
    println!("Number plus two is : {}", num + 2);
}

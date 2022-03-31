use std::io;

fn main(){
    println!("Enter your age: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: usize = number.trim().parse().expect("Not a number");

    if number < 18 {
        println!("You are not eligible to drive on road!");
    }else if number == 18{
        println!("You are 18 and first get your licence, you can drive then, but be careful!!");
    }else{
        println!("You can drive freely dude!");
    }
}
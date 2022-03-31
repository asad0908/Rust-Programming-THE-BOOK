fn main(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);

    let number = five();
    println!("The value of number is {}", number);

    println!("The square of 8 is {}", square_of(8));
}

fn five() -> u8 {
    5
}

fn square_of(x: i16) -> i16{
    x*x
}
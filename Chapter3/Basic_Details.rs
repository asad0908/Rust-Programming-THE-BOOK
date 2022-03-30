use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is {}", x);
    
    // x = 6;
    // println!("The value of x is {}", x);
    
    // const AGE: u16 = 256;
    // println!("The age of the player is {}", AGE);
    
    // let x = 15;
    // println!("The value of outer first value of x is {}", x);
    // let x = x + 1;
    
    // {
    //     let x = x + 2;
    //     println!("The value of inner value of x is {}", x);
    // }
    // println!("The value of outer value of x is {}", x);
    
    
    // let spaces = "     ";
    // let spaces = spaces.len();
    
    // println!("Spaces are: {}", spaces);
    
    // let number: u32 = "42".parse().expect("Not a number!");
    // println!("The number is {}", number);
    
    // let age: i8 = -128;
    // println!("The value of age is {}", age);
    
    // let decimal_number = 98_222;
    // let hex_number = 0xff;
    // let octal_number = 0o77;
    // let binary_number = 0b1111_0000;
    // let byte_number = b'A';
    
    // println!("Decimal Number: {}, Hex Number: {}, Octal Number: {}, Binary Number: {}, Byte Number: {}", decimal_number, hex_number, octal_number, binary_number, byte_number);
    
    
    // let difference = 95.5 - 15.3;
    // println!("The value of difference is {}", difference);
    
    // let quotient = 96.2 / 2.5;
    // println!("The value of quotient is {}", quotient);
    
    // let quotient2 = 3/5;
    // println!("The value of quotient2 is {}", quotient2);
    
    // let quotient3 = 3.0/5.0;
    // println!("The value of quotient2 is {}", quotient3);
    
    // let remainder = 10%3;
    // println!("The value of remainder is {}", remainder);
    
    // let mut bool = true;
    // println!("The value of bool is {}", bool);
    // bool = false;
    // println!("The value of bool is {}", bool);
    
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';
    
    // println!("Value of c is {}, z is {}, heartEyedCat is {}", c, z, heart_eyed_cat);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of x = {}, y is {}, z is {}", x, y, z);
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;
    // println!("The value of six point four is {}", six_point_four);

    // let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "July", "Aug", "Sept", "Oct", "Nov", "Dec"];
    // println!("My birthday is in {}", months[10]);
    // let numbers: [i32; 5] = [1,2,3,4,5];
    // println!("Number at index 2 is {}", numbers[2]);

    // let array_with_same_value = [3; 5]; // [3,3,3,3,3];
    // println!("The value in the array is {}", array_with_same_value[4]);
    

    let bro_gang = ["Asad", "Obaid", "Hamid", "Kalim", "Faraz", "Arshan"];
    println!("Please enter an index between 0 to 5 ");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let member = bro_gang[index];

    println!("The member of the bro gang at index {} is {}", index, member);

}
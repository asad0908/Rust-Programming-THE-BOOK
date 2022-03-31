use std::io::{self, Write};
//this above line can be also written as
//use std::io;
//use std::io::Write
//using two lines

//Fahrenheit to Celsius or viceversa conversion
fn main(){
    let mut operation = String::new();

    loop {
        println!("");
        println!("*********************************************************");
        println!("***************** Temperature converter *****************");
        println!("*********************************************************");
        println!("");
    
        println!("Select operation from below: ");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("");
        print!("Select operation: ");
        let _ = io::stdout().flush();
    
        io::stdin().read_line(&mut operation).expect("Failed to read line!");
    
        let operation: usize = operation.trim().parse().expect("Not a valid number");
    
        if operation == 1 {operation_convertion('C')} else if operation == 2 {operation_convertion('F')} else {
            println!("Enter a valid output!");
        }
        // println!("");
        // println!("Do you want to do it again! [Y,N]");
        // if false{
        //     break;
        // }
    }
    
}

fn operation_convertion(to: char){
    let mut temp_t = String::new();
    let conv_t;
    if to == 'C'{
        println!("Selected Operation: Fahrenheit to Celsius");
        println!("");
        print!("Enter Fahrenheit temperature: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut temp_t).expect("Failed to read line!");
        let temp_t: f64 = temp_t.trim().parse().expect("Not a vaid number");
        conv_t = (temp_t - 32.0)/1.8;
        println!("{}°F is {}°C of temperature!", temp_t, conv_t);
    }else{
        println!("Selected Operation: Celsius to Fahrenheit");
        println!("");
        print!("Enter Celsius temperature: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut temp_t).expect("Failed to read line!");
        let temp_t: f64 = temp_t.trim().parse().expect("Not a vaid number");
        conv_t = (temp_t*1.8)+32.0;
        println!("{}°C is {}°F of temperature!", temp_t, conv_t);
    }
}

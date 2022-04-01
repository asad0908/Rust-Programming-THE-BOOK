// fn main(){
//     let s: String = String::from("Hello!");
//     takes_ownership(s); // s is passed here and moved to another variable in takes_ownership
//     println!("{}", s);  // s cannot be used here as its moved
// }

// fn takes_ownership(some_string: String){
//     println!("{}", some_string);
// }

fn main(){
    let s: String = String::from("Hello");
    let z: u32 = 20;
    takes_ownership(s); // s is moved, his owner is changed...
    makes_copy(z); // fixed len variables are copied and not moved...
    println!("{} from fn main", z); // we can use z even after the function, unlike s;

    let returned_string: String = give_ownership();
    println!("{}", returned_string);

    let give_and_get_back_string: String = String::from("Hello!");
    let s3: String = takes_and_gives_back(give_and_get_back_string);
    println!("{}", s3);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(x: u32){
    println!("{}", x);
}

fn give_ownership() -> String {
    let some_string: String = String::from("I am from give ownership!");
    some_string //expression which returns, no ;
}

fn takes_and_gives_back(val: String) -> String{
    let returning_val: String = val;
    println!("{} val from takes and gives back!", returning_val);
    returning_val //expression
}
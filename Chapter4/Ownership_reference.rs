// fn main(){
//     let mut s = String::from("Hello");
//     //print_using_reference(&s); //if we pass the reference the ownership is not transferred and we can still use it but we cant modify the string in child function
//     //println!("{} val of s in main after passing it to function", s);

//     print_using_reference_and_change(&mut s);
//     println!("{} after reference and change", s);

// }


// fn print_using_reference_and_change(some_string: &mut String){
//     *some_string = String::from("world!");
// }


fn main(){
    let mut s: String = String::from("Hello!");
    let r1 = &mut s;
    //let r2 = &mut s; //cannot borrow a mutable variable twice..
    println!("{}", r1);
    //println!("{}", r2);
}
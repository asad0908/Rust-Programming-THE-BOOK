// fn main(){
//     //two mutable reference to one mutable string
//     let mut s = String::from("Hello!");
//     {
//         let _r1 = &mut s;
//         //use r1 here and after it goes out of scope it vanishes
//     }
//     let _r2 = &mut s;
//     //we can use r2 here...
// }

// fn main(){
//     let mut s = String::from("Hello!");
//     let r1 = &s;
//     let r2 = &s;
//     // let r3 = &mut s;
//     //println!("{}, {}, {}", r1, r2, r3); // this cannot be done as r3 can change the data reading for r1 and r2...
//     //so we can do one thing
//     println!("{}, {}", r1, r2);
//     let r3 = &mut s;
//     println!("{}", r3);
// }

fn main(){
    let mut s: String = String::from("Hello");
    s = takes_and_back(s);
    println!("{}", s);
}

fn takes_and_back(s: String) -> String {
    let y: String = s;
    println!("{}", y);
    y
}
// fn main(){
//     let mut s: String = String::from("Hello World!");
//     let word = first_word(&s);

//     let _s_n = s.split_off(word);
//     println!("{}", s);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//     s.len()
// }

// fn main(){
//     let s: String = String::from("Hello World!");
//     let len = s.len();
//     let hello = &s[0..5]; //same as &s[..5];
//     let world = &s[6..12]; //same as &s[6..];
//     let hello_world = &s[0..len]; //same as [..];
//     println!("{} {}", hello, world)
// }


fn main(){
    let string: String = String::from("HelloWorld! this is Asad");
    println!("{}", first_word(&string));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate(){
        if item ==b' '{
            return &s[..i]; 
        }
    }
    &s[..]
}
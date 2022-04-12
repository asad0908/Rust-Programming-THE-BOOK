use std::io;

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// fn main(){
//   let user1 = User{
//       email: String::from("fakeemail@gmail.com"),
//       active: true,
//       username: String::from("fake_user"),
//       sign_in_count: 1
//   };
// }

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main(){
    println!("Enter username and email for user to be created!");
    let mut Username: String = String::new();
    io::stdin().read_line(&mut Username).expect("Failed to read line!");
    let mut Email: String = String::new();
    io::stdin().read_line(&mut Email).expect("Failed to read line!");
    let created_user = build_user(Email, Username);

    println!("");
    println!("User created successfully");
    println!("Email of newly created user is {} and username is {}", created_user.email, created_user.username);
}
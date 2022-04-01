fn main(){
    //--------Ownership Rules-----------
    //1. Each value in Rust has a variable thats called its owner
    //2. There can only be one owner at a time
    //3. When the owner goes out of the scope, the value will be dropped
    // {
    //     //s is not valid here
    //     let s: &str = "Hello"; //s is valid from this point
    //     //s is valid uptil here
    // }   //s is not valid outside scope;

    let x: u8 = 5;
    let y: u8 = x; //Copy
    println!("The value of x and y is {} and {}", x, y);

    let a: &str = "Hello";
    let b: &str = a;
    println!("The value of a is '{}' and b is '{}'", a, b);

    let m: String = String::from("Hello, world");
    let n: String = m;
    //we can now use n but not m, m is moved to n....
    //if we need a copy of m in n and also in m, we need to clone it
    // let _n: String = m.clone();

    println!("the value of n is '{}'", n);
}
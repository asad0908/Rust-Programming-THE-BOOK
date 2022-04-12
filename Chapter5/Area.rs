
//normal approach
// fn main(){
//     let width = 10;
//     let height = 20;

//     println!("Let the area of the rectangle is {}", area(width, height));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width*height
// }

//tuple approach
// fn main(){
//     let dimension = (30, 50);

//     println!("The area of the rectangle is {}", area(dimension));
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0*dimension.1
// }

//struct approach
struct Rectangle{
    width: u32,
    height: u32
}

fn main(){
    let rect1 = Rectangle{
        width: 10,
        height: 5
    };
    println!("The area of the rectangle is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
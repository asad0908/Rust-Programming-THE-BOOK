// fn main(){
//     let mut count = 0;
//     loop{
//         println!("Again and Again!");
//         count = count + 1;
//         if count == 10{
//             break;
//         }
//     }
// }

// fn main(){
//     let mut count = 0;
//     'counting_up: loop{
//         println!("Count = {}", count);
//         let mut remaining = 10;

//         loop{
//             println!("Remaining number: {}", remaining);
//             if remaining == 9{
//                 break;
//             }
//             if count == 2{
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count: {}", count);
// }

// fn main() {
//     let mut counter = 0;
//     let result = loop{
//         counter += 1;
//         if counter == 10 {
//             break counter*2;
//         }
//     };
//     println!("The value of the result is {}", result);
// }

// fn main(){
//     let mut number = 3;
//     while number != 0{
//         println!("{}!", number);
//         number -= 1;
//     }
//     println!("Liftoff!!!!");
// }

fn main(){
    // let index = 0;
    // let array: [u32; 5] = [1,2,3,4,5];

    // while index != array.len(){
    //     println!("The value of array at index {} is {}", index, array[index]);
    //     index = index + 1;
    // }

    // for element in array{
    //     println!("The value of the element is {}", element);
    // }

    // for (i, el) in array.iter().enumerate(){
    //     println!("The current element is {}", el);
    //     println!("The index of current element is {}", i);
    //     println!("");
    // }

    // for number in (1..4).rev(){
    //     println!("{}!", number);
    // }
    // println!("LIFEOFFF!!!");

    for number in 1..4{
        println!("{}!", number);
    }
    println!("LIFEOFFF!!!");
}
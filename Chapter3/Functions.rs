fn main(){
    display_info("Asad", 21, 'M');
    display_info("Obaid", 22, 'R');
    display_info("Hamid", 23, 'K');
    display_info("Faraz", 20, 'M');
    display_info("Kalim", 21, 'S');
    display_info("Arshan", 21, 's');
}

fn display_info(name: &str, age: u8, surname: char){
    println!("My name is {} {}, I am {} yrs old!", name, surname, age);
}
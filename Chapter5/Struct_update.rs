struct Student{
    name: String,
    age: u64,
    is_passed: bool,
    email: String,
    college_name: String
}

fn main(){
    let student_1 = Student{
        name: String::from("Asad Memon"),
        age: 21,
        is_passed: true,
        email: String::from("asad@gmail.com"),
        college_name: String::from("RGIT")
    };
    println!("Student {} aged {} studies in {} college", student_1.name, student_1.age, student_1.college_name);

    let student_2 = Student{
        name: String::from("Obaid Khan"),
        email: String::from("obaid@gmail.com"),
        ..student_1
    };
    println!("Student {} aged {} studies in {} college", student_2.name, student_2.age, student_2.college_name);
    // println!("Student {} aged {} studies in {} college", student_1.name, student_1.age, student_1.college_name);
    //now student1 string details are moved to student2 (ownership) and now it cant be used after this...
}
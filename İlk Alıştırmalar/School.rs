#[derive(Debug)]
struct Student {
    number: u32,
    guardian: String,
}

impl Student {
    fn new(number: u32, guardian: String) -> Self {
        Student {
            number,
            guardian,
        }
    }
}

#[derive(Debug)]
struct School {
    students: Vec<Student>,
}

impl School {
    fn new() -> Self {
        School {
            students: vec![],
        }
    }
}

fn print_student(student: Student) -> Student {  
    println!("{:#?}", student);
    student  // Returned
}

fn print_school(school: School) {  
    println!("{:#?}", school);
}

fn main() {
    let student = Student::new(1, String::from("Ahmet"));
    let school = School::new();
 
    print_school(school);  // School is moved and can no longer be used

    let student = print_student(student);

    println!("{:#?}", student.guardian);
}

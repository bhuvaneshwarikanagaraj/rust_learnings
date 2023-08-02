// Add a new student: Each student has a name, roll number, and marks in three subjects (Math, English, and Science).

// Display student details: Display the details of all students, including their names, roll numbers, and total marks.

// Calculate grade: Calculate the grade of each student based on their total marks using the following grading system:

// Total marks less than 150: Grade "F"
// Total marks between 150 and 200: Grade "D"
// Total marks between 200 and 250: Grade "C"
// Total marks between 250 and 300: Grade "B"
// Total marks above 300: Grade "A"
// Search for a student: Given a student's roll number, display the details of that student.

// Exit the program: Allow the user to exit the grading system

    
struct students{
      name:String,
      roll_number:i32,
      maths_marks:i32,
      science_marks:i32,
      english_marks:i32,
}

fn display_function(Students:students, mut total_marks:i32){

    println!("Name {}",Students.name);
    println!("Roll Number {}",Students.roll_number);
    total_marks= Students.maths_marks+Students.science_marks+Students.english_marks;
    println!("Total Marks {}",total_marks);

    match total_marks {
        0..=149=>println!("Grade F"),
        150..=199=>println!("Grade D"),
        200..=249=>println!("Grade C"),
        250..=300=>println!("Grade B"),
        _=>println!("Grade A"),
    }
}

fn main(){
    let mut total_marks:i32=0;

    let student1=students{
        name: String::from("Raju"),
        roll_number:34,
        maths_marks: 95,
        english_marks: 85,
        science_marks: 90,
};

let student2 = students {
    name: String::from("Kaliya"),
    roll_number: 32,
    maths_marks: 80,
    english_marks: 70,
    science_marks: 75,
};

let student3 = students {
    name: String::from("jagu"),
    roll_number: 32,
    maths_marks: 80,
    english_marks: 70,
    science_marks: 75,
};

let student4 = students {
    name: String::from("Bheem"),
    roll_number: 32,
    maths_marks: 80,
    english_marks: 70,
    science_marks: 75,
};

let student4 = students {
    name: String::from("Dolu"),
    roll_number: 32,
    maths_marks: 80,
    english_marks: 70,
    science_marks: 75,
};

    display_function(student1, total_marks);
    display_function(student2, total_marks);
    display_function(student3, total_marks);
    display_function(student4, total_marks);

}

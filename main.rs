use std::io;

struct Course{
    course_id : String,
    grade : String
}

impl Course{
    fn create_new_course(id_of_course:String,  grade:String)-> Course {
        Course{
            course_id : id_of_course.to_string(),
            grade: grade.to_string()
        }
    }
}


struct StudentData{
    student_id : String,
    courses : Vec<Course>
}

impl StudentData{
    fn create_student(id_of_student : String, student_courses_array:Vec<Course>) -> StudentData {
        StudentData{
            student_id : id_of_student.to_string(),
            courses : student_courses_array
        }
    }
}

fn is_grade_valid(grade:String) -> bool {

    let grade_in_lower_case = grade.to_lowercase();

    if grade_in_lower_case.contains('a') || grade_in_lower_case.contains('b') || grade_in_lower_case.contains('c') || grade_in_lower_case.contains('d') || grade_in_lower_case.contains('e') || grade_in_lower_case.contains('f'){
        return true
    }
    else{
        return false
    }
}

fn main() {
    
    println!("Welcome to the Student Data Entry Program, here, you will be required to input 10 student data, with 5 courses for each of them");

    let mut student_data_array : Vec<StudentData> = vec![];

    for student_index in 0..10{
        println!("Please input the student Id ( Mat_ No ) {} : " , student_index);

        let mut student_id = String::from("");
        io::stdin().read_line(&mut student_id).unwrap().to_string();

        let mut count_of_ilterations = 0;
        let mut array_of_student_courses : Vec<Course> = vec![];

       loop{

        println!("What is the Course Code ? : ");

        let mut course_code = String::from("");

        io::stdin().read_line(&mut course_code).unwrap().to_string();

        println!("What is the Grade of {} ? ", course_code.trim());

        let mut course_grade = String::from("");

        io::stdin().read_line(&mut course_grade).unwrap().to_string();

        if is_grade_valid(course_grade.clone()) == true{

            let new_course = Course::create_new_course(course_code, course_grade);

            array_of_student_courses.push(new_course);

            count_of_ilterations+=1;
        }
        else{
            println!("Grade input is not valid, please try again - You have to rewrite the course and the grade \n");
            continue;
        }

        if count_of_ilterations == 5{
            let student_data = StudentData::create_student(student_id, array_of_student_courses);
            student_data_array.push(student_data);
            break;
        }

       }
    }


    for student_data in student_data_array{
            println!("The Student Id {} has the following grades", student_data.student_id.trim());

            for course_data in student_data.courses{
                println!("{}   :   {}", course_data.course_id.trim(), course_data.grade);
            }
    }

}

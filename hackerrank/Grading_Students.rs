fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        }
    }).collect()
}

fn main() {
    let grades = vec![73, 67, 38, 33];
    let result = grading_students(&grades);
    for grade in result {
        println!("{}", grade);
    }
}
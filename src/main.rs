fn main() {
    welcome_message();
    let mut grades: Vec<i32> = Vec::new();
    // delcare a consant named MAX_GRADES and assign it the value 10
    let max_grade: usize = get_num_classes();

    for _ in 0..max_grade {
        grades.push(grade_from_user());
    }
    println!("Grades: {:?}", grades);
    println!("Average: {}", calculate_average(&grades));
    println!("Highest Grade: {}", highest_grade(&grades));
    println!(
        "Average Letter Grade: {}",
        convert_to_letter(calculate_average(&grades) as i32)
    );
    println!("Lowest Grade: {}", lowest_grade(&grades));
}

fn convert_to_letter(grade: i32) -> String {
    match grade {
        90..=100 => String::from("A"),
        80..=89 => String::from("B"),
        70..=79 => String::from("C"),
        60..=69 => String::from("D"),
        _ => String::from("F"),
    }
}

fn calculate_average(grades: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for grade in grades {
        sum += *grade;
    }
    sum as f64 / grades.len() as f64
}

fn highest_grade(grades: &Vec<i32>) -> i32 {
    let mut high = grades[0];
    for grade in grades {
        if *grade > high {
            high = *grade;
        }
    }
    high
}

fn lowest_grade(grades: &Vec<i32>) -> i32 {
    let mut low = grades[0];
    for grade in grades {
        if *grade < low {
            low = *grade;
        }
    }
    low
}

fn grade_from_user() -> i32 {
    println!("Enter a grade: ");
    let mut grade = String::new();
    std::io::stdin()
        .read_line(&mut grade)
        .expect("Failed to read line");
    grade.trim().parse().expect("Please type a number!")
}

fn get_num_classes() -> usize {
    println!("Enter the number of classes: ");
    let mut num_classes = String::new();
    std::io::stdin()
        .read_line(&mut num_classes)
        .expect("Failed to read line");
    num_classes.trim().parse().expect("Please enter a number!")
}

fn welcome_message() {
    println!("Welcome to the Grade Calculator!");
}

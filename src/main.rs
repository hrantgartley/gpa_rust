use rand::prelude::*;
fn main() {
    let mut grades: Vec<i32> = Vec::new();
    // delcare a consant named MAX_GRADES and assign it the value 10
    const MAX_GRADES: usize = 10;

    for _ in 0..MAX_GRADES {
        grades.push(simulate_grade());
    }
    println!("Grades: {:?}", grades);
    println!("Average: {}", calculate_average(&grades));
    println!("Highest Grade: {}", highest_grade(&grades));
    println!(
        "Average Letter Grade: {}",
        convert_to_letter(calculate_average(&grades) as i32)
    );
}

fn simulate_grade() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(30..100)
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

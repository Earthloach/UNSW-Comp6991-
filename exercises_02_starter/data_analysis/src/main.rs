use core::f64;
use std::collections::HashMap;
use serde::Deserialize;
use csv::{Position, ReaderBuilder};
const ENROLMENTS_PATH: &str = "enrolments.psv";

#[derive(Deserialize, Debug)]
struct Student {
    course_code: String,
    zid: String,
    name: String,
    program: String,
    plan: String,
    wam: f64,
    session: String,
    birthdate: String,
    sex: String,
}
fn main() {
    let mut reader= ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(ENROLMENTS_PATH)
        .unwrap();
    
    let mut students: HashMap<String, Student> = HashMap::new();

    reader.deserialize().for_each(|row| {
        let student: Student = row.unwrap();
        students.insert(student.zid.clone(), student);
    });

    println!("Number of students: {}", students.len());

    let mut courses: HashMap<String, u32> = HashMap::new();
    reader.seek(Position::new()).unwrap();
    reader.deserialize().for_each(|row| {
        let student: Student = row.unwrap();
        courses.entry(student.course_code)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    let (course, count) = courses
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();

    println!("Most common course: {} with {} students", course, count);

    let (course, count) = courses
        .iter()
        .min_by_key(|(_, count)| *count)
        .unwrap();

    println!("Least common course: {} with {} students", course, count);

    let total_wam = students.values()
        .map(|student| student.wam)
        .sum::<f64>(); 
    let average_wam = total_wam / students.len() as f64;

    println!("Average WAM: {:.02}", average_wam);
}

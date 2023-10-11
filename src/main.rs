use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug)]
struct Student {
    pub first: u32,
    pub second: u32,
}

impl Default for Student {
    fn default() -> Self {
        Student {
            first: 0,
            second: 0,
        }
    }
}

fn iskola_kereses(student: usize, students: &Vec<Student>, schools: &mut Vec<(u32, u32)>, results:&mut Vec<u32>) -> bool {
    if student >= students.len() {
        match check_results(schools) {
            Some(()) => {
                println!("{:?}", results);
                return true;
            },
            None => return false
        }
    }
    for kb in [students[student].first, students[student].second] {
        if kb == 0 { continue; }
        schools[kb as usize - 1].1 += 1;
        match check_results(schools) {
            Some(()) => {
                if iskola_kereses(student + 1, students, schools, results) {
                    results.push(kb);
                    return true;
                }
            }
            None => {
                if student < students.len() {
                    if iskola_kereses(student + 1, students, schools, results) {
                        results.push(kb);
                        return true;
                    }
                }
                schools[kb as usize - 1].1 -= 1;
            },
        }
    }
    match check_results(schools) {
        Some(()) => return true,
        None => return false
    }
}

fn check_results(schools: &mut Vec<(u32, u32)>) -> Option<()> {
    for school in schools {
        if school.0 != school.1 {
            // println!("no");
            return None;
        }
    }
    // println!("yes");
    Some(())
}

fn main() -> Result<()> {
    let mut string = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut string)?;
    let lines = string.split("\n").collect::<Vec<&str>>();
    let (n, m): (usize, usize) = (
        lines[0].split(" ").nth(0).unwrap().parse().unwrap(),
        lines[0].split(" ").nth(1).unwrap().parse().unwrap(),
    );

    let mut students: Vec<Student> = vec![];
    let mut schools: Vec<(u32, u32)> = vec![];
    for line in lines[1..n + 1].into_iter() {
        let split = line.split(" ").collect::<Vec<&str>>();
        if split.len() < 2 {
            students.push(Student {
                first: split[0].parse().expect("PARSE ERROR"),
                ..Student::default()
            })
        } else {
            students.push(Student {
                first: split[0].parse().unwrap(),
                second: split[1].parse().unwrap(),
            })
        }
    }
    for line in lines[n + 1..n + 1 + m].into_iter() {
        schools.push((line.parse().unwrap(), 0));
    }

    let mut results = vec![];
    iskola_kereses(0, &students, &mut schools, &mut results);

    results.reverse();
    println!("{:?}::{:?}", results, schools);

    Ok(())
}

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

fn iskola_kereses(student: usize, students: &Vec<Student>, schools: &mut Vec<(u32, u32)>, result: &mut Vec<u32>, results: &mut Vec<Vec<u32>>) {
    for kb in [students[student].first, students[student].second] {
        if kb == 0 { continue; }
        match check_results(schools, kb) {
            Some(()) => {
                schools[kb as usize - 1].1 += 1;
                result.push(kb);
                if student < students.len() - 1 {
                    iskola_kereses(student + 1, students, schools, result, results);
                }
                else {
                    results.push(result.clone());
                }
                schools[kb as usize - 1].1 -= 1;
                result.pop();
            }
            None => (),
        }
    }
}

fn check_results(schools: &mut Vec<(u32, u32)>, kb: u32) -> Option<()> {
    if schools[kb as usize - 1].1 + 1 <= schools[kb as usize - 1].0 {
        return Some(());
    }
    None
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
    let mut result = vec![];
    iskola_kereses(0, &students, &mut schools, &mut result, &mut results);

    results.reverse();
    println!("{:?}", results);

    Ok(())
}

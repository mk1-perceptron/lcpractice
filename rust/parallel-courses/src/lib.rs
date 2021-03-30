//! Parallel Courses
//!
//! You are given an integer n which idicates that we have n courses, labeled
//! from 1 to n. You are also given an array relations where relations[i]
//! = [1, b], representing a prerequisite relationship between course a and
//! course b: course a has to be studied before course b.
//!
//! In one semester, you can study any number of courses as long as you have
//! studied all the prerequisites for the course you are studying.
//!
//! Return the minimum number of semesters needed to study all courses. If
//! there is not way to study all the courses, return -1.

use std::collections::{HashMap, HashSet};

pub fn minimum_semesters(_: i32, relations: Vec<Vec<i32>>) -> i32 {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    for rel in relations {
        let key = rel[0];
        let value = rel[1];
        graph.entry(key).or_default().insert(value);
        graph.entry(value).or_default();
        in_degree.entry(key).or_default();
        *in_degree.entry(value).or_default() += 1;
    }

    let mut semester_count = 0;
    let mut class_count = 0;
    let mut semester: Vec<i32> = in_degree
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
        .collect();

    while !semester.is_empty() {
        for class in semester.iter() {
            in_degree.remove(&class);
            for dep in graph.entry(*class).or_default().iter() {
                *in_degree.entry(*dep).or_default() -= 1;
            }
            class_count += 1;
        }

        semester = in_degree
            .iter()
            .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
            .collect();

        semester_count += 1;
    }

    if class_count == graph.len() {
        semester_count
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let n = 3;
        let relations = vec![vec![1, 3], vec![2, 3]];
        let result = minimum_semesters(n, relations);

        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let n = 3;
        let relations = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let result = minimum_semesters(n, relations);

        assert_eq!(result, -1);
    }
}

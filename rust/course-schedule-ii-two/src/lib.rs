//! Course Schedule II
//!
//! Attempt to do course schedule II without help.
//!
//! There are a total of n courses you have to take labelled from 0 to n - 1.
//!
//! Some courses may have prerequisites, for example,
//! iuf prerequisites[i] = [ai, bi] this means you must take the course bi
//! before the course ai.
//!
//! Given the total number of courses numCourses and a list of the
//! prerequisite pairs, return the ordering of courses you should take to finish
//! all courses.
//!
//! If there are many valid answers, return any of them. If it is impossible to finish all courses,
//! return an empty array.

use std::collections::VecDeque;

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); num_courses];
    let mut in_degree: Vec<usize> = vec![0; num_courses];
    for relation in prerequisites {
        let course = relation[0];
        let prereq = relation[1] as usize;
        graph[prereq].push(course);
        in_degree[course as usize] += 1;
    }

    let mut ordered: Vec<i32> = Vec::with_capacity(num_courses);
    let mut count = 0;
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if *n == 0 { Some(i as i32) } else { None })
        .collect();

    while let Some(n) = queue.pop_front() {
        count += 1;
        ordered.push(n);
        let n = n as usize;
        for course in graph[n].iter() {
            let c = *course as usize;
            in_degree[c] -= 1;

            if in_degree[c] == 0 {
                queue.push_back(*course);
            }
        }
    }

    if count == num_courses {
        ordered
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let num_courses = 2;
        let prereqs = vec![vec![1, 0]];
        let result = find_order(num_courses, prereqs);

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_two() {
        let num_courses = 4;
        let prereqs = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let result = find_order(num_courses, prereqs);

        assert!(vec![vec![0, 2, 1, 3], vec![0, 1, 2, 3]].contains(&result));
    }

    #[test]
    fn example_three() {
        let num_courses = 1;
        let prereqs = vec![];
        let result = find_order(num_courses, prereqs);
        assert_eq!(result, vec![0]);
    }
}

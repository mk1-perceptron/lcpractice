//! There are a total of numCourses courses you have to take, labeled from 0 to
//! numCourses - 1, You are given an array prerequisites where
//! prerequisites[i] = [ai, bi] indicates that you must take course bi
//! first if you want to take course ai.
//!
//! For example, the pai [0, 1], indicates that to take course 0 you
//! have to frist take course 1.
//!
//! Return the ordering of courses you should take to finish all courses. If there
//! are many valid answers, return any of them. If it is impossible to finish all
//! courses, return an empty array.

use std::collections::{HashMap, VecDeque};

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(num_courses);
    let mut in_degree: HashMap<i32, u32> = HashMap::with_capacity(num_courses);

    for n in 0..num_courses {
        let n = n as i32;
        graph.entry(n).or_default();
        in_degree.entry(n).or_default();
    }

    for pair in prerequisites.iter() {
        if let [c, p] = pair[..] {
            graph.entry(p).or_default().push(c);
            *in_degree.entry(c).or_default() += 1;
        }
    }

    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
        .collect();
    let mut top_order: Vec<i32> = Vec::new();
    let mut i = 0;

    while let Some(c) = queue.pop_front() {
        top_order.push(c);
        i += 1;

        for &n in graph.get(&c).unwrap().iter() {
            let deg = in_degree.get_mut(&n).unwrap();
            *deg -= 1;

            if *deg == 0 {
                queue.push_back(n);
            }
        }
    }

    if i == num_courses {
        top_order
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let num = 2;
        let prereqs = vec![vec![1, 0]];
        let result = find_order(num, prereqs);

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_two() {
        let num = 4;
        let prereqs = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let result = find_order(num, prereqs);

        assert!(vec![vec![0, 2, 1, 3], vec![0, 1, 2, 3]].contains(&result));
    }

    #[test]
    fn example_three() {
        let num = 1;
        let prereqs = vec![];
        let result = find_order(num, prereqs);

        assert_eq!(result, vec![0]);
    }
}

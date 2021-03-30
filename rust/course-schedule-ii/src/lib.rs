//! Course Schedule II
//!
//! There are a total of n courses you have to take labelled from 0 to n - 1.
//!
//! Some courses may have prerequisites, for example,
//! if prerequisites[i] = [ai, bi] this means you must take the course bi
//! before the course ai.
//!
//! Given the total number of courses numCourses and a list of the
//! prerequisite pairs, return the ordering of courses you should take to finish
//! all courses.
//!
//! If there are many valid answers, return any of them. If it is impossible to
//! finish all courses, return an empty array.

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); num_courses as usize];
    let mut in_degree: Vec<i32> = vec![0; num_courses as usize];
    let mut top_order: Vec<i32> = vec![0; num_courses as usize];

    for edge in prerequisites {
        let v = edge[0];
        let i = edge[1] as usize;
        graph[i].push(v);
        in_degree[v as usize] += 1;
    }

    let mut queue: std::collections::VecDeque<i32> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == 0 { Some(i as i32) } else { None })
        .collect();

    let mut i = 0;
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        top_order[i] = node;
        i += 1;

        for &n in graph[node as usize].iter() {
            let n = n as usize;
            in_degree[n] -= 1;
            if in_degree[n] == 0 {
                queue.push_back(n as i32);
            }
        }
    }

    if i == num_courses as usize {
        top_order
    } else {
        vec![]
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

        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn example_three() {
        let num_courses = 1;
        let prereqs = vec![];
        let result = find_order(num_courses, prereqs);

        assert_eq!(result, vec![0]);
    }
}

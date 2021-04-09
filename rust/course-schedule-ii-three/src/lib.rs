use std::collections::{HashMap, VecDeque};

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(num_courses as usize);
    let mut in_degree: HashMap<i32, i32> = HashMap::with_capacity(num_courses as usize);
    let mut top_order: Vec<i32> = Vec::with_capacity(num_courses as usize);

    for edge in prerequisites {
        let dep = edge[0];
        let prereq = edge[1];
        graph.entry(prereq).or_default().push(dep);
        graph.entry(dep).or_default();
        *in_degree.entry(dep).or_default() += 1;
        in_degree.entry(prereq).or_default();
    }

    println!("{:?}", in_degree);

    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
        .collect();
    let mut i = 0;

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        top_order.push(node);
        i += 1;

        for &n in graph.entry(node).or_default().iter() {
            let deg = in_degree.entry(n).or_default();
            *deg -= 1;

            if *deg == 0 {
                queue.push_back(n);
            }
        }
    }

    if i == num_courses {
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
        let prerequisites = vec![vec![1, 0]];
        let result = find_order(num_courses, prerequisites);

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_two() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let result = find_order(num_courses, prerequisites);

        assert!(vec![vec![0, 1, 2, 3], vec![0, 2, 1, 3]].contains(&result));
    }
}

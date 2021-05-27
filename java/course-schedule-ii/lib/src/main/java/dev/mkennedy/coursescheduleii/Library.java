/*
 * There are a total of numCourses courses you have to take, labeled from 0 to
 * numCourses - 1. You are given an array prerequisites where prerequisites[i] =
 * [ai, bi] indicates that you must take course bi first if you want to take
 * course ai.
 *
 * For example, the pair [0, 1] indicates that to take course 0 you have to
 * first take course 1.
 *
 * Return the ordering of courses you should take to finish all courses. If
 * there are many valid answers, return any of them. If it is impossible to
 * finish all courses, return an empty array.
 */

package dev.mkennedy.coursescheduleii;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.stream.Collectors;

public class Library {
    public static int[] findOrder(int numCourses, int[][] prerequisites) {
        Map<Integer, List<Integer>> graph = new HashMap<>(numCourses);
        Map<Integer, Integer> inDegree = new HashMap<>(numCourses);

        for (int i = 0; i < numCourses; ++i) {
            graph.put(i, new ArrayList<>());
            inDegree.put(i, 0);
        }

        for (int[] edge : prerequisites) {
            int pre = edge[1];
            int course = edge[0];
            graph.get(pre).add(course);
            inDegree.compute(course, (k, v) -> ++v);
        }

        Queue<Integer> queue = inDegree.entrySet().stream()
                .filter(entry -> entry.getValue().equals(0))
                .map(entry -> entry.getKey())
                .collect(Collectors.toCollection(ArrayDeque::new));
        List<Integer> order = new ArrayList<>(numCourses);
        int count = 0;

        while (!queue.isEmpty()) {
            int course = queue.poll();
            order.add(course);
            ++count;

            for (int dep : graph.get(course)) {
                inDegree.compute(dep, (k, v) -> --v);

                if (inDegree.get(dep) == 0) {
                    queue.add(dep);
                }
            }
        }

        if (count == numCourses) {
            return order.stream().mapToInt(i -> i).toArray();
        } else {
            return new int[] {};
        }
    }
}

/**
 * <h1>Course Schedule II</h1>
 * <p>
 * There are total of <code>numCourses</code> courses you have to take, labeled
 * from <code>0</code> to <code>numCourses - 1</code>. You are given an array
 * <code>prerequisites</code> where <code>prerequisites[i] = [ai, bi]</code>
 * indicates that you must take course <code>bi</code> first if you want to take
 * course <code>ai</code>.
 * </p>
 *
 * <ul>
 * <li>For example, the pair <code>[0, 1]</code>, indicates that to take course
 * <code>0</code> you have to first take course <code>1</code>.</li>
 * </ul>
 *
 * <p>
 * Return <em>the ordering of courses you should take to finish all
 * courses</em>. If There are many valid answers, return <b>any</b> of them. If
 * it is impossible to finish all courses, return <b>an empty array</b>.
 * </p>
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
            int src = edge[1];
            int dest = edge[0];
            graph.get(src).add(dest);
            inDegree.compute(dest, (k, v) -> ++v);
        }

        Queue<Integer> queue = inDegree.entrySet().stream()
                .filter(e -> e.getValue().equals(0)).map(e -> e.getKey())
                .collect(Collectors.toCollection(ArrayDeque::new));
        List<Integer> order = new ArrayList<>(numCourses);
        int count = 0;

        while (!queue.isEmpty()) {
            int course = queue.poll();
            order.add(course);
            ++count;

            for (int dep : graph.get(course)) {
                inDegree.compute(dep, (k, v) -> --v);
                int deg = inDegree.get(dep);

                if (deg == 0) {
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

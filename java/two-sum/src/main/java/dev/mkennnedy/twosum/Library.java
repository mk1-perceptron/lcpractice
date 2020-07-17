/*
 * Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a
 * specific target.
 * You may assume that each input would have exactly one solution, and you may not use the same
 * element twice.
 */

package dev.mkennnedy.twosum;

import java.util.HashMap;
import java.util.Map;

public class Library {
    public static int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> remainders = new HashMap<>(nums.length);

        for (int i = 0; i < nums.length; ++i) {
            int val = nums[i];

            if (remainders.containsKey(val)) {
                return new int[]{remainders.get(val), i};
            }

            remainders.put(target - val, i);
        }

        return new int[]{};
    }
}

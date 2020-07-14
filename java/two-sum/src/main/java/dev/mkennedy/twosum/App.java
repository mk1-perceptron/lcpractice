package dev.mkennedy.twosum;

import java.util.HashMap;
import java.util.Map;

public class App {
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

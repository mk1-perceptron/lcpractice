/*
 * Remove Duplicates from Sorted Array
 *
 * Given a sorted array nums, remove the duplicates in-place such that each element appear only
 * once and return the new length.
 * Do not allocate extra space for another array, you must do this by modifying the input array
 * in-place with O(1) extra memory.
 */
package dev.mkennedy.removeduplicatesfromsortedarray;

public class Library {
    public static int removeDuplicates(int[] nums) {
        if (nums.length <= 1) {
            return nums.length;
        }

        int end = 1;
        int prev = nums[0];

        for (int i = 0; i < nums.length; ++i) {
            int cur = nums[i];
            if (cur != prev) {
                nums[end] = cur;
                end += 1;
            }

            prev = cur;
        }

        return end;
    }
}

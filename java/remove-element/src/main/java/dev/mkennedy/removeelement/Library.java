/*
 * Remove Element
 *
 * Given an array nums and a value val, remove all instances of that value in-place and return the
 * new length.
 * Do not allocate extra space for another array, you must do this by modifying the input array
 * in-place with O(1) extra memory.
 * The order of elements can be changed. It doesn't matter what your leave beyond the new length.
 */

package dev.mkennedy.removeelement;

public class Library {
    public static int removeElement(int[] nums, int val) {
        int endIndex = 0;

        for (int i = 0; i < nums.length; ++i) {
            if (nums[i] != val) {
                nums[endIndex] = nums[i];
                endIndex += 1;
            }
        }

        return endIndex;
    }
}

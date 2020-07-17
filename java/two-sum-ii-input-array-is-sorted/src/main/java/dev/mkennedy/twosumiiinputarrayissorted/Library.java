/*
 * Two Sum II - Input array is sorted
 *
 * Given an array of integers that is already sorted in ascending order, find two numbers such that
 * they add up to a specific target number.
 * The function twoSum should return indices of the two numbers such that they add up to the
 * target, where idnex1 must be less than index2.
 *
 * Note:
 * - Your returned answers (both index1 and index2) are not zero-base.
 * - Your may assume that each input would have exactly one solution and you may not use the same
 *   element twice.
 */
package dev.mkennedy.twosumiiinputarrayissorted;

public class Library {
    public static int[] twoSum(int[] numbers, int target) {
        int left = 0;
        int right = numbers.length - 1;

        while (left < right) {
            int sum = numbers[left] + numbers[right];

            if (sum < target) {
                left += 1;
            } else if (sum > target) {
                right -= 1;
            } else {
                return new int[]{left + 1, right + 1};
            }
        }

        return new int[]{};
    }
}

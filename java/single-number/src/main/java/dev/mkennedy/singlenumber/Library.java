/*
 * Single Number
 *
 * Given a non-empty array of integers, every element appears twice except for one. Find that
 * single one.
 *
 * Note:
 * Your alogrithm should have a linear runtime complexity. Could you implement it without using
 * extra memory?
 */

package dev.mkennedy.singlenumber;

import java.util.Arrays;

public class Library {
    public static int singleNumber(int[] nums) {
        return Arrays.stream(nums).reduce(0, (acc, num) -> acc ^ num);
    }
}

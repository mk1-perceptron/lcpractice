/*
 * Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D, and M.
 *
 * Symbol       Value
 * I            1
 * V            5
 * X            10
 * L            50
 * C            100
 * D            500
 * M            1000
 *
 * For example, two is written as II in Roman numeral, just two one's added together. Twelve is
 * written as, XII, which is simple X + II. The number twenty seven is written as XXVII, which is
 * XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral
 * for four is not IIII. Instead, the number four is written as IV. Because the one is before the
 * five we subtract it make four. The same principle applies to the number nine, which is written
 * as IX. There are six instances where subtraction is used:
 * - I can be placed before V (5) and X (10) to make 4 and 9.
 * - X can be placed before L (50) and C (100) to make 40 and 90.
 * - C can be placed before D (500) and M (1000) to make 400 and 900.
 * Given a roman numeral, conver it to an integer. Input is guaranteed to be within the range from
 * 1 to 3999.
 */

package dev.mkennedy.romantointeger;

import java.util.Map;
import static java.util.Map.entry;

public class Library {

    public static Map<Character, Integer> values = Map.ofEntries(
        entry('I', 1),
        entry('V', 5),
        entry('X', 10),
        entry('L', 50),
        entry('C', 100),
        entry('D', 500),
        entry('M', 1000)
    );

    public static int romanToInt(String s) {
        int total = 0;
        int prevVal = 0;

        for (int i = s.length() - 1; i >= 0; --i) {
            int val = values.get(s.charAt(i));
            int mult = (prevVal > val) ? -1 : 1;
            total += mult * val;
            prevVal = val;
        }

        return total;
    }
}

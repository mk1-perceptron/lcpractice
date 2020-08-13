/*
 * Valid parentheses
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[', and ']', determine if
 * the input string is valid.
 *
 * An input string is valid if:
 * 1. Open brackets must be closed by the same type of brackets.
 * 2. Open brackets must be closed in the correct order.
 *
 * Note that an empty string is considered valid.
 */

package dev.mkennedy.validparentheses;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.Map;
import static java.util.Map.entry;

public class Library {
    static Map<Character, Character> closeToOpenMap = Map.ofEntries(
        entry(')', '('),
        entry('}', '{'),
        entry(']', '[')
    );

    public static boolean isValid(String s) {
        Deque<Character> stack = new ArrayDeque<>(s.length() / 2);

        for (int i = 0; i < s.length(); ++i) {
            char cur = s.charAt(i);
            Character open = closeToOpenMap.get(cur);

            if (open == null) {
                stack.push(cur);
                continue;
            }

            Character top = stack.poll();

            if (top == null || !open.equals(top)) {
                return false;
            }
         }

        return stack.isEmpty();
    }
}

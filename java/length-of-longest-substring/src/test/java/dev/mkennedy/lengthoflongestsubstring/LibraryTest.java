package dev.mkennedy.lengthoflongestsubstring;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class LibraryTest {
    @Test
    void exampleOne() {
        String input = "abcabcbb";
        int result = Library.lengthOfLongestSubstring(input);

        assertEquals(3, result);
    }

    @Test
    void exampleTwo() {
        String input = "bbbbb";
        int result = Library.lengthOfLongestSubstring(input);

        assertEquals(1, result);
    }

    @Test
    void exampleThree() {
        String input = "pwwkew";
        int result = Library.lengthOfLongestSubstring(input);

        assertEquals(3, result);
    }
}

cmake_minimum_required(VERSION 4.0)
project(leetcode_cpp)

set(CMAKE_CXX_STANDARD 17)

add_executable(leetcode_cpp main.cpp
        "leetcode/editor/cn/[1]Two Sum.cpp"
        "leetcode/editor/cn/[9]Palindrome Number.cpp"
        "leetcode/editor/cn/[13]Roman to Integer.cpp"
        "leetcode/editor/cn/[704]Binary Search.cpp"
        "leetcode/editor/cn/[35]Search Insert Position.cpp"
        "leetcode/editor/cn/[34]Find First and Last Position of Element in Sorted Array.cpp"
        "leetcode/editor/cn/[69]Sqrt(x).cpp"
        "leetcode/editor/cn/[367]Valid Perfect Square.cpp"
        "leetcode/editor/cn/[27]Remove Element.cpp"
        "leetcode/editor/cn/[26]Remove Duplicates from Sorted Array.cpp"
        "leetcode/editor/cn/[283]Move Zeroes.cpp"
        "leetcode/editor/cn/[844]Backspace String Compare.cpp"
        "leetcode/editor/cn/[977]Squares of a Sorted Array.cpp"
        "leetcode/editor/cn/[209]Minimum Size Subarray Sum.cpp"
        "leetcode/editor/cn/[904]Fruit Into Baskets.cpp"
)

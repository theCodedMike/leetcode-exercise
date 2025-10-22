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
)

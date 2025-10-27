namespace leetcode_csharp;

static class LeetCodeProgram
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");

        int[] nums = new[] { 0, 2, 4, 6, 8, 10 };
        Console.WriteLine(Array.BinarySearch(nums, 1));
    }
}
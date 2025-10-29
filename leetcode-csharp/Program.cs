namespace leetcode_csharp;

static class LeetCodeProgram
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");

        int[] fruits = [1, 2, 1];
        Console.WriteLine(SlidingWindow(fruits));
    }
    
    
    static int SlidingWindow(int[] fruits)
    {
        int res = 0;
        int left = 0;
        Dictionary<int, int> map = new Dictionary<int, int>();

        for (int right = 0; right < fruits.Length; right++)
        {
            map[fruits[right]]++;
            while (map.Count > 2)
            {
                map[fruits[left]]--;
                if (map[fruits[left]] == 0)
                    map.Remove(fruits[left]);
                ++left;
            }

            res = Math.Max(res, right - left + 1);
        }
        
        return res;
    }
}
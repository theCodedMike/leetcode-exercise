package main

import "fmt"

func main() {
	fmt.Println("hello leetcode-go!")

	nums := []int{-4, -1, 0, 3, 10}
	fmt.Println(twoPointers1(nums))
}

func twoPointers1(nums []int) []int {
	res, idx := make([]int, len(nums)), len(nums)-1
	left, right := 0, len(nums)-1

	for left <= right {
		left_square := nums[left] * nums[left]
		right_square := nums[right] * nums[right]
		if left_square < right_square {
			res[idx] = right_square
			right--
			idx--
		} else if left_square > right_square {
			res[idx] = left_square
			left++
			idx--
		} else {
			res[idx] = right_square
			if left != right {
				res[idx-1] = left_square
			}
			left++
			right--
			idx -= 2
		}
	}

	return res
}

package src

import (
	"reflect"
	"testing"
)

// 在终端执行：
//
// go test ./leetcode/editor/cn

func TestTwoSum1(t *testing.T) {
	var nums = []int{2, 7, 11, 15}
	target := 9
	var want = []int{0, 1}

	res := twoSum(nums, target)
	if !reflect.DeepEqual(res, want) {
		t.Errorf("res(%v) != want(%v)", res, want)
	}
}

func TestTwoSum2(t *testing.T) {
	var nums = []int{3, 2, 4}
	target := 6
	var want = []int{1, 2}

	res := twoSum(nums, target)
	if !reflect.DeepEqual(res, want) {
		t.Errorf("res(%v) != want(%v)", res, want)
	}
}

func TestTwoSum3(t *testing.T) {
	var nums = []int{3, 3}
	target := 6
	var want = []int{0, 1}

	res := twoSum(nums, target)
	if !reflect.DeepEqual(res, want) {
		t.Errorf("res(%v) != want(%v)", res, want)
	}
}

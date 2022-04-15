package dynamic

/**
描述
输入一个长度为n的整型数组array，数组中的一个或连续多个整数组成一个子数组，子数组最小长度为1。求所有子数组的和的最大值。

要求:时间复杂度为 O(n)，空间复杂度为 O(n)
进阶:时间复杂度为 O(n)，空间复杂度为 O(1)
----------------------------------------
示例1
输入：
[1,-2,3,10,-4,7,2,-5]
返回值：
18

说明：
经分析可知，输入数组的子数组[3,10,-4,7,2]可以求得最大和为18
-----------------------
示例2
输入：
[2]
返回值：
2
------------------------
示例3
输入：
[-10]
返回值：
-10
*/

/**
在有时间复杂度和空间复杂度的限制下：
* 解题思路:
该题目符合动态规划的特征，其最优子结构有如下性质
一 如果空间复杂度为O（n）
设result[n]，result[n]代表前n个数的最大和
最优子结构为：
result[n] = max(0, n + result[n-1])
*/

func sumOfArrayn(arrays int[]) int {
	result := make([]int, len(arrays))
	biggest := 0
	for i := 0; i < len(arrays); i++ {
		if i == 0 {
			if arrays[i] > 0 {
				result[i] = arrays[i]
			}
			continue
		}
		if arrays[i]+result[i-1] > 0 {
			result[i] = arrays[i] + result[i-1]
		}
		if result[i] > biggest {
			biggest = result[i]
		}
	}
	return biggest
}

/**
二 如果空间复杂度为O（1）
设最大和为biggest，当前和为sum
1. 如果a+b<0 则 a+b = 0
2. 如果a+b > biggest，则biggest = a + b
3. 如果a+b >0 则贪心的认为继续向下加遇见正数时得到的和，一定比正数本身更大

警告： 数组可能全为负数
*/
func sumOfArray1(arrays int[]) int {
	biggest := arrays[0]
	sum := arrays[0]
	for i := 1; i < len(arrays); i++ {
		if sum > 0 && sum+arrays[i] > 0 {
			sum = sum + arrays[i]
		} else {
			sum = arrays[i]
		}
		if sum > biggest {
			biggest = sum
		}
	}
	return biggest
}

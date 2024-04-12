package sort

/**
给定一个长度为 n 的可能有重复值的数组，找出其中不去重的最小的 k 个数。
例如数组元素是4,5,1,6,2,7,3,8这8个数字，则最小的4个数字是1,2,3,4(任意顺序皆可)。
数据范围：0<= k, n≤10000，数组中每个数的大小0≤val≤1000
要求：空间复杂度 O(n) ，时间复杂度 O(nlogn)

示例1
输入：
[4,5,1,6,2,7,3,8],4
返回值：
[1,2,3,4]
说明：
返回最小的4个数即可，返回[1,3,2,4]也可以
---------------------
示例2
输入：
[1],0
复制
[]
-------------------
示例3
输入：
[0,1,2,1,2],3
返回值：
[0,1,1]
*/

/**
思路说明：
从示例的几个数据来说，本题目的第一眼的思路在于找到最小的几个元素。即这些元素可能相等。
所以本题适合使用插入排序，即提前构建好长度为k的数组，按照自然序排序。
但是因为要求时间复杂度为nlogn，所以本题使用快排，先排序然后求出topk。
*/
/**
 * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
 *
 * @param input int整型一维数组
 * @param k int整型
 * @return int整型一维数组
 */
func GetLeastNumbers_Solution(input []int, k int) []int {
	// write code here
	quickSort(input, 0, len(input)-1)
	return input[:k]

}

func quickSort(input []int, first, last int) {
	low := first
	high := last
	key := input[low]
	for low < high {
		// 右边都比key大
		for low < high && input[high] > key {
			high--
		}
		if low < high {
			input[low] = input[high]
			low++
		}

		// 左边都比key小
		for low < high && input[low] < key {
			low++
		}
		if low < high {
			input[high] = input[low]
			high--
		}
	}
	input[low] = key
	quickSort(input, first, low-1)
	quickSort(input, low, last)
}

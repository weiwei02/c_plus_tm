package dynamic

/**
最长回文子串
描述
对于长度为n的一个字符串A（仅包含数字，大小写英文字母），请设计一个高效算法，计算其中最长回文子串的长度。


数据范围： 1≤n≤1000
要求：空间复杂度 O(1)，时间复杂度 O(n^2)
进阶:  空间复杂度 O(n)，时间复杂度 O(n)
--------------------------
示例1
输入：
"ababc"
返回值：
3
说明：
最长的回文子串为"aba"与"bab"，长度都为3
------------------------------
示例2
输入：
"abbba"
返回值：
5
-------------------
示例3
输入：
"b"
返回值：
1
--------------
解题思路：
对于str[i]回文字符串，有str[i-j] = str[i+j]的特征，
如果回文字符串str[i+j]如果符合规则，那么它的子串star[i+j-j]也一定是回文字符串，所以回文字符串符合重复子问题的特征
对于回文字符串
str[i-j] -> str[i+j]，如果 str[i-j] = str[i+j]则当前最大回文字符串值加1

另外根据回文字符串的性质，一个字符串回文子串的长度>=字符串长度除以2，此时该子串一定是最长回文字符串，不需要再进行更多搜索。
为了尽快找到这个最长子串，我们可以借助二分查找来加快搜索速度。

 * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
 *
 *
 * @param A string字符串
 * @return int整型
*/
var longestPalindrome = 0

func getLongestPalindrome(A string) int {
	if len(A) < 2 {
		return 0
	}
	// write code here
	dp := make([]int, len(A))
	searchChildPalindrome(dp, A, len(A)/2)
	return longestPalindrome
}

// 递归快速查找子串
func searchChildPalindrome(dp []int, str string, mid int) {
	if mid < 2 || mid > len(str)-2 {
		return
	}
	
	dp[mid] = 0
	for i := 0; i <= mid; i++ {
		index := mid + i
		dp[index] = 0
		for j := i; j < mid; j++ {
			if index+j >= len(str) || mid-j < 0 {
				break
			}
			if str[index-j] == str[index+j] {
				lens := dp[index-j+1] + 1
				dp[index-j] = lens
				dp[index+j] = lens
				if longestPalindrome < lens {
					longestPalindrome = lens
				}
			} else {
				break
			}
		}
	}
}

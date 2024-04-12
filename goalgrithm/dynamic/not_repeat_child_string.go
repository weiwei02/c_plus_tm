package dynamic

/**
题目描述：
输入一个字符串（只包含 a~z 的字符），求其最长不含重复字符的子字符串的长度。
例如对于 arabcacfr，最长不含重复字符的子字符串为 acfr，长度为 4。

解题思路：
要求最长子串问题，即重复字符出现的最大间隔。
设当前子串长度为l，最优解存在下面两个选择中
1. 假如对字符 a，上次出现的位置是i，当前出现的位置是j，如果j-i大于l，则说明在l+1的范围内还未出现重复子串。否则应进入2
2. 当前子串长度应该是j-i，即 [i +1, j]是当前的子串。

*/
//
//func longest_no_repeat_child_string(str string) int {
//	array := make([]int, 26)
//	for i := 0; i < len(array); i++ {
//		array[i] = -1
//	}
//
//	// 当前子串长度
//	curLen := 0
//	// 最长子串长度
//	maxLen := 0
//	for i := 0; i < len(str); i++ {
//
//	}
//}

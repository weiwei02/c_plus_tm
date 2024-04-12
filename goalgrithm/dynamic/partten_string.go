package dynamic

/*
描述
请实现一个函数用来匹配包括'.'和'*'的正则表达式。
1.模式中的字符'.'表示任意一个字符
2.模式中的字符'*'表示它前面的字符可以出现任意次（包含0次）。
在本题中，匹配是指字符串的所有字符匹配整个模式。例如，字符串"aaa"与模式"a.a"和"ab*ac*a"匹配，但是与"aa.a"和"ab*a"均不匹配

数据范围:
1.str 只包含从 a-z 的小写字母。
2.pattern 只包含从 a-z 的小写字母以及字符 . 和 *，无连续的 '*'。
3. 0 <= str.length <= 26
4. 0 <= pattern.length <= 26

示例1
输入：
"aaa","a*a"
返回值：
true
说明：
中间的*可以出现任意次的a，所以可以出现1次a，能匹配上
-------------------------
示例2
输入：
"aad","c*a*d"
返回值：
true
说明：
因为这里 c 为 0 个，a被重复一次， * 表示零个或多个a。因此可以匹配字符串 "aad"。
--------------------------
示例3
输入：
"a",".*"
返回值：
true
说明：
".*" 表示可匹配零个或多个（'*'）任意字符（'.'）
-------------------------------
示例4
输入：
"aaab","a*a*a*c"
返回值：
false
*/

/**
 * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
 *
 *
 * @param str string字符串
 * @param pattern string字符串
 * @return bool布尔型
 */
func match(str string, pattern string) bool {
	// write code here
	return false
}

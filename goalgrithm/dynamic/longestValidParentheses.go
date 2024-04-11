package dynamic

/**
描述
给出一个长度为 n 的，仅包含字符 '(' 和 ')' 的字符串，计算最长的格式正确的括号子串的长度。

例1: 对于字符串 "(()" 来说，最长的格式正确的子串是 "()" ，长度为 2 .
例2：对于字符串 ")()())" , 来说, 最长的格式正确的子串是 "()()" ，长度为 4 .

字符串长度：0 <= n <= 1*10^5
要求时间复杂度 O(n) ,空间复杂度 O(n).
示例1
输入：
"(()"
返回值：
2
---------------------
示例2
输入：
"(())"
返回值：
4
*/
/**
 * 算法思路：
一个格式正确的括号条件：
1. 不以右括号开头
2. 括号是成对出现的

因为会允许括号嵌套，所以父括号如果要合法的条件时子括号必须先合法。
因为题目声明了字符串只包含()连个符号，所以最小的子问题如果想合法，那么必定有()相邻的字符串。
如果把所有合法的字符串清空，上面的步骤是一个重复子问题，接着看父级字符串是否合法

 * @param s string字符串
 * @return int整型
*/
const (
	bl = byte('(')
	br = byte(')')
	b0 = byte('0')
)

func longestValidParentheses(s string) int {
	if len(s) < 2 {
		return 0
	}
	// write code here
	maxLen := 0
	bytes := []byte(s)
	for i := 0; i < len(s); i++ {
		if bytes[i] == bl {
			first := i
			last := i
			j := i + 1
			for j < len(s) && bytes[j] == b0 {
				j++
			}
			if j < len(s) && bytes[j] == br {
				bytes[i] = b0
				bytes[j] = b0
				last = j
				for last < len(s) && bytes[last] == b0 {
					last++
				}
			}

			// 回退到非0的字符串位置，看看父串是否合法
			if last-first != 0 {
				for k := i; k > 0 && bytes[k] == b0; k-- {
					if bytes[k-1] == bl {
						i = k - 2
						break
					} else if bytes[k-1] == br {
						break
					}
					first--
				}
				if maxLen < last-first {
					maxLen = last - first
				}
			}

		}
	}
	return maxLen
}

package dynamic

/**
 * 矩形覆盖
 * 题目描述
 * 我们可以用 2*1 的小矩形横着或者竖着去覆盖更大的矩形。
 * 请问用 n 个 2*1 的小矩形无重叠地覆盖一个 2*n 的大矩形，总共有多少种方法？
 *
 * 地址：https://www.nowcoder.com/practice/72a5a919508a4251859fb2cfb987a0e6?tpId=13&tqId=11163&tPage=1&rp=1&ru=/ta/coding-interviews&qru=/ta/coding-interviews/question-ranking&from=cyc_github
 */

func rectCover(n int) int {
	if n <= 3 {
		return n
	}

	a := 1
	b := 2
	var c int
	for i := 3; i <= n; i++ {
		c = a + b
		a = b
		b = c
	}
	return c
}

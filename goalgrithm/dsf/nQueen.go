package dsf

/**
N 皇后问题是指在 n * n 的棋盘上要摆 n 个皇后，
要求：任何两个皇后不同行，不同列也不在同一条斜线上，
求给一个整数 n ，返回 n 皇后的摆法数。

数据范围: 1≤n≤9
要求：空间复杂度 O(1) ，时间复杂度 O(n!)

例如当输入4时，对应的返回值为2，

示例1
输入：
1
返回值：
1
-------------------
示例2
输入：
8
返回值：
92


解题思路：
n=1， 结果 1
n=2， 结果 0
n=3,  结果 0
n=4,  结果 2

设矩阵arr[i][j]为皇后的位置，
则 arr[i][j], arr[k][j],arr[i+k][j+k],arr[i-k],[j-k]任意位置都不能有元素
如果n个皇后可以在满足以上矩阵的位置放下，则认为条件满足
 *
*/

/**
 *
 * @param n int整型 the n
 * @return int整型
 */
func Nqueen(n int) int {
	// write code here

	return 0
}

//allPath := make()

func positions(n int, arr []int) []int {
	if len(arr) == n*2 {

	}
	for i := 0; i <= n; i++ {
		for j := 0; j <= n; j++ {
			// 判断是否还有位置可放
			for k := 0; k < len(arr); k += 2 {
				// 横竖左右皆可
				if i != arr[k] && j != arr[k+1] && j-i != arr[k+1]-arr[k] {
					newPath := append(arr, i, j)
					positions(n, newPath)
				}
			}
		}
	}
	return []int{0}
}

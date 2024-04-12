package dsf

/**
描述
请设计一个函数，用来判断在一个n乘m的矩阵中是否存在一条包含某长度为len的字符串所有字符的路径。路径可以从矩阵中的任意一个格子开始，
每一步可以在矩阵中向左，向右，向上，向下移动一个格子。如果一条路径经过了矩阵中的某一个格子，则该路径不能再进入该格子。
例如
[a, b, c, e]
[s, f, c, s]
[a, d, e, e]
  矩阵中包含一条字符串"bcced"的路径，但是矩阵中不包含"abcb"路径，
因为字符串的第一个字符b占据了矩阵中的第一行第二个格子之后，路径不能再次进入该格子。
数据范围：0 <= n,m <= 20,1 <= len <= 25
示例1
输入：
[[a,b,c,e],[s,f,c,s],[a,d,e,e]],"abcced"
返回值：
true

示例2
输入：
[[a,b,c,e],[s,f,c,s],[a,d,e,e]],"abcb"
返回值：
false

*/

/**
解题思路：
该题适合使用回溯法
求解问题时使用贪心策略
设matrix[i][j]代表矩阵任意一点的元素，i-1代表向左移动，i+1代表向右移动。
设path[]代表本次遍历已经走过的路径
word[k]代表要检索的字符。
1. matrix[i][j]上下左右四个方向的某个元素，如matrix[i+1][j]等于word[k]，且这个元素的位置不在path中，则进入2。否则返回false
2. 把当前矩阵指针设置为matrix[i+1][j]，如果word[k]是word的最后一个元素，则返回true。否则word向后移动一个元素，纪录path，并跳转到1.
*/

type path struct {
	i int
	j int
	k int
}

/**
 * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
 *
 *
 * @param matrix char字符型二维数组
 * @param word string字符串
 * @return bool布尔型
 */
func hasPath(matrix [][]byte, word string) bool {
	// write code here
	// 遍历矩阵，如果存在任意一条满足途径的路径，则直接返回true
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[i]); j++ {
			if isHasPath(matrix, []path{{i, j, 0}}, word) {
				return true
			}
		}
	}
	return false
}

// 递归搜索最佳路径
func isHasPath(matrix [][]byte, paths []path, word string) bool {
	current := paths[len(paths)-1]
	// 如果word已经被搜索到最后一个字符，代表着矩阵中已经找到的目标字符串
	if current.k == len(word) {
		return true
	}
	// 矩阵已经越界
	if current.i < 0 ||
		current.i == len(matrix) ||
		current.j < 0 ||
		current.j == len(matrix[0]) {
		return false
	}

	// 元素不匹配
	if matrix[current.i][current.j] != word[current.k] {
		return false
	}

	// 重复索引
	for i := len(paths) - 2; i >= 0; i-- {
		if current.i == paths[i].i && current.j == paths[i].j {
			return false
		}
	}

	// 向左检索
	if isHasPath(matrix, append(paths, path{current.i - 1, current.j, current.k + 1}), word) {
		return true
	}
	// 向右检索
	if isHasPath(matrix, append(paths, path{current.i + 1, current.j, current.k + 1}), word) {
		return true
	}
	// 向上检索
	if isHasPath(matrix, append(paths, path{current.i, current.j - 1, current.k + 1}), word) {
		return true
	}
	return isHasPath(matrix, append(paths, path{current.i, current.j + 1, current.k + 1}), word)
}

package tree

import (
	"strconv"
	"strings"
)

/**
请实现两个函数，分别用来序列化和反序列化二叉树，不对序列化之后的字符串进行约束，
但要求能够根据序列化之后的字符串重新构造出一棵与原二叉树相同的树。

二叉树的序列化(Serialize)是指：把一棵二叉树按照某种遍历方式的结果以某种格式保存为字符串，
从而使得内存中建立起来的二叉树可以持久保存。序列化可以基于先序、中序、后序、层序的二叉树等
遍历方式来进行修改，序列化的结果是一个字符串，序列化时通过 某种符号表示空节点（#）

二叉树的反序列化(Deserialize)是指：根据某种遍历顺序得到的序列化字符串结果str，重构二叉树。

序列化(即用函数Serialize转化)如上的二叉树转为"{1,2,3,#,#,6,7}"，
再能够调用反序列化(Deserialize)将"{1,2,3,#,#,6,7}"构造成如上的二叉树。

当然你也可以根据满二叉树结点位置的标号规律来序列化，还可以根据先序遍历和中序遍历的结果来序列化。
不对序列化之后的字符串进行约束，所以欢迎各种奇思妙想。

数据范围：节点数 n≤100，树上每个节点的值满足 0≤val≤150
要求：序列化和反序列化都是空间复杂度 O(n)，时间复杂度 O(n)


思想：使用广度优先遍历算法来构造二叉树
序列化方法：
1. 借助队列数据结构，把根节点存到队列中
2. 从队列获取元素，打印该元素，进入3
3. 把其左右子节点分别存入到队列中。如果子节点为空，则存入nil。

[1,2,-1,-1, 3,6,7]

反序列化方法：
-------------------------
该方法不合适
(这个性质仅适用于满二叉树)根据广度优先遍历二叉树的性质：
* n 的左右子节点分别在 2n 和 2n+1的位置上
所以算法思想如下：
1. 将字符串根据分割符做切割，得到字符串数组a
2. 构造节点 a[n]，可选的作为上一个节点的左子树或右子树
3. 如果有访问a[n]的左子树，并进入1
4. 如果有访问a[n]的右子树，并进入1

------------------
1. 将字符串根据分割符做切割，得到字符串数组a
2. 设置一个队列，并把a[n]放入队列
3. 如果有访问a[n]的左子树，并进入1
4. 如果有访问a[n]的右子树，并进入1
*/

const (
	split = ","
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
 *
 * @param root TreeNode类
 * @return TreeNode类
 */
func Serialize(root *TreeNode) string {
	str := &strings.Builder{}
	sv(root, str)
	if str.Len() > 1 {
		return str.String()[:str.Len()-1]
	}
	return str.String()
}

func sv(root *TreeNode, str *strings.Builder) {
	if root == nil {
		str.WriteString("-1")
	} else {
		str.WriteString(strconv.Itoa(root.Val))
	}
	str.WriteString(split)

	if root != nil {
		sv(root.Left, str)
		sv(root.Right, str)
	}
}

/**
深度优先反遍历
*/
func Deserialize(s string) *TreeNode {
	// write code here
	return dv(strings.Split(s, split), 0)
}

var maxIndex = 0

func dv(arr []string, index int) *TreeNode {
	maxIndex = index
	if index >= len(arr) {
		return nil
	}
	if arr[index] == "" {
		return nil
	}
	val, err := strconv.Atoi(arr[index])
	if err != nil {
		return nil
	}
	if val == -1 {
		return nil
	}
	node := &TreeNode{Val: val}

	// 继续遍历左子树
	node.Left = dv(arr, index+1)
	node.Right = dv(arr, maxIndex+1)
	return node
}

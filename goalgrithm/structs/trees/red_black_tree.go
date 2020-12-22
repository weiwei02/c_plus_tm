package trees

import (
	"fmt"
)

type nodeColor uint8
type Data int64

const (
	RED_COLOR   = 0
	BLACK_COLOR = iota
)

func (c nodeColor) color() string {
	switch c {
	case 0:
		return "RED"
	case 1:
		return "BLACK"
	default:
		return fmt.Sprintf("Unknown color %d", c)
	}
}

/**
 * 树节点
 */
type Node struct {
	value     Data
	color     nodeColor
	leftTree  *Node
	rightTree *Node
	parent    *Node
}

func newNode() *Node {
	return &Node{0, RED_COLOR, nil, nil, nil}
}

func (n *Node) grandParent() *Node {
	if n.parent == nil {
		return nil
	}
	return n.parent.parent
}

func (n *Node) uncle() *Node {
	if n.grandParent() == nil {
		return nil
	}
	if n.parent == n.grandParent().rightTree {
		return n.grandParent().leftTree
	}
	return n.grandParent().rightTree
}

/**
 * 获取兄弟节点
 *
 */
func (n *Node) sibling() *Node {
	if n.parent.leftTree == n {
		return n.parent.rightTree
	}
	return n.parent.leftTree
}

/**
 * 红黑树主数据结构定义
 *
 */
type RedBlackTree struct {
	root *Node
}

/**
 * 右旋
 */
func (*RedBlackTree) rotateRight(n *Node) {

}

/**
 * 左旋
 */
	//          |                                  |
	//          X                                  Y
	//         / \         left rotate            / \
	//        α  Y       ------------->         X   γ
	//           / \                            / \
	//          β  γ                            α  β
func (*RedBlackTree) rotateLeft(n *Node) {

}

/**
 * 变色
 */
func (*RedBlackTree) inorder(n *Node) {

}

/**
 * 获取最小子节点
 */
func getSmallestChild(n *Node) *Node {
	if n.leftTree == nil {
		return n
	}
	return getSmallestChild(n.leftTree)
}

/**
 * 删除子节点
 */
func (tree *RedBlackTree) deleteChild(n *Node, data Data) bool {
	if n.value > data {
		if n.leftTree == nil {
			return false
		}
		return tree.deleteChild(n.leftTree, data)
	} else if n.value < data {
		if n.rightTree == nil {
			return false
		}
		return tree.deleteChild(n.rightTree, data)
	} else if n.value == data {
		if n.rightTree == nil {
			tree.deleteOneChild(n)
			return true
		}
		smallest := getSmallestChild(n)
		temp := n.value
		n.value = smallest.value
		smallest.value = temp
		tree.deleteOneChild(smallest)
		return true
	} else {
		return false
	}
}

/**删除一个节点
 *
 */
func (tree *RedBlackTree) deleteOneChild(n *Node) {
	child := func() *Node {
		if n.leftTree == nil {
			return n.rightTree
		}
		return n.leftTree
	}()
	// 当前节点就是根节点
	if n.parent == nil && n.leftTree == nil && n.rightTree == nil {
		tree.root = nil
		return
	}

	if n.parent == nil {
		child.parent = nil
		tree.root = child
		tree.root.color = BLACK_COLOR
		return
	}

	if n.parent.leftTree == n {
		n.parent.leftTree = child
	} else {
		n.parent.rightTree = child
	}
	child.parent = n.parent

	if n.color == BLACK_COLOR {
		if child.color == RED_COLOR {
			child.color = BLACK_COLOR
		} else {
			tree.deleteCase(child)
		}
	}
}

/**
 * 具体删除某个单元
 */
func (tree *RedBlackTree) deleteCase(n *Node) {
	// 场景1，没有父节点
	if n.parent == nil {
		n.color = BLACK_COLOR
		return
	}
	if n.sibling().color == RED_COLOR {
		n.parent.color = RED_COLOR
		n.sibling().color = BLACK_COLOR
		if n == n.parent.leftTree {
			tree.rotateLeft(n.sibling())
		} else {
			tree.rotateRight(n.sibling())
		}
	}
	if n.parent.color == BLACK_COLOR && 
		n.sibling().color == BLACK_COLOR && 
		n.sibling().leftTree.color == BLACK_COLOR && 
		n.sibling().rightTree.color == BLACK_COLOR {
			n.sibling().color = RED_COLOR
			tree.deleteCase(n.parent)
	}else if n.parent.color == RED_COLOR &&
		n.sibling().color == BLACK_COLOR &&
		n.sibling().leftTree.color == BLACK_COLOR &&
		n.sibling().rightTree.color == BLACK_COLOR{
			n.sibling().color = RED_COLOR
			n.parent.color = BLACK_COLOR
	}else{
		if n.sibling().color == BLACK_COLOR {
			if n == n.parent.leftTree && 
				n.sibling().leftTree.color == RED_COLOR &&
				n.sibling().rightTree.color == BLACK_COLOR {
					n.sibling().color = RED_COLOR
					n.sibling().leftTree.color = BLACK_COLOR
					tree.rotateRight(n.sibling().leftTree)
			}else if n.parent.rightTree == n &&
					n.sibling().leftTree.color == BLACK_COLOR &&
					n.sibling().rightTree.color == RED_COLOR {
						n.sibling().color = RED_COLOR
						n.sibling().rightTree.color = BLACK_COLOR
						tree.rotateLeft(n.sibling().rightTree)
					}
		}
		n.sibling().color = n.parent.color
		n.parent.color = BLACK_COLOR
		if n.parent.leftTree == n {
			n.sibling().rightTree.color = RED_COLOR
			tree.rotateLeft(n.sibling())
		}else {
			n.sibling().leftTree.color = BLACK_COLOR
			tree.rotateRight(n.sibling())
		}
	}
}


/**
 * 插入新节点
 * 插入节点默认为红色节点
 * 插入节点要么左子树要么右子树，执行递归插入
*/
func(tree *RedBlackTree) insert(n *Node, data Data){
	if n.value >= data {
		if n.leftTree != nil{
			tree.insert(n.leftTree, data)
		}else{
			temp := newNode()
			temp.value = data
			temp.parent = n
			n.leftTree = temp
		}
	}else{
		if n.rightTree != nil{
			tree.insert(n.rightTree, data)
		} else {
			temp := newNode()
			temp.value = data
			temp.parent = n
			n.rightTree = temp
			tree.insertCase(temp)
		}
	}
}

/**
 * 红黑树数据插入分为5种场景
 * 
 * case 1：要插入的节点是根节点，则把要插入节点颜色转黑，直接插入
 * 
 * case 2: 插入节点父节为黑色，
 * 不违反任何性质，无需做任何修改。
 * 
 * case 3: 插入节点的父节点为红色，父节点为父父节点的左孩子，父父节点的右孩子为黑色，
 * 插入节点为左孩子(或者父节点为父父节点的右孩子，父父节点的左孩子为黑色，插入节点为右孩子)。
 * 这是一种插入节点和父节点在一个方向上的情况(例如父节点为左孩子，插入节点也为左孩子)和情形5相反
 * 父节点 及 父父节点变色，再进行左/右旋转， 具体左还是右看你插入的节点的父节点是左子树还是右子树，图例为左子树。
 * 此处 : 变色 - > 右旋转
 * 
 * case 4: 插入节点父节点为红色，父父节点的左/右孩子为红色
 * 先将父节点和父父节点右孩子变黑，父父节点变红，然后将父节点当做新插入的红色
 * 节点一样递归向上进行平衡红黑树性质操作。 若父节点为根节点直接变父节点为黑色即可.
 * 
 * case 5: 插入节点的父节点为红色，父节点为父父节点的左孩子，父父节点的右孩子为黑色，
 * 插入节点为右孩子(或者父节点为父父节点的右孩子，父父节点的左孩子为黑色，插入节点为左孩子)。
 * 和情形3类比是一种反向情况，这种情况进行两次旋转，先左/右旋转，旋转后变成了情形3，接着按情形3变换即可。
 * 左旋转 -> 变色 -> 右旋转
*/
func (tree *RedBlackTree) insertCase(n *Node){
	// case 1
	if n.parent == nil {
		tree.root = n
		n.color = BLACK_COLOR
		return
	}
	if n.parent.color == RED_COLOR {
		// case 4
		if n.uncle().color == RED_COLOR {
			n.parent.color = BLACK_COLOR
			n.uncle().color = BLACK_COLOR
			n.grandParent().color = RED_COLOR
			tree.insertCase(n.grandParent())
		} else {

		}
	}

}

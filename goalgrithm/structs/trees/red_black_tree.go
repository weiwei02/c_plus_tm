package trees

import (
	"fmt"
)

type nodeColor uint8
type Data int64

const (
	RED_COLOR  = 0
	BLACK_COLOR  = iota
)

func(c nodeColor) color() string{
	switch c {
	case 0: return "RED"
	case 1: return "BLACK"
	default: return fmt.Sprintf("Unknown color %d", c)
	}
}


/**
 * 树节点
*/
type Node struct {
	value Data
	color nodeColor
	leftTree *Node
	rightTree *Node
	parent *Node
}

func newNode() *Node{
	return &Node{ 0, BLACK_COLOR, nil, nil, nil}
}

func (n *Node) grandParent() *Node{
	if n.parent == nil {
		return nil
	}
	return n.parent.parent
}

func (n *Node) uncle() *Node{
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
func (n *Node) sibling() *Node{
	if n.parent.leftTree == n {
		return n.parent.rightTree
	}
	return n.parent.leftTree
}

type RedBlackTree struct {}

/**
 * 右旋
*/
func(*RedBlackTree) rotateRight(n *Node){

}

/**
 * 左旋
*/
func (*RedBlackTree) rotateLeft(n *Node){

}

/**
 * 变色
*/
func (*RedBlackTree) inorder(n *Node){

}

/**
 * 获取最小子节点
*/
func (*RedBlackTree) getSmallestChild(n *Node) *Node{
	if n.leftTree == nil {
		return n
	}
	return getSmallestChild(n.leftTree)
}

/**
 * 删除子节点
*/
func (*RedBlackTree) deleteChild(n *Node, data Data) bool{
	if n.value > data {
		if n.leftTree == nil {
			return false
		}
		return deleteChild(n.leftTree, data)
	}else if n.value < data {
		if n.rightTree == nil {
			return false
		}
		return deleteChild(n.rightTree, data)
	}else if n.value == data {
		if n.rightTree == nil {
			deleteOneChild(n)
			return true
		}
		smallest := getSmallestChild(n)
		temp := n.value
		n.value = smallest.value
		smallest.value = temp
		deleteOneChild(smallest)
		return true
	}else {
		return false
	}
}


/**删除一个节点
 * 
*/
func (*RedBlackTree) deleteOneChild(n *Node){

}
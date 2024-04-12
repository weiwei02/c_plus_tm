package containers

import (
	dll "github.com/emirpasic/gods/lists/doublylinkedlist"
	"github.com/emirpasic/gods/utils"
)

func doublyLinkedlist() {
	list := dll.New()
	list.Add("a")
	list.Add("c", "b", "a", "q")
	list.Sort(utils.StringComparator)

}

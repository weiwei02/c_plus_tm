package sort

// 冒泡排序
func BubbleSort(src []int) {
	size := len(src)
	for i := 0; i < size-1; i++ {
		for j := 0; j < size-i-1; j++ {
			if src[j] > src[j+1] {
				temp := src[j+1]
				src[j+1] = src[j]
				src[j] = temp
			}
		}
	}
}

// 改进版本冒泡排序

func BubbleSortOrderly(src []int) {
	orderly := false
	for i := 0; i < len(src)-1 && !orderly; i++ {
		orderly = true
		for j := 0; j < len(src)-1-i; j++ {
			if src[j] > src[j+1] {
				orderly = false
				temp := src[j+1]
				src[j+1] = src[j]
				src[j] = temp
			}
		}
	}
}

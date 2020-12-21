package sort

// 归并排序：把数据分为两段，从两段中逐个选最小的元素移入新数据段的末尾。可从上到下或从下到上进行。

/*****************
    迭代版
*****************/
//整數或浮點數皆可使用,若要使用物件(class)時必須設定"小於"(<)的運算子功能
func MergeSort(src []int){

}

/**
 * 递归
*/
func MergeSortRecursive(src, dst []int, start, end int){
	if start >= end {
		return
	}
	size := end - start
	mid := (size >> 1) + start
	start1 := start
	end1 := mid
	start2 := mid + 1
	end2 := end
	MergeSortRecursive(src, )
}
package sort

/*
（小数，基准元素，大数）。在区间中随机挑选一个元素作基准，将小于基准的元素放在基准之前，大于基准的元素放在基准之后，再分别对小数区与大数区进行排序。
快速排序思路：
1. 选取第一个数为基准
2. 将比基准小的数交换到前面，比基准大的数交换到后面
3. 对左右区间重复第二步，直到各区间只有一个数
*/
// 快速排序（递归）
func QuickSort(src []int, low, high int){
	if low >= high {
		return
	}
	first := low
	last := high
	key := src[first]

	for first < last {
		// 将比第一个小的移到前面
		for first < last && src[last] >= key {
			last--;
		}
		if first < last {
			src[first] = src[last]
			first++
		}

		// 将比第一个大的移到后面
		for first < last && src[first] <= key {
			first++
		}
		if first < last {
			src[last] = src[first]
			last--
		}
	}
	// 基准位置
	src[first] = key
	// 前半递归
	QuickSort(src, low, first - 1)
	// 后半递归
	QuickSort(src, first + 1, high)
}
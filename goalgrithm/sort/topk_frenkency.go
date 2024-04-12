/**
 *
 *  出现频率最多的 k 个元素
 */
package sort

import "sort"

/**
 * 设置若干个桶，每个桶存储出现频率相同的数。桶的下标表示数出现的频率，即第 i 个桶中存储的数出现的频率为 i。
把数都放到桶之后，从后向前遍历桶，最先得到的 k 个数就是出现频率最多的的 k 个数。
*/
func topKFrequent(nums []int, k int) []int {
	sorted := sort.IntSlice(nums)
	sorted.Sort()
	return nil
}

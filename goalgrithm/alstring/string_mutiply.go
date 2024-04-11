package alstring

import (
	"strconv"
	"strings"
)

/*
给定两个以字符串形式表示的非负整数num1和num2，返回num1和num2的乘积，它们的乘积也表示为字符串形式。

注意：不能使用任何内置的 BigInteger 库或直接将输入转换为整数。

示例 1:
输入: num1 = "2", num2 = "3"
输出: "6"

示例2:
输入: num1 = "123", num2 = "456"
输出: "56088"
提示：

1 <= num1.length, num2.length <= 200
num1和 num2只能由数字组成。
num1和 num2都不包含任何前导零，除了数字0本身。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/multiply-strings
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

解法思路：
n1和n2最大位数是 n1的长度 + n2的长度
利用乘法分解并把结果相加的原理
1. 被乘数取第j位数字，n2[j]
2. 把n2[j] 与n1从右到左的每个数字相乘，在这个过程中注意考虑进位
3. 把2得到的结果写入到 result 数组中，从左到右开始写，其中result[k] 代表的数值是 result[k] * 10
4. 重复1
5. 计算result[k] 相加的结果

// 题设说全部是 >0 的数字，所以不涉及到正负号判定的问题
对整体结果求正负号原理：
利用 ^ 计算乘数和被乘数的结果符号
*/

func stringMutiply(str1, str2 string) string {
	if str1 == "0" || str2 == "0" {
		return "0"
	}
	len1 := len(str1)
	len2 := len(str2)
	result := make([]int, len1+len2)

	// 从右到左遍历被乘数
	for i := len1 - 1; i >= 0; i-- {
		// 表示乘法的进位
		// 从右到左遍历乘数
		for j := len2 - 1; j >= 0; j-- {
			int1, err := strconv.Atoi(string(str2[j]))
			if err != nil {
				return ""
			}
			int2, err := strconv.Atoi(string(str1[i]))
			if err != nil {
				return ""
			}
			// 从后向前依次放循环相乘的结果，最终result里放的是每位子乘法得到的结果
			result[i+j+1] += int1 * int2
		}
	}

	// 对结果做累加求和
	// 进位使用临时变量
	temp := 0
	for k := len(result) - 1; k >= 0; k-- {
		sum := result[k] + temp
		// 新的进位数
		temp = sum / 10
		//当前位的值
		result[k] = sum % 10
	}

	builder := &strings.Builder{}
	// 打印结果
	for k := 0; k < len(result); k++ {
		if k != 0 || result[k] != 0 {
			builder.WriteString(strconv.Itoa(result[k]))
		}
	}

	return builder.String()
}

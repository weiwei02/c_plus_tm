// https://leetcode.cn/problems/simplify-path/description
// 给你一个字符串 path ，表示指向某一文件或目录的 Unix 风格 绝对路径 （以 '/' 开头），请你将其转化为更加简洁的规范路径。
// 在 Unix 风格的文件系统中，一个点（.）表示当前目录本身；此外，两个点 （..） 表示将目录切换到上一级（指向父目录）；两者都可以是复杂相对路径的组成部分。任意多个连续的斜杠（即，'//'）都被视为单个斜杠 '/' 。 对于此问题，任何其他格式的点（例如，'...'）均被视为文件/目录名称。

// 请注意，返回的 规范路径 必须遵循下述格式：

// 始终以斜杠 '/' 开头。
// 两个目录名之间必须只有一个斜杠 '/' 。
// 最后一个目录名（如果存在）不能 以 '/' 结尾。
// 此外，路径仅包含从根目录到目标文件或目录的路径上的目录（即，不含 '.' 或 '..'）。
// 返回简化后得到的 规范路径 。

// 示例 1：
// 输入：path = "/home/"
// 输出："/home"

// 解释：
// 应删除尾部斜杠。

// 示例 2：
// 输入：path = "/home//foo/"
// 输出："/home/foo"

// 解释：
// 多个连续的斜杠被单个斜杠替换。

// 示例 3：

// 输入：path = "/home/user/Documents/../Pictures"
// 输出："/home/user/Pictures"

// 解释：
// 两个点 ".." 表示上一级目录。

// 示例 4：
// 输入：path = "/../"
// 输出："/"

// 解释：
// 不可能从根目录上升级一级。

// 示例 5：
// 输入：path = "/.../a/../b/c/../d/./"
// 输出："/.../b/d"

// 解释：
// "..." 是此问题中目录的有效名称。

// 提示：
// 1 <= path.length <= 3000
// path 由英文字母，数字，'.'，'/' 或 '_' 组成。
// path 是一个有效的 Unix 风格绝对路径。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_path() {
        // assert_eq!(Solution::simplify_path(String::from("/home/")), "/home");
        // assert_eq!(
        //     Solution::simplify_path(String::from("/home//foo/")),
        //     "/home/foo"
        // );
        // assert_eq!(
        //     Solution::simplify_path(String::from("/home/user/Documents/../Pictures")),
        //     "/home/user/Pictures"
        // );
        // assert_eq!(Solution::simplify_path(String::from("/../")), "/");
        assert_eq!(
            Solution::simplify_path(String::from("/.../a/../b/c/../d/./")),
            "/.../b/d"
        );
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 1. 遍历字符串，遇到'/'，就将字符串其加入栈中
    // 2. 遇到'.'，判断当前字符串是否为空或者".."
    // 3. 如果是，则继续遍历下一个字符
    // 4. 如果是"."，则跳过
    // 5. 如果是其他字符串，则将该字符串压入栈中
    // 6. 最后返回栈中的字符串
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<String> = vec![];
        let mut resultstack: Vec<String> = vec![];

        for c in path.chars() {
            if c == '/' {
                if stack.last() != Some(&"/".to_string()) {
                    stack.push(c.to_string());
                }
            } else if !stack.is_empty() {
                if stack.last() == Some(&"/".to_string()) {
                    stack.push(c.to_string());
                } else {
                    stack.last_mut().unwrap().push(c);
                }
            }
        }

        println!("stack: {:?}", stack);
        for i in 0..stack.len() {
            if stack[i] == "." {
                continue;
            } else if stack[i] == ".." {
                if resultstack.len() > 1 {
                    // 出栈两次
                    resultstack.pop();
                    resultstack.pop();
                }
            } else if stack[i] == "/" {
                if i == 0 {
                    resultstack.push(stack[i].to_string());
                } else if resultstack.last() != Some(&stack[i]) {
                    resultstack.push(stack[i].to_string());
                }
            } else {
                resultstack.push(stack[i].to_string());
            }
        }
        let result = resultstack.concat().trim_end_matches(|c| c == '/').to_string();
        if result.is_empty() {
            return "/".to_string();
        } else {
            return result;
        }
    }
}

// https://leetcode.cn/problems/course-schedule/description/?envType=study-plan-v2&envId=top-interview-150
// 207. 课程表
// 中等
// 你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。
// 在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，
// 其中 prerequisites[i] = [ai, bi] ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。

// 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
// 请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。

// 示例 1：
// 输入：numCourses = 2, prerequisites = [[1,0]]
// 输出：true
// 解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。

// 示例 2：
// 输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
// 输出：false
// 解释：总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。

// 提示：
// 1 <= numCourses <= 2000
// 0 <= prerequisites.length <= 5000
// prerequisites[i].length == 2
// 0 <= ai, bi < numCourses
// prerequisites[i] 中的所有课程对 互不相同

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_schedule() {
        assert_eq!(
            Solution::can_finish(4, vec![vec![0, 10], vec![3, 18], vec![5, 5]]),
            false
        );
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            Solution::can_finish(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]),
            false
        );
        assert_eq!(
            Solution::can_finish(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]),
            true
        );
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);

        assert_eq!(
            Solution::can_finish(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]),
            false
        );
        assert_eq!(
            Solution::can_finish(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            true
        );
    }
}

struct Solution;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

impl Solution {
    // 思路：
    // 1. 构建图
    // 2. 遍历图，如果发现环，则返回false
    // 3. 如果没有环，则返回true
    // 使用拓扑排序，如果存在环，则返回false
    // 构建图的方法：
    //
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let courses_map = Course::build_from_prerequisites_vec(&prerequisites);
        let mut visited: HashSet<i32> = HashSet::new();
        let mut in_stack: HashSet<i32> = HashSet::new();
        if prerequisites.len() == 0 {
            return true;
        }

        // 1. 找到没有环路的课程起点，并开始执行搜索。直到满足 count == num_courses 时候结束。
        for (_, course) in courses_map.iter() {
            // if !course.borrow().prerequisites.is_empty() {
            //     continue;
            // }
            if course.borrow().dfs_has_circle(&mut visited, &mut in_stack) {
                return false;
            }
        }
        // println!("visited: {:?} in_stack: {:?}", visited, in_stack);
        if visited.len() == 0 {
            return false;
        }
        true
    }
}

#[derive(Debug)]
struct Course {
    id: i32,
    prerequisites: Vec<Rc<CourseRequirement>>,
    next: Vec<Rc<CourseRequirement>>,
}

#[derive(Debug)]
struct CourseRequirement {
    to: Rc<RefCell<Course>>,
    from: Rc<RefCell<Course>>,
}

impl Course {
    // 从课程依赖关系中构建课程依赖关系图
    fn build_from_prerequisites_vec(
        prerequisites_arr: &Vec<Vec<i32>>,
    ) -> HashMap<i32, Rc<RefCell<Course>>> {
        let mut courses = HashMap::new();

        for requisites_vec in prerequisites_arr.iter() {
            courses
                .entry(requisites_vec[0])
                .or_insert(Rc::new(RefCell::new(Course {
                    id: requisites_vec[0],
                    prerequisites: Vec::new(),
                    next: Vec::new(),
                })));

            courses
                .entry(requisites_vec[1])
                .or_insert(Rc::new(RefCell::new(Course {
                    id: requisites_vec[1],
                    prerequisites: Vec::new(),
                    next: Vec::new(),
                })));
            let a = courses.get(&requisites_vec[0]).unwrap();
            let b = courses.get(&requisites_vec[1]).unwrap();
            let cr = Rc::new(CourseRequirement {
                to: a.clone(),
                from: b.clone(),
            });
            a.borrow_mut().prerequisites.push(cr.clone());
            b.borrow_mut().next.push(cr.clone());
        }
        courses
    }

    fn dfs_has_circle(&self, visited: &mut HashSet<i32>, in_stack: &mut HashSet<i32>) -> bool {
        // 注意环路，一般的环路指的时两个节点之间有环，但是本题中，两个节点之间有环，但是没有环，所以需要判断是否已经访问过
        if in_stack.contains(&self.id) {
            return true;
        }

        // 所有节点都访问过，说明没有环
        if !visited.insert(self.id) {
            return false;
        }
        in_stack.insert(self.id);

        for next in self.next.iter() {
            if next.to.borrow().dfs_has_circle(visited, in_stack) {
                return true;
            }
        }
        in_stack.remove(&self.id);
        false
    }
}

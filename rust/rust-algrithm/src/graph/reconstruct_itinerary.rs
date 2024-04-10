// You are given a list of airline tickets where tickets[i] = [fromi, toi] 
// represent the departure and the arrival airports of one flight. 
// Reconstruct the itinerary in order and return it.

// All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK". If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.

// For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
// You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.

// Exmple 1:
// Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
// Output: ["JFK","MUC","LHR","SFO","SJC"]

// Exmple 2:
// Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
// Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
// Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.

// Constraints:

// 1 <= tickets.length <= 300
// tickets[i].length == 2
// fromi.length == 3
// toi.length == 3
// fromi and toi consist of uppercase English letters.
// fromi != toi

use std::collections::HashMap;

// 给定一个机票的字符串二维数组[from, to]，子数组中的两个成员表示航班的出发和降落地点，对该形成进行重新规划排序。所有的机票都属于从JFK（肯尼迪国际机场）出发的先生，所以该行程必须从 JFK 出发。
// 说明:
// 1. 如果存在多种有效行程，你可以按字符串排序返回最小的行程组合。例如行程 ["JFK", "LGA"] 与行程 ["JFK", "LGB"] 相比，["JFK", "LGA"] 更小，排序靠前。
// 2. 所有机场都使用三个大写字母表示。
// 3. 假定机票至少存在一种合理的行程。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_re_match() {
        let tikets  =vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
        ];
        let itinerary = vec!["JFK".to_string(), "MUC".to_string(), "LHR".to_string(), "SFO".to_string()];
        assert_eq!(reconstruct_itimerary(tikets), itinerary);
    }
}

fn reconstruct_itimerary(tickets: Vec<Vec<String>>) -> Vec<String> { 
    let itinerary:Vec<String> = Vec::new();
    // let mut ticketMap:HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    // // 第一步，构建一个所有航班的映射表
    // for ticket in tickets.iter() {
    //     let from = ticket.get(0).unwrap().as_str();
    //     let to = ticket.get(1).unwrap().as_str();

    //     let to_map = match ticketMap.get_mut(from) {
    //         Some(map) => map,
    //         None => {
    //             ticketMap.insert(from, HashMap::new());
    //             ticketMap.get_mut(from).unwrap()
    //         }
    //     };
        
    //     if to_map.contains_key(to) {
    //         to_map.insert(to, to_map.get(to).unwrap() + 1);
    //     } else {
    //         to_map.insert(to, 1);
    //     }
    // }

    // // 第二步，构建多个可能的路径结果
    // let try_map = ticketMap.clone();
    // let mut paths:Vec<String> = Vec::new();
    // let jfk_map = try_map.get("JFK").unwrap();
    

    itinerary
}
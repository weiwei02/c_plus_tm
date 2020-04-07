// 无序容器，比传统的 map/multimap,set/multiset 效率更高
// map 和set 内部的实现是红黑树，在做插入操作时会自动排序
// Created by wangww on 2020/4/6.
// 无序容器需要key自己实现 hash_value 函数
#include <unordered_map>
#include <string>
#include <bitset>
#include <vector>

using std::string;
struct Key{
    std::string first;
    std::string second;
};

struct KeyHash{
    std::size_t operator()(const Key& k) const {
        return std::hash<std::string>()(k.first) ^ (std::hash<std::string>()(k.second) << 1);
    }
};

struct KeyEqual{
    bool operator()(const Key &lhs, const Key &rhs) const {
        return lhs.first == rhs.first && lhs.second == rhs.second;
    }
};
int t_unorder_map(){
    // default constructor :empty map
    std::unordered_map<string, string> m1;

    // list construct
    std::unordered_map<int ,string> m2 = {
            {1, "foo"},
            {2, "bar"},
            {3, "baz"},
    };

    // copy constructor
    std::unordered_map<int ,string> m3 = m2;

    // move constructor
    std::unordered_map<int ,string> m4 = std::move(m2);

    //range constructor
    std::vector<std::pair<std::bitset<8>, int >> v = {
            {0x12, 1},
            {0x01, -1},
    };
    std::unordered_map<std::bitset<8>, double> m5(v.begin(), v.end());

    //constructor of custom type
    std::unordered_map<Key ,string, KeyHash, KeyEqual> m6 = {
            {{"John", "Doe"}, "example"},
            {{"Mary", "Sue"}, "another"},
    };
}

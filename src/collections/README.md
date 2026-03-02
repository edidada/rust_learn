# Rust 集合类型示例

本目录包含了 Rust 标准库中各种集合类型的示例代码，对应 C++ 和 Java 中的类似集合。

## 集合类型列表

| 集合类型 | C++ 对应 | Java 对应 | 核心特性 | 示例文件 |
|---------|---------|---------|---------|---------|
| `Vec<T>` | `std::vector` | `ArrayList`, `Vector` | 动态数组，连续内存，支持随机访问 | `vec.rs` |
| `VecDeque<T>` | `std::deque` | `ArrayDeque` | 两端高效插入删除 | `vecdeque.rs` |
| `LinkedList<T>` | `std::list` | `LinkedList` | 双向链表，适用于大量中间插入 | `linkedlist.rs` |
| `Box<[T]>` | 无直接对应 | 无直接对应 | 固定大小数组，运行时决定长度 | `box_array.rs` |
| `BTreeMap<K, V>` | `std::map` | `TreeMap` | 基于 B-树 的有序映射，按键排序 | `btreemap.rs` |
| `HashMap<K, V>` | `std::unordered_map` | `HashMap` | 哈希表实现，无序存储 | `hashmap.rs` |
| `HashSet<T>` | `std::unordered_set` | `HashSet` | 基于 HashMap 的集合实现 | `hashset.rs` |
| `BTreeSet<T>` | `std::set` | `TreeSet` | 基于 BTreeMap 的有序集合 | `btreeset.rs` |

## 运行示例

要运行这些示例，你需要安装 Rust 工具链，然后使用以下命令：

```bash
# 运行 Vec 示例
cargo run --bin vec

# 运行 VecDeque 示例
cargo run --bin vecdeque

# 运行 LinkedList 示例
cargo run --bin linkedlist

# 运行 Box<[T]> 示例
cargo run --bin box_array

# 运行 BTreeMap 示例
cargo run --bin btreemap

# 运行 HashMap 示例
cargo run --bin hashmap

# 运行 HashSet 示例
cargo run --bin hashset

# 运行 BTreeSet 示例
cargo run --bin btreeset
```

## 注意事项

1. 所有示例都包含了详细的注释，解释了各个集合类型的核心特性和API使用方法。
2. 每个示例都展示了集合的基本操作、访问方法、迭代方式以及性能特性。
3. 你可以根据需要修改示例代码，探索不同集合类型的更多用法。

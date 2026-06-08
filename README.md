# omer

表示"一个或多个"的枚举类型，用于处理序列化中"单个值或数组"的灵活场景。

## 解决的问题

在 JSON、YAML、TOML 等常见配置格式中，同一个字段有时是单一值，有时是数组：

```json
{ "depends_on": "postgres" }
```

```json
{ "depends_on": ["postgres", "redis"] }
```

标准 Rust 反序列化难以同时兼容这两种形式。`Omer<T>` 透明地处理"一或多的歧义：

- **序列化时**：`One(T)` 序列化为单个值，`More(Vec<T>)` 序列化为数组
- **反序列化时**：标量值（字符串、数字、布尔、对象）解析为 `One`，数组解析为 `More`

## 快速开始

```toml
[dependencies]
omer = "0.1"
```

`serde` 特性默认开启。如果不需要序列化支持：

```toml
omer = { version = "0.1", default-features = false }
```

## 使用示例

```rust
use omer::Omer;
use serde::Deserialize;

// —— 构造 ——
let one: Omer<i32> = Omer::One(42);
let more: Omer<i32> = Omer::More(vec![1, 2, 3]);

// —— 序列化 ——
let json = serde_json::to_string(&Omer::One("hello")).unwrap();
// => "\"hello\""

let json = serde_json::to_string(&Omer::More(vec!["a", "b"])).unwrap();
// => "[\"a\",\"b\"]"

// —— 反序列化：单个值 => Omer::One ——
#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    packages: Omer<String>,
}

let config: Config = serde_json::from_str(r#"{ "packages": "foo" }"#).unwrap();
assert_eq!(config.packages, Omer::One("foo".into()));

// —— 反序列化：数组 => Omer::More ——
let config: Config = serde_json::from_str(
    r#"{ "packages": ["foo", "bar", "baz"] }"#
).unwrap();
assert_eq!(config.packages, Omer::More(vec!["foo".into(), "bar".into(), "baz".into()]));
```

## 支持的类型转换

反序列化时，非数组类型会通过 serde 的 `Deserialize` 转换为目标类型 `T`：

| 输入类型 | 反序列化结果 |
| --- | --- |
| 数组 `[...]` | `Omer::More(Vec<T>)` |
| 字符串 `"..."` | `Omer::One(T::deserialize(...))` |
| 数字 `42` | `Omer::One(T::deserialize(...))` |
| 布尔 `true` | `Omer::One(T::deserialize(...))` |
| 对象 `{...}` | `Omer::One(T::deserialize(...))` |

## 许可

MIT
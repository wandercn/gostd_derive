# [gostd_derive](https://github.com/wandercn/gostd_derive).
proc_macro_derive library for [gostd](https://github.com/wandercn/gostd).


## Fmt

 用宏模拟实现Go中的Stringer接口。
 在Go中printf函数，自动打印自定义实现的String方法返回内容。

- 使用方法 

`#[derive(Fmt)]`

example:
```
#[derive(Fmt)]
struct Foo{
 ...
}

// 必须为附加Fmt继承宏的Struct 或者 Emun 实现String方法才能正常运行
impl Foo {

    fn String()->String{
        ...
    }
}
```

- 功能逻辑

Fmt功能就是继承Display 并调用String()方法，在println!()实现自定义打印格式。

功能的rust表示如下。
```rust
impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.String())
    }
} 
```

- 如何调试

本库只使用官方的proc_macro没有办法调试。
唯一方法，只有运行 `cargo check` 检查,没报错就问题。



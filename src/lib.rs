//! proc_macro_derive library for [gostd](https://github.com/wandercn/gostd).
#![allow(non_snake_case)]
use proc_macro::TokenStream;
use quote::quote;
use syn;

///  Fmt implement the Stringer interface in Go using macro emulation.
///  Printf function in Go to automatically print the content returned by a custom implementation of String method.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///  用宏模拟实现Go中的Stringer接口。
///  在Go中printf函数，自动打印自定义实现的String方法返回内容。
/// </details>
///
/// - 使用方法
///
///`#[derive(Fmt)]`
///
/// # example:
///```no_run
///use gostd_derive::Fmt;
///
///#[derive(Fmt)]
///struct Foo{
/// name:String,
///}
/// // 必须为附加Fmt继承宏的Struct 或者 Emun 实现自定义的String方法才能正常运行
///
///impl Foo {
///
///    fn String(&self)->String{
///         "test".to_string()
///    }
///}
///```
///
///- 功能逻辑
///
/// Fmt功能就是继承Display 并调用自定义的String()方法，在println!()实现自定义打印格式。
///
/// 功能的rust表示如下。
///```no_run
///struct Foo{
///  name:String,
/// }
///
/// impl Foo {
///
///    fn String(&self)->String{
///         "test".to_string()
///    }
///}
/// // 下面的代码就是宏实现的批量生成代码的rust代码
///impl std::fmt::Display for Foo {
///    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///        write!(f, "{}", self.String())
///    }
///}
///```
///- 如何调试
///
/// 本库只使用官方的proc_macro没有办法调试。
/// 唯一方法，只有运行 `cargo check` 检查,没报错就问题。
#[proc_macro_derive(Fmt)]
pub fn fmt_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_fmt(&ast)
}

fn impl_fmt(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl std::fmt::Display for #name{
             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.String())
            }
        }
    };
    gen.into()
}

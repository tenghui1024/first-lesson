//! 结构体和对应TokenStream结构示例如下:
//!
//! ## Struct
//!
//! ```rust
//! #[allow(dead_code)]
//! #[derive(Debug, RawBuilder)]
//! pub struct Command {
//!     executable: String,
//!     args: Vec<String>,
//!     env: Vec<String>,
//!     current_dir: Option<String>,
//! }
//! ```
//!
//! ## TokenStream
//!
//! ```json
//! TokenStream [
//!     Punct {
//!         ch: '#',
//!         spacing: Alone,
//!         span: #0 bytes(25..26),
//!     },
//!     Group {
//!         delimiter: Bracket,
//!         stream: TokenStream [
//!             Ident {
//!                 ident: "allow",
//!                 span: #0 bytes(27..32),
//!             },
//!             Group {
//!                 delimiter: Parenthesis,
//!                 stream: TokenStream [
//!                     Ident {
//!                         ident: "dead_code",
//!                         span: #0 bytes(33..42),
//!                     },
//!                 ],
//!                 span: #0 bytes(32..43),
//!             },
//!         ],
//!         span: #0 bytes(26..44),
//!     },
//!     Ident {
//!         ident: "pub",
//!         span: #0 bytes(74..77),
//!     },
//!     Ident {
//!         ident: "struct",
//!         span: #0 bytes(78..84),
//!     },
//!     Ident {
//!         ident: "Command",
//!         span: #0 bytes(85..92),
//!     },
//!     Group {
//!         delimiter: Brace,
//!         stream: TokenStream [
//!             Ident {
//!                 ident: "executable",
//!                 span: #0 bytes(99..109),
//!             },
//!             Punct {
//!                 ch: ':',
//!                 spacing: Alone,
//!                 span: #0 bytes(109..110),
//!             },
//!             Ident {
//!                 ident: "String",
//!                 span: #0 bytes(111..117),
//!             },
//!             Punct {
//!                 ch: ',',
//!                 spacing: Alone,
//!                 span: #0 bytes(117..118),
//!             },
//!             Ident {
//!                 ident: "args",
//!                 span: #0 bytes(123..127),
//!             },
//!             Punct {
//!                 ch: ':',
//!                 spacing: Alone,
//!                 span: #0 bytes(127..128),
//!             },
//!             Ident {
//!                 ident: "Vec",
//!                 span: #0 bytes(129..132),
//!             },
//!             Punct {
//!                 ch: '<',
//!                 spacing: Alone,
//!                 span: #0 bytes(132..133),
//!             },
//!             Ident {
//!                 ident: "String",
//!                 span: #0 bytes(133..139),
//!             },
//!             Punct {
//!                 ch: '>',
//!                 spacing: Joint,
//!                 span: #0 bytes(139..140),
//!             },
//!             Punct {
//!                 ch: ',',
//!                 spacing: Alone,
//!                 span: #0 bytes(140..141),
//!             },
//!             Ident {
//!                 ident: "env",
//!                 span: #0 bytes(146..149),
//!             },
//!             Punct {
//!                 ch: ':',
//!                 spacing: Alone,
//!                 span: #0 bytes(149..150),
//!             },
//!             Ident {
//!                 ident: "Vec",
//!                 span: #0 bytes(151..154),
//!             },
//!             Punct {
//!                 ch: '<',
//!                 spacing: Alone,
//!                 span: #0 bytes(154..155),
//!             },
//!             Ident {
//!                 ident: "String",
//!                 span: #0 bytes(155..161),
//!             },
//!             Punct {
//!                 ch: '>',
//!                 spacing: Joint,
//!                 span: #0 bytes(161..162),
//!             },
//!             Punct {
//!                 ch: ',',
//!                 spacing: Alone,
//!                 span: #0 bytes(162..163),
//!             },
//!             Ident {
//!                 ident: "current_dir",
//!                 span: #0 bytes(168..179),
//!             },
//!             Punct {
//!                 ch: ':',
//!                 spacing: Alone,
//!                 span: #0 bytes(179..180),
//!             },
//!             Ident {
//!                 ident: "Option",
//!                 span: #0 bytes(181..187),
//!             },
//!             Punct {
//!                 ch: '<',
//!                 spacing: Alone,
//!                 span: #0 bytes(187..188),
//!             },
//!             Ident {
//!                 ident: "String",
//!                 span: #0 bytes(188..194),
//!             },
//!             Punct {
//!                 ch: '>',
//!                 spacing: Joint,
//!                 span: #0 bytes(194..195),
//!             },
//!             Punct {
//!                 ch: ',',
//!                 spacing: Alone,
//!                 span: #0 bytes(195..196),
//!             },
//!         ],
//!         span: #0 bytes(93..198),
//!     },
//! ]
//! ```
use anyhow::Result;
use askama::Template;
use proc_macro::{Ident, TokenStream, TokenTree};
use std::collections::VecDeque;

/// 处理 jinja 模板的数据结构，在模板中我们使用了 name / builder_name / fields
#[derive(Template)]
#[template(path = "builder.j2", escape = "none")]
pub struct BuilderContext {
    name: String,
    builder_name: String,
    fields: Vec<Fd>,
}

/// 描述 struct 的每个field
#[derive(Debug, Default)]
struct Fd {
    name: String,
    ty: String,
    optional: bool,
}

impl Fd {
    /// name 和 field 都是通过冒号 Punct切分出来的TokenTree切片
    /// eg:
    /// ```json
    /// TokenStream [
    ///     ...
    ///     Group {
    ///         delimiter: Brace,
    ///         stream: TokenStream [
    ///             ...
    ///             Ident {
    ///                 ident: "current_dir",
    ///                 span: #0 bytes(168..179),
    ///             },
    ///             Punct {
    ///                 ch: ':',
    ///                 spacing: Alone,
    ///                 span: #0 bytes(179..180),
    ///             },
    ///             Ident {
    ///                 ident: "Option",
    ///                 span: #0 bytes(181..187),
    ///             },
    ///             Punct {
    ///                 ch: '<',
    ///                 spacing: Alone,
    ///                 span: #0 bytes(187..188),
    ///             },
    ///             Ident {
    ///                 ident: "String",
    ///                 span: #0 bytes(188..194),
    ///             },
    ///             Punct {
    ///                 ch: '>',
    ///                 spacing: Joint,
    ///                 span: #0 bytes(194..195),
    ///             },
    ///             Punct {
    ///                 ch: ',',
    ///                 spacing: Alone,
    ///                 span: #0 bytes(195..196),
    ///             },
    ///         ],
    ///         span: #0 bytes(93..198),
    ///     },
    /// ]
    /// ```
    pub fn new(name: &[TokenTree], ty: &[TokenTree]) -> Self {
        // 把类似 Ident("Option"), Punct('<'), Ident("String"), Punct('>) 的ty
        // 收集成一个String列表, 如 vec!["Option", "<", "String", ">"]
        // eg:
        // ```json
        // TokenStream [
        //      ...
        //         stream: TokenStream [
        //             ...
        //             Ident {
        //                 ident: "current_dir",
        //                 span: #0 bytes(168..179),
        //             },
        //             Punct {
        //                 ch: ':',
        //                 spacing: Alone,
        //                 span: #0 bytes(179..180),
        //             },
        //             Ident {
        //                 ident: "Option",
        //                 span: #0 bytes(181..187),
        //             },
        //             Punct {
        //                 ch: '<',
        //                 spacing: Alone,
        //                 span: #0 bytes(187..188),
        //             },
        //             Ident {
        //                 ident: "String",
        //                 span: #0 bytes(188..194),
        //             },
        //             Punct {
        //                 ch: '>',
        //                 spacing: Joint,
        //                 span: #0 bytes(194..195),
        //             },
        //             Punct {
        //                 ch: ',',
        //                 spacing: Alone,
        //                 span: #0 bytes(195..196),
        //             },
        //         ],
        //         span: #0 bytes(93..198),
        //     },
        // ]
        // ```
        let ty = ty
            .iter()
            .map(|v| match v {
                TokenTree::Ident(n) => n.to_string(),
                TokenTree::Punct(p) => p.as_char().to_string(),
                e => panic!("Expect ident, got {:?}", e),
            })
            .collect::<Vec<_>>();

        // 冒号前最后一个TokenTree是field的名字
        // eg: current_dir: Option<String>
        //         stream: TokenStream [
        //             ...
        //             Ident {
        //                 ident: "current_dir",
        //                 span: #0 bytes(168..179),
        //             },
        //             Punct {
        //                 ch: ':',
        //                 spacing: Alone,
        //                 span: #0 bytes(179..180),
        //             },
        //             Ident {
        //                 ident: "Option",
        //                 span: #0 bytes(181..187),
        //             },
        //          ]
        // 注意这里不应该用name[0], 因为可能是pub executable: String
        // 甚至, 带attributes的field
        // 例如: #[builder(hello = world)] pub executable: String
        // 因此, 无论 name的TokenStream列表中包含到少ts, 最后一个一定是名字
        match name.last() {
            Some(TokenTree::Ident(name)) => {
                // 如果ty第0项是Option, 那么从第二项到倒数第一项
                // 取完后上面的例子中的ty会变成["String"], optional=true
                let (ty, optional) = if ty[0].as_str() == "Option" {
                    (&ty[2..ty.len() - 1], true)
                } else {
                    (&ty[..], false)
                };
                Self {
                    name: name.to_string(),
                    ty: ty.join(""), // 把ty join成字符串
                    optional,
                }
            }
            e => panic!("Expect ident, got {:?}", e),
        }
    }
}

impl BuilderContext {
    /// 从TokenStream中提取信息, 构建BuilderContext
    fn new(input: TokenStream) -> Self {
        let (name, input) = split(input);
        let fields = get_struct_fields(input);
        Self {
            builder_name: format!("{}Builder", name),
            name: name.to_string(),
            fields,
        }
    }

    // 把模版渲染成字符串
    pub fn render(input: TokenStream) -> Result<String> {
        let template = Self::new(input);
        Ok(template.render()?)
    }
}

/// 把 TokenStream 分出 struct 的名字，和包含 fields 的 TokenStream
/// ```json
/// TokenStream [
///     Punct {
///         ch: '#',
///         spacing: Alone,
///         span: #0 bytes(25..26),
///     },
///     Group {
///       ...
///     },
///     Ident {
///         ident: "pub",
///         span: #0 bytes(74..77),
///     },
///     Ident {
///         ident: "struct",
///         span: #0 bytes(78..84),
///     },
///     Ident {
///         ident: "Command",
///         span: #0 bytes(85..92),
///     },
///     Group {
///         delimiter: Brace,
///         stream: TokenStream [
///             ...
///         ],
///         span: #0 bytes(93..198),
///     },
/// ]
/// ```
fn split(input: TokenStream) -> (Ident, TokenStream) {
    let mut input = input.into_iter().collect::<VecDeque<_>>();
    // 拿到TokenStream开始遍历, 直到找到struct的TokenTree
    while let Some(item) = input.pop_front() {
        if let TokenTree::Ident(v) = item {
            if v.to_string() == "struct" {
                break;
            }
        }
    }

    // struct 后面是struct name
    //     Ident {
    //         ident: "struct",
    //         span: #0 bytes(78..84),
    //     },
    //     Ident {
    //         ident: "Command",
    //         span: #0 bytes(85..92),
    //     },
    let ident;
    if let Some(TokenTree::Ident(v)) = input.pop_front() {
        ident = v;
    } else {
        panic!("Didn't find struct name");
    }

    // struct后面可能还有若干TokenTree 我们不管, 找到第一个Group
    // 里面包含了一个TokenStream, 存放的是Fields的信息
    // ```json
    // TokenStream [
    //     Punct {
    //     },
    //     Group {
    //     },
    //     Ident {
    //         ident: "pub",
    //         span: #0 bytes(74..77),
    //     },
    //     Ident {
    //         ident: "struct",
    //         span: #0 bytes(78..84),
    //     },
    //     Ident {
    //         ident: "Command",
    //         span: #0 bytes(85..92),
    //     },
    //     Group {
    //         delimiter: Brace,
    //         stream: TokenStream [
    //             Ident {
    //                 ident: "current_dir",
    //                 span: #0 bytes(168..179),
    //             },
    //             Punct {
    //                 ch: ':',
    //                 spacing: Alone,
    //                 span: #0 bytes(179..180),
    //             },
    //             Ident {
    //                 ident: "Option",
    //                 span: #0 bytes(181..187),
    //             },
    //             ...
    //         ],
    //         span: #0 bytes(93..198),
    //     },
    // ]
    // ```
    let mut group = None;
    for item in input {
        if let TokenTree::Group(g) = item {
            group = Some(g);
            break;
        }
    }
    (ident, group.expect("Didn't find field group").stream())
}

/// 从包含fields的TokenStream中切出来一个个Fd
fn get_struct_fields(input: TokenStream) -> Vec<Fd> {
    let input = input.into_iter().collect::<Vec<_>>();
    input
        // 切分出每一个包含field所有信息的&[TokenTree]
        // 得到[&[TokenTree], &[TokenTree], &[TokenTree]]
        // 每一个item是一个包含该field所有信息的TokenTree列表
        .split(|v| match v {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        })
        // 再把每一个包含field信息的item &[TokenTree]进行切分
        // 得到包含field的name和type所有信息的两个TokenTree列表
        // 即[&[&[TokenTree], &[TokenTree]], &[&[TokenTree], &[TokenTree]]]
        .map(|tokens| {
            tokens
                .split(|v| match v {
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        // 过滤一下, 如果得到的Item不是包含两项(name, type)的扔掉
        .filter(|tokens| tokens.len() == 2)
        // 使用Fd::new 为每一项创建一个Fd实例
        .map(|tokens| Fd::new(tokens[0], tokens[1]))
        // 把每一个实例收集到一个Vec中返回(Vec<Fd>)
        .collect()
}

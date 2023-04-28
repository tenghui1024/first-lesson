//! 1. 打印DeriveInput结构
//! 2. 定义自己用于处理derive宏的数据结构
//! 3. 把DeriveInput转换成自己的数据结构
//! 4. 使用quote生成代码
//! 5. 完整实现
//!
//! 示例结构体和对应的DeriveInput结构
//! ```rust
//!#[allow(dead_code)]
//! #[derive(Debug, Builder)]
//! pub struct Command {
//!     executable: String,
//!     args: Vec<String>,
//!     env: Vec<String>,
//!     current_dir: Option<String>,
//! }
//! ```
//!
//! ```json
//! DeriveInput {
//!     attrs: [
//!         Attribute {
//!             pound_token: Pound,
//!             style: Outer,
//!             bracket_token: Bracket,
//!             path: Path {
//!                 leading_colon: None,
//!                 segments: [
//!                     PathSegment {
//!                         ident: Ident {
//!                             ident: "allow",
//!                             span: #0 bytes(24..29),
//!                         },
//!                         arguments: None,
//!                     },
//!                 ],
//!             },
//!             tokens: TokenStream [
//!                 Group {
//!                     delimiter: Parenthesis,
//!                     stream: TokenStream [
//!                         Ident {
//!                             ident: "dead_code",
//!                             span: #0 bytes(30..39),
//!                         },
//!                     ],
//!                     span: #0 bytes(29..40),
//!                 },
//!             ],
//!         },
//!     ],
//!     vis: Public(
//!         VisPublic {
//!             pub_token: Pub,
//!         },
//!     ),
//!     ident: Ident {
//!         ident: "Command",
//!         span: #0 bytes(79..86),
//!     },
//!     generics: Generics {
//!         lt_token: None,
//!         params: [],
//!         gt_token: None,
//!         where_clause: None,
//!     },
//!     data: Struct(
//!         DataStruct {
//!             struct_token: Struct,
//!             fields: Named(
//!                 FieldsNamed {
//!                     brace_token: Brace,
//!                     named: [
//!                         Field {
//!                             attrs: [],
//!                             vis: Inherited,
//!                             ident: Some(
//!                                 Ident {
//!                                     ident: "executable",
//!                                     span: #0 bytes(93..103),
//!                                 },
//!                             ),
//!                             colon_token: Some(
//!                                 Colon,
//!                             ),
//!                             ty: Path(
//!                                 TypePath {
//!                                     qself: None,
//!                                     path: Path {
//!                                         leading_colon: None,
//!                                         segments: [
//!                                             PathSegment {
//!                                                 ident: Ident {
//!                                                     ident: "String",
//!                                                     span: #0 bytes(105..111),
//!                                                 },
//!                                                 arguments: None,
//!                                             },
//!                                         ],
//!                                     },
//!                                 },
//!                             ),
//!                         },
//!                         Comma,
//!                         Field {
//!                             attrs: [],
//!                             vis: Inherited,
//!                             ident: Some(
//!                                 Ident {
//!                                     ident: "args",
//!                                     span: #0 bytes(117..121),
//!                                 },
//!                             ),
//!                             colon_token: Some(
//!                                 Colon,
//!                             ),
//!                             ty: Path(
//!                                 TypePath {
//!                                     qself: None,
//!                                     path: Path {
//!                                         leading_colon: None,
//!                                         segments: [
//!                                             PathSegment {
//!                                                 ident: Ident {
//!                                                     ident: "Vec",
//!                                                     span: #0 bytes(123..126),
//!                                                 },
//!                                                 arguments: AngleBracketed(
//!                                                     AngleBracketedGenericArguments {
//!                                                         colon2_token: None,
//!                                                         lt_token: Lt,
//!                                                         args: [
//!                                                             Type(
//!                                                                 Path(
//!                                                                     TypePath {
//!                                                                         qself: None,
//!                                                                         path: Path {
//!                                                                             leading_colon: None,
//!                                                                             segments: [
//!                                                                                 PathSegment {
//!                                                                                     ident: Ident {
//!                                                                                         ident: "String",
//!                                                                                         span: #0 bytes(127..133),
//!                                                                                     },
//!                                                                                     arguments: None,
//!                                                                                 },
//!                                                                             ],
//!                                                                         },
//!                                                                     },
//!                                                                 ),
//!                                                             ),
//!                                                         ],
//!                                                         gt_token: Gt,
//!                                                     },
//!                                                 ),
//!                                             },
//!                                         ],
//!                                     },
//!                                 },
//!                             ),
//!                         },
//!                         Comma,
//!                         Field {
//!                             attrs: [],
//!                             vis: Inherited,
//!                             ident: Some(
//!                                 Ident {
//!                                     ident: "env",
//!                                     span: #0 bytes(140..143),
//!                                 },
//!                             ),
//!                             colon_token: Some(
//!                                 Colon,
//!                             ),
//!                             ty: Path(
//!                                 TypePath {
//!                                     qself: None,
//!                                     path: Path {
//!                                         leading_colon: None,
//!                                         segments: [
//!                                             PathSegment {
//!                                                 ident: Ident {
//!                                                     ident: "Vec",
//!                                                     span: #0 bytes(145..148),
//!                                                 },
//!                                                 arguments: AngleBracketed(
//!                                                     AngleBracketedGenericArguments {
//!                                                         colon2_token: None,
//!                                                         lt_token: Lt,
//!                                                         args: [
//!                                                             Type(
//!                                                                 Path(
//!                                                                     TypePath {
//!                                                                         qself: None,
//!                                                                         path: Path {
//!                                                                             leading_colon: None,
//!                                                                             segments: [
//!                                                                                 PathSegment {
//!                                                                                     ident: Ident {
//!                                                                                         ident: "String",
//!                                                                                         span: #0 bytes(149..155),
//!                                                                                     },
//!                                                                                     arguments: None,
//!                                                                                 },
//!                                                                             ],
//!                                                                         },
//!                                                                     },
//!                                                                 ),
//!                                                             ),
//!                                                         ],
//!                                                         gt_token: Gt,
//!                                                     },
//!                                                 ),
//!                                             },
//!                                         ],
//!                                     },
//!                                 },
//!                             ),
//!                         },
//!                         Comma,
//!                         Field {
//!                             attrs: [],
//!                             vis: Inherited,
//!                             ident: Some(
//!                                 Ident {
//!                                     ident: "current_dir",
//!                                     span: #0 bytes(162..173),
//!                                 },
//!                             ),
//!                             colon_token: Some(
//!                                 Colon,
//!                             ),
//!                             ty: Path(
//!                                 TypePath {
//!                                     qself: None,
//!                                     path: Path {
//!                                         leading_colon: None,
//!                                         segments: [
//!                                             PathSegment {
//!                                                 ident: Ident {
//!                                                     ident: "Option",
//!                                                     span: #0 bytes(175..181),
//!                                                 },
//!                                                 arguments: AngleBracketed(
//!                                                     AngleBracketedGenericArguments {
//!                                                         colon2_token: None,
//!                                                         lt_token: Lt,
//!                                                         args: [
//!                                                             Type(
//!                                                                 Path(
//!                                                                     TypePath {
//!                                                                         qself: None,
//!                                                                         path: Path {
//!                                                                             leading_colon: None,
//!                                                                             segments: [
//!                                                                                 PathSegment {
//!                                                                                     ident: Ident {
//!                                                                                         ident: "String",
//!                                                                                         span: #0 bytes(182..188),
//!                                                                                     },
//!                                                                                     arguments: None,
//!                                                                                 },
//!                                                                             ],
//!                                                                         },
//!                                                                     },
//!                                                                 ),
//!                                                             ),
//!                                                         ],
//!                                                         gt_token: Gt,
//!                                                     },
//!                                                 ),
//!                                             },
//!                                         ],
//!                                     },
//!                                 },
//!                             ),
//!                         },
//!                         Comma,
//!                     ],
//!                 },
//!             ),
//!             semi_token: None,
//!         },
//!     ),
//! }
//! ```
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path, Type,
    TypePath,
};

/// 描述字段所有信息
struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
}

/// 描述 struct 的所有信息
pub struct BuilderContext {
    // 可以从Ident中直接获取
    name: Ident,
    // 需要从data内部的DataStruct{fields}中获取
    // 目前只关心field 的 ident 和 ty
    fields: Vec<Fd>,
}

/// 把一个Field转换成Fd
impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        Self {
            name: f.ident.unwrap(),
            optional,
            ty: ty.to_owned(),
        }
    }
}

/// 把DeriveInput转换成自己的数据结构BuilderContext
impl From<DeriveInput> for BuilderContext {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;

        // 这里包含了1个隐式匹配: 判断input.data是不是一个Data::Struct(DataStruct), 如果不是条件不成立
        // 和2个显式匹配:
        // 1. fields 必须满足如下要求 -> 必须是Fields::Named(FieldsNamed)
        // 2. 其他字段不关心
        // 然后把input.data赋值给结构体中的named字段, 其他字段不关心, 然后把局部变量named赋值给
        // 最外层的fields
        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields.into_iter().map(Fd::from).collect();
        Self { name, fields: fds }
    }
}

/// 如果 T = Option<Inner> 返回 (true, Inner); 否则返回 (false, T)
///```json
///                             ty: Path(
///                                 TypePath {
///                                     qself: None,
///                                     path: Path {
///                                         leading_colon: None,
///                                         segments: [
///                                             PathSegment {
///                                                 ident: Ident {
///                                                     ident: "Option",
///                                                     span: #0 bytes(175..181),
///                                                 },
///                                                 arguments: AngleBracketed(
///                                                     AngleBracketedGenericArguments {
///                                                         colon2_token: None,
///                                                         lt_token: Lt,
///                                                         args: [
///                                                             Type(
///                                                                 Path(
///                                                                     TypePath {
///                                                                         qself: None,
///                                                                         path: Path {
///                                                                             leading_colon: None,
///                                                                             segments: [
///                                                                                 PathSegment {
///                                                                                     ident: Ident {
///                                                                                         ident: "String",
///                                                                                         span: #0 bytes(182..188),
///                                                                                     },
///                                                                                     arguments: None,
///                                                                                 },
///                                                                             ],
///                                                                         },
///                                                                     },
///                                                                 ),
///                                                             ),
///                                                         ],
///                                                         gt_token: Gt,
///                                                     },
///                                                 ),
///                                             },
///                                         ],
///                                     },
///                                 },
///                             ),
/// ```
fn get_option_inner(ty: &Type) -> (bool, &Type) {
    // 首先模式匹配出segments
    // 这里有一个隐式匹配: ty 必须式Type::Path(TypePath类型)
    // 一个显式匹配: ty.path.segments 赋值给变量segments
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        // 如果 PathSegment 第一个是 Option，那么它内部应该是 AngleBracketed，比如
        // 获取其第一个值，如果是 GenericArgument::Type，则返回 (true, t)
        // 如上的DeriveInput中最后一个field是Option<String>
        // 那么返回如下
        // Type(
        //     Path(
        //         TypePath {
        //             qself: None,
        //             path: Path {
        //                 leading_colon: None,
        //                 segments: [
        //                     PathSegment {
        //                         ident: Ident {
        //                             ident: "String",
        //                             span: #0 bytes(182..188),
        //                         },
        //                         arguments: None,
        //                     },
        //                 ],
        //             },
        //         },
        //     ),
        // ),
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArguments"),
                };
                return (true, t);
            }
        }
    }

    // 如果type不是Option, 如下, 直接返回ty
    // segments: [
    //     PathSegment {
    //         ident: Ident {
    //             ident: "Vec",
    //             span: #0 bytes(145..148),
    //         },
    //         arguments: AngleBracketed(
    //             AngleBracketedGenericArguments {
    //                 colon2_token: None,
    //                 lt_token: Lt,
    //                 args: [
    //                     Type(
    //                         Path(
    //                             TypePath {
    //                                 qself: None,
    //                                 path: Path {
    //                                     leading_colon: None,
    //                                     segments: [
    //                                         PathSegment {
    //                                             ident: Ident {
    //                                                 ident: "String",
    //                                                 span: #0 bytes(149..155),
    //                                             },
    //                                             arguments: None,
    //                                         },
    //                                     ],
    //                                 },
    //                             },
    //                         ),
    //                     ),
    //                 ],
    //                 gt_token: Gt,
    //             },
    //         ),
    //     },
    // ],
    return (false, ty);
}

impl BuilderContext {
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
        // 生成xxxBuilder的ident
        let builder_name = Ident::new(&format!("{}Builder", name), name.span());

        let optionized_fields = self.gen_optionized_fields();
        let methods = self.gen_methods();
        let assigns = self.gen_assigns();

        quote! {
            /// Builder 结构
            #[derive(Debug, Default)]
            struct #builder_name {
                #(#optionized_fields,)*
            }

            /// Builder结构体每个字短赋值的方法，以及build()方法
            impl #builder_name {
                #(#methods)*

                pub fn build(mut self) -> Result<#name, &'static str> {
                    Ok(#name {
                        #(#assigns,)*
                    })
                }
            }

            /// 为使用Builder的原结构提供builder()方法, 生成Builder结构
            impl #name {
                fn builder() -> #builder_name {
                    Default::default()
                }
            }
        }
    }

    /// 为 XXXBuilder 生成 Option 字段
    /// 比如：executable: String -> executable: Option
    fn gen_optionized_fields(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| quote! { #name: std::option::Option<#ty> })
            .collect()
    }

    // 为 XXXBuilder 生成处理函数
    // 比如：methods: fn executable(mut self, v: impl Into) -> Self { self.executable = Some(v); self }
    fn gen_methods(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| {
                quote! {
                   pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                        self.#name = Some(v.into());
                        self
                    }
                }
            })
            .collect()
    }

    // 为 XXXBuilder 生成相应的赋值语句，把 XXXBuilder 每个字段赋值给 XXX 的字段
    // 比如：#field_name: self.#field_name.take().ok_or(" xxx need to be set!")
    fn gen_assigns(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, optional, .. }| {
                if *optional {
                    return quote! {
                        #name: self.#name.take()
                    };
                }

                quote! {
                    #name: self.#name.take().ok_or(concat!(stringify!(#name), " needs to be set!"))?
                }
            })
            .collect()
    }
}

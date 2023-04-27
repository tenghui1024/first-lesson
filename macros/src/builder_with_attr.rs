//! 1. 打印DeriveInput结构
//! 2. 定义自己用于处理derive宏的数据结构
//! 3. 把DeriveInput转换成自己的数据结构
//! 4. 使用quote生成代码
//! 5. 完整实现
use darling::FromField;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path, Type,
    TypePath,
};

#[derive(Debug, Default, FromField)]
#[darling(default, attributes(builder))]
struct Opts {
    each: Option<String>,
    default: Option<String>,
}

/// 描述字段所有信息
struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
    opts: Opts,
}

/// 描述 struct 的所有信息
pub struct BuilderContext {
    name: Ident,
    fields: Vec<Fd>,
}

/// 把一个Field转换成Fd
impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        let opts = Opts::from_field(&f).unwrap_or_default();
        Self {
            opts,
            name: f.ident.unwrap(),
            optional,
            ty: ty.to_owned(),
        }
    }
}

/// 把DeriveInput转换成BuilderContext
impl From<DeriveInput> for BuilderContext {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;

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

// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_option_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Option")
}

// 如果是 T = Vec<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_vec_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Vec")
}

fn get_type_inner<'a>(ty: &'a Type, name: &str) -> (bool, &'a Type) {
    // 首先模式匹配出 segments
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == name {
                // 如果 PathSegment 第一个是 Option/Vec 等类型，那么它内部应该是 AngleBracketed，比如 <T>
                // 获取其第一个值，如果是 GenericArgument::Type，则返回
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
    return (false, ty);
}

impl BuilderContext {
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
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

    fn gen_optionized_fields(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| quote! { #name: std::option::Option<#ty> })
            .collect()
    }

    fn gen_methods(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|f| {
                let name = &f.name;
                let ty = &f.ty;
                // 如果不是Option类型, 且定义了each attribute
                if !f.optional && f.opts.each.is_some() {
                    let each = Ident::new(f.opts.each.as_deref().unwrap(), name.span());
                    let (is_vec, ty) = get_vec_inner(ty);
                    if is_vec {
                        return quote! {
                            pub fn #each(mut self, v: impl Into<#ty>) -> Self {
                                let mut data = self.#name.take().unwrap_or_default();
                                data.push(v.into());
                                self.#name = Some(data);
                                self
                            }
                        };
                    }
                }
                quote! {
                    pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                        self.#name = Some(v.into());
                        self
                    }
                }
            })
            .collect()
    }

    fn gen_assigns(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, optional, opts, .. }| {
                if *optional {
                    return quote! {
                        #name: self.#name.take()
                    };
                }

                // 如果定义了default那么把default里的字符串转换成TokenStream
                // 使用unwrap_or_else
                // 在没有值的时候使用缺省结果
                if let Some(default) = opts.default.as_ref() {
                    let ast: TokenStream = default.parse().unwrap();
                    return quote! {
                        #name: self.#name.take().unwrap_or_else(|| #ast)
                    };
                }

                quote! {
                    #name: self.#name.take().ok_or(concat!(stringify!(#name), " needs to be set!"))?
                }
            })
            .collect()
    }
}

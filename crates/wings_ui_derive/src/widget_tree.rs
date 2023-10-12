#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use core::panic;
use std::ops::Deref;
use proc_macro2::{TokenStream, Punct, Spacing};
use wings_ui::prelude::*;

use quote::{format_ident, quote, TokenStreamExt, ToTokens};
use syn::{parse::{Parse, ParseBuffer, ParseStream}, parse2, punctuated::Punctuated, token::Comma,
          Expr, GenericArgument, GenericParam, Generics, Ident, Result, Token, Type, Visibility,
          WhereClause, token, braced, bracketed, parenthesized};

pub(crate) fn widget_tree(input: TokenStream) -> TokenStream {
    let mut codegen = quote! { define_ui_types_bundles!(); };
    let widget = parse2::<UiWidgetBuilder>(input).unwrap();

    if let Some(node) = widget.node {
        codegen.extend(generate_widget(node, "commands"));
    }

    codegen.into()
}

fn generate_widget(node: UiWidgetNode, spawn_name: &str) -> TokenStream {
    let spawn_fn = format_ident!("{}", spawn_name);
    let UiWidgetNode { widget_type, tags, props, children } = node;
    let full_widget_type = format_ident!("{}Bundle", widget_type.clone().unwrap());
    let mut ctor_codegen = quote! { #full_widget_type::default() };
    let mut children_codegen = quote! {};
    let mut tags_codegen = quote! {};

    if !tags.is_empty() {
        for tag in tags {
            tags_codegen.extend(quote! { #tag, });
        }
    }

    if !props.is_empty() {
        let mut props_codegen = quote! {};
        for prop in props {
            props_codegen.extend(quote! { #prop, });
        }

        let props_type = format_ident!("{}Props", widget_type.clone().unwrap());

        ctor_codegen = quote! {
            #full_widget_type::from(#props_type { #props_codegen ..default() })
        };
    }


    if !children.is_empty() {
        let mut children_tokens = TokenStream::default();
        for child in children {
            children_tokens.extend(generate_widget(*child, "parent"));
        }

        children_codegen = quote! {
            .with_children(|parent| {
                #children_tokens
            })
        };
    }

    let codegen = quote! {
        #spawn_fn.spawn(
            (
                #tags_codegen
                #ctor_codegen,
            ),
        )#children_codegen;
    };

    codegen.into()
}

#[derive(Debug, PartialEq)]
struct UiWidgetBuilder {
    node: Option<UiWidgetNode>,
}

impl Parse for UiWidgetBuilder {
    fn parse(input: ParseStream) -> Result<Self> {
        let widget_type: Ident = input.parse()?;

        return if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            Ok(UiWidgetBuilder {
                node: Some(UiWidgetNode {
                    widget_type: Some(widget_type),
                    tags: Vec::default(),
                    props: Vec::default(),
                    children: Vec::default(),
                }),
            })
        } else {
            let block;
            let _ = braced!(block in input);
            let block_input: TokenStream = block.parse()?;

            if block_input.is_empty() {
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                return Ok(UiWidgetBuilder {
                    node: Some(UiWidgetNode {
                        widget_type: Some(widget_type),
                        tags: Vec::default(),
                        props: Vec::default(),
                        children: Vec::default(),
                    }),
                });
            }

            let block = Some(parse2::<UiWidgetNode>(block_input).unwrap()).unwrap();

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }

            Ok(UiWidgetBuilder {
                node: Some(UiWidgetNode {
                    widget_type: Some(widget_type),
                    tags: block.tags,
                    props: block.props,
                    children: block.children,
                }),
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UiWidgetNode {
    pub widget_type: Option<Ident>,
    pub tags: Vec<Type>,
    pub props: Vec<UiWidgetProp>,
    pub children: Vec<Box<UiWidgetNode>>,
}

#[derive(Debug, PartialEq)]
pub struct UiWidgetProp {
    pub name: Ident,
    pub expr: Expr,
}

impl ToTokens for UiWidgetProp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.name.clone());
        tokens.append(Punct::new(':', Spacing::Alone));
        tokens.extend(self.expr.clone().into_token_stream());
    }
}

impl Parse for UiWidgetNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tags = Vec::new();
        let mut props = Vec::new();
        let mut children = Vec::new();

        loop {
            let name: Option<Ident> = input.parse()?;

            if let Some(name) = name {
                input.parse::<Token![:]>()?;

                if name == "tags" {
                    let array_tag;
                    let x = bracketed!(array_tag in input);

                    if array_tag.is_empty() {
                        continue;
                    }

                    let array_tag = array_tag.parse_terminated(Type::parse, Token![,]).unwrap();
                    for element in array_tag {
                        tags.push(element);
                    }

                } else if name == "child" {
                    let widget_type: Ident = input.parse()?;

                    let block;
                    let x = braced!(block in input);
                    let block_input: TokenStream = block.parse()?;

                    let child_node = parse2::<UiWidgetNode>(block_input).unwrap();

                    children.push(Box::new(UiWidgetNode {
                        widget_type: Some(widget_type),
                        tags: child_node.tags,
                        props: child_node.props,
                        children: child_node.children,
                    }));
                } else if name == "children" {
                    let array_block;
                    let x = bracketed!(array_block in input);

                    if array_block.is_empty() {
                        continue;
                    }

                    loop {
                        let child_widget_type: Option<Ident> = array_block.parse()?;

                        if let Some(child_widget_type) = child_widget_type {
                            let block;
                            let x = braced!(block in array_block);
                            let block_input: TokenStream = block.parse()?;

                            let child_node = parse2::<UiWidgetNode>(block_input).unwrap();

                            children.push(Box::new(UiWidgetNode {
                                widget_type: Some(child_widget_type),
                                tags: child_node.tags,
                                props: child_node.props,
                                children: child_node.children,
                            }));
                        } else {
                            break;
                        }
                    }
                } else {
                    let expr: Expr = input.parse()?;
                    props.push(UiWidgetProp { name, expr });
                }
            } else {
                break;
            }
        }

        Ok(UiWidgetNode {
            widget_type: None,
            tags,
            props,
            children,
        })
    }
}

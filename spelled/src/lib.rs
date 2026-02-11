use proc_macro::TokenStream;
use quote::quote;
use syn::{
    spanned::Spanned,
    Expr, Local, Pat, PatIdent, Token,
    visit_mut::{self, VisitMut},
};

/// Explicit declaration syntax for learning Rust.
///
/// The ownership model reduces to two things:
/// - `owner(x)` — this binding holds the owner tag. It controls when space dies.
/// - `name(r)` — this binding names space owned elsewhere (coordinates/references).
///
/// ## Pattern transforms (left side of `=`)
///
/// - `owner(x)` → `x` (owns the space)
/// - `owner(rebindable(x))` → `mut x` (owns, can rebind)
/// - `name(x)` → `x` (names space owned elsewhere)
/// - `name(rebindable(x))` → `mut x` (names, can rebind to different coordinates)
///
/// ## Expression transforms (right side of `=`)
///
/// - `coord_shared(y)` → `&y` (coordinates, many allowed)
/// - `coord_exclusive(y)` → `&mut y` (coordinates, one allowed)
/// - `coord_heap(y)` → `Box::new(y)` (coordinates, owning, on heap)
/// - `mem_copy(y)` → `y` (copy bytes, Copy types)
/// - `clone_copy(y)` → `y.clone()` (call .clone(), Clone types)
/// - `take(y)` → `y` (take SPACE, original deleted)
/// - `take_or_mem_copy(y)` → `y` (compiler decides: mem_copy if Copy, else take)
/// - `at(r)` → `*r` (get value at coordinates)
///
/// ## Valid combinations
///
/// | Left | Right | Meaning |
/// |------|-------|---------|
/// | `owner(y)` | `take(x)` | tag transfers from x to y |
/// | `owner(y)` | `mem_copy(x)` | new space, y owns it, x unchanged |
/// | `owner(y)` | `take_or_mem_copy(x)` | y owns (new or transferred) |
/// | `owner(y)` | `vec![1,2,3]` | new space created, y owns it |
/// | `name(r)` | `coord_shared(x)` | r borrows x's space (shared) |
/// | `name(r)` | `coord_exclusive(x)` | r borrows x's space (exclusive) |
///
/// Usage:
/// ```ignore
/// fn main() {
///     explicit! {
///         let owner(x) = vec![1,2,3];        // x owns this space
///         let owner(y) = take(x);            // ownership transfers to y
///         let name(r) = coord_shared(y);     // r names y's space
///     }
/// }
/// ```
#[proc_macro]
pub fn explicit(input: TokenStream) -> TokenStream {
    let mut block: syn::Block = syn::parse(
        format!("{{ {} }}", input).parse().unwrap()
    ).expect("Failed to parse explicit! block");

    let mut transformer = ExplicitTransformer;
    for stmt in &mut block.stmts {
        transformer.visit_stmt_mut(stmt);
    }

    let stmts = &block.stmts;
    quote! { #(#stmts)* }.into()
}

struct ExplicitTransformer;

impl VisitMut for ExplicitTransformer {
    fn visit_local_mut(&mut self, local: &mut Local) {
        // First transform the pattern
        transform_pat(&mut local.pat);

        // Then transform the init expression
        if let Some(init) = &mut local.init {
            transform_expr(&mut init.expr);
        }

        visit_mut::visit_local_mut(self, local);
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        transform_expr(expr);
        visit_mut::visit_expr_mut(self, expr);
    }
}

fn transform_pat(pat: &mut Pat) {
    match pat {
        // owner(x) or name(x) → x
        // owner(rebindable(x)) or name(rebindable(x)) → mut x
        Pat::TupleStruct(ts) if is_path_ident(&ts.path, "owner") || is_path_ident(&ts.path, "name") => {
            if let Some(first) = ts.elems.first() {
                match first {
                    // owner(rebindable(x)) or name(rebindable(x)) → mut x
                    Pat::TupleStruct(inner) if is_path_ident(&inner.path, "rebindable") => {
                        if let Some(inner_first) = inner.elems.first() {
                            if let Pat::Ident(ident) = inner_first {
                                *pat = Pat::Ident(PatIdent {
                                    attrs: vec![],
                                    by_ref: None,
                                    mutability: Some(Token![mut](ts.path.span())),
                                    ident: ident.ident.clone(),
                                    subpat: None,
                                });
                            }
                        }
                    }
                    // owner(x) or name(x) → x
                    Pat::Ident(ident) => {
                        *pat = Pat::Ident(PatIdent {
                            attrs: vec![],
                            by_ref: None,
                            mutability: None,
                            ident: ident.ident.clone(),
                            subpat: None,
                        });
                    }
                    _ => {}
                }
            }
        }
        // (name(a), name(b)) → (a, b)
        Pat::Tuple(tuple) => {
            for elem in &mut tuple.elems {
                transform_pat(elem);
            }
        }
        Pat::Paren(paren) => {
            transform_pat(&mut paren.pat);
        }
        _ => {}
    }
}

fn is_path_ident(path: &syn::Path, name: &str) -> bool {
    path.get_ident().map(|i| i == name).unwrap_or(false)
}

fn transform_expr(expr: &mut Expr) {
    match expr {
        Expr::Call(call) => {
            let func_name = get_ident_name(&call.func);

            match func_name.as_deref() {
                Some("coord_shared") => {
                    // coord_shared(y) → &y
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg.clone();
                        transform_expr(&mut arg);
                        *expr = syn::parse_quote! { &#arg };
                    }
                }
                Some("coord_exclusive") => {
                    // coord_exclusive(y) → &mut y
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg.clone();
                        transform_expr(&mut arg);
                        *expr = syn::parse_quote! { &mut #arg };
                    }
                }
                Some("coord_heap") => {
                    // coord_heap(y) → Box::new(y)
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg.clone();
                        transform_expr(&mut arg);
                        *expr = syn::parse_quote! { Box::new(#arg) };
                    }
                }
                Some("mem_copy") | Some("take") | Some("take_or_mem_copy") => {
                    // mem_copy(y), take(y), take_or_mem_copy(y) → y
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg.clone();
                        transform_expr(&mut arg);
                        *expr = arg;
                    }
                }
                Some("clone_copy") => {
                    // clone_copy(y) → y.clone()
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg.clone();
                        transform_expr(&mut arg);
                        *expr = syn::parse_quote! { #arg.clone() };
                    }
                }
                Some("at") => {
                    // at(r) → *r
                    if let Some(arg) = call.args.first() {
                        let mut arg = arg.clone();
                        transform_expr(&mut arg);
                        *expr = syn::parse_quote! { *#arg };
                    }
                }
                _ => {
                    // Transform arguments of other calls
                    for arg in &mut call.args {
                        transform_expr(arg);
                    }
                }
            }
        }
        Expr::Tuple(tuple) => {
            for elem in &mut tuple.elems {
                transform_expr(elem);
            }
        }
        Expr::Paren(paren) => {
            transform_expr(&mut paren.expr);
        }
        Expr::Unary(unary) => {
            transform_expr(&mut unary.expr);
        }
        Expr::Reference(reference) => {
            transform_expr(&mut reference.expr);
        }
        _ => {}
    }
}

fn get_ident_name(expr: &Expr) -> Option<String> {
    if let Expr::Path(path) = expr {
        path.path.get_ident().map(|i| i.to_string())
    } else {
        None
    }
}

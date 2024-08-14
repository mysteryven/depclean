use oxc_ast::{
    ast::{CallExpression, Expression},
    AstKind,
};
use oxc_span::CompactStr;

use crate::{runtime::DepCheckerContext, Checker};

/// Implements a checker for JavaScript family files, read their ESM and `require` dependencies.
#[derive(Default, Debug)]
pub struct JSFamily;

impl Checker for JSFamily {
    fn run(&self, ctx: &DepCheckerContext) {
        // import and export statement
        let module_record = ctx.module_records();
        for request_module in module_record.requested_modules.keys() {
            if is_bare_import(request_module) {
                ctx.add_use(request_module.clone())
            }
        }

        for node in ctx.nodes().iter() {
            match node.kind() {
                // require statement
                AstKind::CallExpression(call_expr) => {
                    if !is_global_require_call(call_expr, ctx) {
                        continue;
                    }
                    if call_expr.arguments.len() != 1 {
                        continue;
                    }
                    if let Some(str) =
                        get_string_value(&call_expr.arguments[0].as_expression().unwrap())
                    {
                        if !is_bare_import(&str) {
                            continue;
                        }
                        ctx.add_use(str);
                    }
                }
                // import("a.js")
                AstKind::ImportExpression(import_expr) => {
                    if let Some(str) = get_string_value(&import_expr.source) {
                        if !is_bare_import(&str) {
                            continue;
                        }
                        ctx.add_use(str);
                    }
                }
                _ => {}
            }
        }
    }
}

fn is_global_require_call(call_expr: &CallExpression, ctx: &DepCheckerContext) -> bool {
    if let Expression::Identifier(ident) = &call_expr.callee {
        if ident.name != "require" {
            return false;
        }
        return ctx.semantic().is_reference_to_global_variable(ident);
    }

    return false;
}

fn is_bare_import(s: &str) -> bool {
    !s.starts_with(".") && !s.starts_with("/")
}

fn get_string_value(expr: &Expression) -> Option<CompactStr> {
    match expr {
        Expression::StringLiteral(str_lit) => Some(str_lit.value.to_compact_str()),
        Expression::TemplateLiteral(temp_lit) => {
            if temp_lit.expressions.is_empty() {
                Some(temp_lit.quasis[0].value.raw.to_compact_str())
            } else {
                None
            }
        }
        _ => None,
    }
}

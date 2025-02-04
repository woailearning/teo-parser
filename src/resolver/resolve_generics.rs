use itertools::Itertools;
use crate::ast::generics::{GenericsConstraint, GenericsDeclaration};
use crate::resolver::resolve_type_expr::resolve_type_expr;
use crate::resolver::resolver_context::ResolverContext;

pub(super) fn resolve_generics_declaration<'a>(
    generics_declaration: &'a GenericsDeclaration,
    context: &'a ResolverContext<'a>
) {
    generics_declaration.identifiers.iter().duplicates_by(|i| i.name()).for_each(|i| {
        context.insert_diagnostics_error(i.span, "duplicated generics identifier")
    })
}

pub(super) fn resolve_generics_constraint<'a>(
    generics_constraint: &'a GenericsConstraint,
    context: &'a ResolverContext<'a>,
    generics_declaration: &'a GenericsDeclaration,
) {
    generics_constraint.items.iter().duplicates_by(|i| i.identifier.name()).for_each(|i| {
        context.insert_diagnostics_error(i.span, "duplicated generics constraint")
    });
    for item in &generics_constraint.items {
        if generics_declaration.identifiers.iter().find(|i| i.name() == item.identifier.name()).is_none() {
            context.insert_diagnostics_error(item.identifier.span, "undefined generics identifier")
        }
        resolve_type_expr(
            &item.type_expr,
            &vec![generics_declaration],
            &vec![],
            context,
        )
    }
}
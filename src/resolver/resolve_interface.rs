use crate::ast::interface::InterfaceDeclaration;
use crate::resolver::resolve_field::{FieldParentType, resolve_field_class};
use crate::resolver::resolve_generics::{resolve_generics_constraint, resolve_generics_declaration};
use crate::resolver::resolve_type_expr::resolve_type_expr;
use crate::resolver::resolver_context::ResolverContext;

pub(super) fn resolve_interface<'a>(interface_declaration: &'a InterfaceDeclaration, context: &'a ResolverContext<'a>) {
    if context.has_examined_default_path(&interface_declaration.string_path) {
        context.insert_duplicated_identifier(interface_declaration.identifier.span);
    }
    if let Some(generics_declaration) = &interface_declaration.generics_declaration {
        resolve_generics_declaration(generics_declaration, context);
        if let Some(generics_constraint) = &interface_declaration.generics_constraint {
            resolve_generics_constraint(generics_constraint, context, generics_declaration);
        }
    }
    for extend in &interface_declaration.extends {
        resolve_type_expr(
            extend,
            &if let Some(generics_declaration) = interface_declaration.generics_declaration.as_ref() {
                vec![generics_declaration]
            } else {
                vec![]
            },
            &if let Some(generics_constraint) = interface_declaration.generics_constraint.as_ref() {
                vec![generics_constraint]
            } else {
                vec![]
            },
            context
        );
        if !extend.resolved().is_interface_object() {
            context.insert_diagnostics_error(extend.span(), "TypeError: type is not interface");
        }
    }
    for field in &interface_declaration.fields {
        resolve_field_class(
            field,
            FieldParentType::Interface,
            interface_declaration.generics_declaration.as_ref(),
            interface_declaration.generics_constraint.as_ref(),
            context,
        )
    }
}

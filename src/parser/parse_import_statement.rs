use crate::ast::identifier::Identifier;
use crate::ast::import::Import;
use crate::ast::literals::StringLiteral;
use crate::parser::parse_identifier::parse_identifier;
use crate::parser::parse_literals::parse_string_literal;
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};

pub(super) fn parse_import_statement(pair: Pair<'_>, source_path: &str, context: &mut ParserContext) -> Import {
    let span = parse_span(&pair);
    let mut identifiers = vec![];
    let mut source: Option<StringLiteral> = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::string_literal => source = Some(parse_string_literal(&current)),
            Rule::import_identifier_list => identifiers = parse_import_identifier_list(current, context),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    let mut file_path = context.file_util.import_path(source_path, source.as_ref().unwrap().value.as_str());
    if let Some(file_found) = match_import_file(&file_path, context) {
        file_path = file_found;
    } else {
        context.insert_error(source.as_ref().unwrap().span.clone(), "ImportError: file doesn't exist")
    }
    Import {
        path: context.next_path(),
        identifiers,
        source: source.unwrap(),
        span,
        file_path,
    }
}

fn parse_import_identifier_list(pair: Pair<'_>, context: &mut ParserContext) -> Vec<Identifier> {
    let mut identifiers = vec![];
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::identifier => identifiers.push(parse_identifier(&current)),
            Rule::TRAILING_COMMA | Rule::BLOCK_CLOSE => (),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    identifiers
}

fn match_import_file(original: &str, context: &mut ParserContext) -> Option<String> {
    if (context.file_util.file_exists)(original) && !(context.file_util.file_is_directory)(original) {
        Some(original.to_string())
    } else {
        let append_extension = format!("{original}.teo");
        if (context.file_util.file_exists)(&append_extension) && !(context.file_util.file_is_directory)(&append_extension) {
            Some(append_extension)
        } else {
            let index_teo = (context.file_util.path_join)(original, "index.teo");
            if (context.file_util.file_exists)(&index_teo) && !(context.file_util.file_is_directory)(&index_teo) {
                Some(index_teo)
            } else {
                None
            }
        }
    }
}
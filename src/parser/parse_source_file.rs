use std::borrow::Cow;
use crate::ast::source::Source;
use crate::parser::parse_source::parse_source;
use crate::parser::parser_context::ParserContext;

pub(super) fn parse_source_file(path: impl AsRef<str>, base_path: &str, context: &mut ParserContext) -> Source {
    let path_str = path.as_ref();
    let abs_path = if (context.file_util.path_is_absolute)(path_str) {
        Cow::Borrowed(path_str)
    } else {
        Cow::Owned((context.file_util.path_join)(base_path, path_str))
    };
    let content = match context.read_file(abs_path.as_ref()) {
        Some(content) => content,
        None => panic!("Cannot read schema file content at '{}'", abs_path)
    };
    parse_source(&content, abs_path.as_ref(), false, context)
}
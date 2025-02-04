mod test {
    use teo_parser::{parse, print_to_terminal, generate_json_diagnostics, auto_complete_items};

    #[test]
    fn print() {
        let (_, diagnostics) = parse("test.teo", None, None);
        print_to_terminal(&diagnostics);
    }

    #[test]
    fn lint_to_json() {
        let (_, diagnostics) = parse("/Users/victor/Developer/teo-namespace-example/schema.teo", None, None);
        let result = generate_json_diagnostics(&diagnostics, true);
        println!("{}", result)
    }

    #[test]
    fn print_dup() {
        let (_, diagnostics) = parse("/Users/victor/Developer/teo-namespace-example/schema.teo", None, None);
        print_to_terminal(&diagnostics)
    }

    #[test]
    fn auto_completion() {
        let path = "/Users/victor/Developer/teo-namespace-example/part.teo";
        let (schema, _) = parse(path, None, None);
        let completions = auto_complete_items(&schema, path, (4, 13));
        println!("{:?}", completions);
    }

    #[test]
    fn type_expr() {
        let path = "/Users/victor/Developer/teo-parser/src/builtin/std.teo";
        let (schema, _) = parse(path, None, None);
        //let completions = auto_complete_items(&schema, path, (4, 13));
        //println!("{:?}", completions);
    }

    #[test]
    fn call_expr() {
        let path = "/Users/victor/Developer/teo-namespace-example/part.teo";
        let (schema, _) = parse(path, None, None);
        //let completions = auto_complete_items(&schema, path, (4, 13));
        //println!("{:?}", completions);
    }
}
#[cfg(test)]
mod tests {
    use crate::{compile, Lexer, Parser};

    #[test]
    fn test_simple_controller() {
        let source = "controller UserController {\n    getAll()\n}\n";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let js = compile(&prog);
        assert!(js.contains("UserController"));
        assert!(js.contains("class"));
    }

    #[test]
    fn test_simple_service() {
        let source = "service UserService {\n    findAll() {\n        return []\n    }\n}\n";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let js = compile(&prog);
        assert!(js.contains("UserService"));
        assert!(js.contains("findAll"));
    }

    #[test]
    fn test_simple_dto() {
        let source = "dto UserDto {\n    name: string\n    email: string\n}\n";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let js = compile(&prog);
        assert!(js.contains("UserDto"));
        assert!(js.contains("name"));
        assert!(js.contains("email"));
    }

    #[test]
    fn test_controller_with_param_route() {
        let source = "controller UserController {\n    path \"/api/users\"\n    @Get(\"/:id\")\n    getById(id)\n}\n";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        println!("TOKENS:");
        for (i, t) in tokens.iter().enumerate() {
            println!("  {}: {:?}", i, t);
        }

        let mut parser = Parser::new(tokens);
        let prog = parser.parse();

        for node in &prog.nodes {
            println!("Node: {:?}", node.kind);
            for item in &node.body {
                println!(
                    "  - {:?}: name={:?} props={:?}",
                    item.kind, item.name, item.props
                );
            }
        }

        let js = compile(&prog);
        println!("PARAM ROUTE OUTPUT:\n{}", js);
        assert!(
            js.contains("/api/users/:id"),
            "Expected path to include :id param"
        );
    }
}

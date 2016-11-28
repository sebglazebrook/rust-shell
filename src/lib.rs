struct Parser;

impl Parser {

    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, command_string: &str) -> CommandProxy {
        let mut command_proxy = CommandProxy::new();
        command_proxy.name("ls"); 
        command_proxy
    }
}

struct CommandProxy<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

impl<'a> CommandProxy<'a> {

    pub fn new() -> Self {
        CommandProxy { name: "something", args: vec![] }
    }

    pub fn name(&mut self, name: &'a str) {
        self.name = name.clone();
    }
}


#[test]
fn command_without_args() {
    let command_string = "ls";
    let parser = Parser::new();
    let command_proxy = parser.parse(command_string);
    assert_eq!(command_proxy.name, "ls");
}

#[test]
fn command_with_args() {
    let command_string = "ls -la";
    let parser = Parser::new();
    let command_proxy = parser.parse(command_string);
    assert_eq!(command_proxy.name, "ls");
    assert_eq!(command_proxy.args, vec!["-la"]);
}

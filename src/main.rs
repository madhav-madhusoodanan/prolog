enum BuiltInFunctions {
    cwd(String),
    pwd,
}

enum Token {
    relation(String, Vec<Box<Token>>, bool), 
    variable(String),
    constant(String),
}

enum Operator {
    if_condition,    // lhs, rhs
    and_condition,   // lhs, rhs
    or_condition,    //
    end,
}

enum Thing {
    Op(Operator),
    Tok(Token),
}

struct Expression {
    statements: Vec<Thing>
}

fn main() {
    let mut tokens = Vec::new();
}

fn parse(code: String) -> Result<Token, String> {

}

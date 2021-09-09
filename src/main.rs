use regex::Regex;
use lazy_static::lazy_static;
enum Token {
    Relation(String, Vec<Box<Token>>, bool), 
    Variable(String),
    Constant(String),
}
impl Token {
    fn represent(&self, level: usize) -> String {
        let rep = |name: &String, level: usize| -> String {
            let mut rep = String::new();
            for _ in 0..level {
                rep.push('-');
            }
            rep.push_str(&name);
            rep.push(' ');
            rep
        };

        match self {
                Token::Constant(c) => rep(c, level),
                Token::Variable(v) => rep(v, level),
                Token::Relation(name, vec, _) => {
                    let mut rep_temp = rep(name, level);
                    for i in vec {
                        rep_temp.push_str(&i.represent(level + 1));
                    }
                    rep_temp
            }
        }
    }
}

#[derive(Debug)]
enum ParseError {
    CantRead
}

enum Operator {
    If,    // lhs, rhs
    And,   // lhs, rhs
    Or,
}

impl Operator {
    fn represent(&self) -> String {
        match self {
            Operator::And => String::from("and"),
            Operator::If => String::from("If"),
            Operator::Or => String::from("Or")
        }
    }
}

enum Thing {
    Op(Operator),
    Tok(Token),
}

impl Thing {
    fn represent(&self, _: usize) -> String {
        match self {
            Thing::Tok(tok) => tok.represent(0),
            Thing::Op(op) => op.represent(),
        }
    }
}
struct Evaluator {
    statements: Vec<Thing>
}
impl Evaluator {
    fn new() -> Evaluator {
        Evaluator { statements: Vec::new() } 
    }

    
    fn parse(&self, code: &String) -> Result<&Vec<Thing>, ParseError> {
        let statements: Vec<String> = code.split('.')
                                          .filter(|&line| line.len() != 0)
                                          .map(|line| line.to_string())
                                          .collect();
        for line in statements {
            /* :- , and ; must be split  */
        }
        Ok(&self.statements)
    }

    fn query(&self, code: &String) -> Result<String, ParseError> {
        let mut result = String::new();

        Ok(result)
    }
}

fn main() {
    let mut tokens: Vec<Token> = Vec::new();
}

#[test]
fn array() {
    for i in 0..1 {
        println!("{}", i);
    }
}

#[test]
fn parse_test_prim() -> Result<(), ParseError> {
    let mut args: Vec<Box<Token>> = Vec::new();
    args.push(Box::new(Token::Constant(String::from("romeo"))));
    args.push(Box::new(Token::Constant(String::from("juliet"))));
    let line = Thing::Tok(Token::Relation(String::from("loves"), args, true));
    assert_eq!(line.represent(0), "loves -romeo -juliet ");
    Ok(())
}

#[test]
fn parse_test() -> Result<(), ParseError> {
    let evaluator = Evaluator::new();
    let code = "loves(romeo, juliet). loves (Y, X) :- loves(X, Y).".to_string();
    let result = evaluator.parse(&code)?;
    for line in result {
        assert_eq!(line.represent(0), "juliet");
    }
    Ok(())
}

#[test]
fn query() -> Result<(), ParseError> {
    let query = "loves(X, Y).".to_string();

    let evaluator = Evaluator::new();
    let code = "loves(romeo, juliet). loves (Y, X) :- loves(X, Y).".to_string();
    evaluator.parse(&code)?;
    let result = evaluator.query(&query)?;
    assert_eq!(result, "juliet");
    Ok(())
}
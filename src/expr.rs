use crate::expr::LiteralValue::Number;
use crate::expr::LiteralValue::StringValue;
use crate::expr::LiteralValue::True;
use crate::expr::LiteralValue::False;
use crate::expr::LiteralValue::Nil;

use crate::Token;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False, 
    Nil
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Number(x) => x.to_string(),
            StringValue(x) => x.clone(),
            True=> "true".to_string(),
            False=> "false".to_string(),
            Nil => "nil".to_string()
        }
    }
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping {expression: Box<Expr>},
    Literal {value: LiteralValue},
    Unary { operator: Token, right: Box<Expr>}
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } =>  format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string()),
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = &operator.lexeme;
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}


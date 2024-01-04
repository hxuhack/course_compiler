use std::iter::Peekable;
use std::slice::Iter;
use std::fmt;

#[derive(Debug)]
enum Token {
    UNUM(i64),  // Unsigned number
    BINOP(BinOp),  // Binary operator
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    NONE,
}

#[derive(Debug)]
struct BinTree {
    op: Option<BinOp>,
    left: Option<Box<BinTree>>,
    right: Option<Box<BinTree>>,
    value: i64,
}

impl BinTree {
    fn new(op: Option<BinOp>, value:i64) -> BinTree {
        BinTree {
            op,
            left: None,
            right: None,
            value,
        }
    }
}

impl fmt::Display for BinTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(op) = self.op {
            write!(f, "{:?}", op)?;
        } else {
            write!(f, "{}", self.value)?;
        }
        if let Some(left) = &self.left {
            write!(f, ", {}", left)?;
        } else {
            write!(f, ", NULL")?;
        }
        if let Some(right) = &self.right {
            write!(f, ", {}", right)?;
        } else {
            write!(f, ", NULL")?;
        }
        Ok(())
    }
}

fn create_bin_tree(op: Option<BinOp>, left: BinTree, right: BinTree) -> BinTree {
    BinTree {
        op,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        value: 0,
    }
}

#[derive(Debug)]
struct PrattParser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> PrattParser<'a> {
    fn new(tokens: &'a [Token]) -> PrattParser<'a> {
        PrattParser {
            tokens: tokens.iter().peekable(),
        }
    }

    fn parse(&mut self, precedence: i32) -> Result<BinTree, &'static str> {
        let mut left = match self.tokens.next() {
            Some(&Token::UNUM(value)) => BinTree::new(None, value),
            _ => return Err("Expected unsigned number"),
        };

        while let Some(&&Token::BINOP(op)) = self.tokens.peek() {
            let (lp, rp) = match op {
                BinOp::ADD | BinOp::SUB => (1, 2),
                BinOp::MUL | BinOp::DIV => (3, 4),
                BinOp::POW => (6, 5),
                _ => unreachable!(),
            };

            if lp < precedence {
                break;
            }

            self.tokens.next(); // Consume the operator

            let right = self.parse(rp)?;

            left = create_bin_tree(Some(op), left, right);
        }

        Ok(left)
    }
}

fn main() {
    let tokens = vec![
        Token::UNUM(1),
        Token::BINOP(BinOp::ADD),
        Token::UNUM(2),
        Token::BINOP(BinOp::MUL),
        Token::UNUM(3),
        Token::BINOP(BinOp::POW),
        Token::UNUM(4),
        Token::BINOP(BinOp::MUL),
        Token::UNUM(5),
        Token::BINOP(BinOp::SUB),
        Token::UNUM(6),
    ];

    let mut parser = PrattParser::new(&tokens);
    match parser.parse(0) {
        Ok(tree) =>println!("{}", &tree),
        Err(err) => println!("Error: {}", err),
    }
}
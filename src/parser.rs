// Parser for converting tokens into an Abstract Syntax Tree
use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        Parser { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    #[allow(dead_code)]
    fn peek(&self, offset: usize) -> &Token {
        self.tokens.get(self.position + offset).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current()))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while self.current() != &Token::Eof {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        match self.current() {
            Token::Let | Token::Const | Token::Var => self.var_declaration(),
            Token::Function => self.function_declaration(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::For => self.for_statement(),
            Token::Return => self.return_statement(),
            Token::Break => {
                self.advance();
                self.consume_semicolon();
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                self.consume_semicolon();
                Ok(Stmt::Continue)
            }
            Token::Throw => self.throw_statement(),
            Token::Try => self.try_statement(),
            Token::LBrace => self.block_statement(),
            Token::Semicolon => {
                self.advance();
                Ok(Stmt::Empty)
            }
            _ => {
                let expr = self.expression()?;
                self.consume_semicolon();
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let kind = match self.current() {
            Token::Let => VarKind::Let,
            Token::Const => VarKind::Const,
            Token::Var => VarKind::Var,
            _ => return Err("Expected variable declaration".to_string()),
        };
        self.advance();

        let name = match self.current() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected identifier".to_string()),
        };
        self.advance();

        let init = if self.current() == &Token::Eq {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };

        self.consume_semicolon();
        Ok(Stmt::VarDecl { name, init, kind })
    }

    fn function_declaration(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Function)?;

        let name = match self.current() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.advance();

        self.expect(Token::LParen)?;
        let params = self.parameter_list()?;
        self.expect(Token::RParen)?;

        self.expect(Token::LBrace)?;
        let body = self.block_body()?;
        self.expect(Token::RBrace)?;

        Ok(Stmt::FunctionDecl { name, params, body })
    }

    fn parameter_list(&mut self) -> Result<Vec<String>, String> {
        let mut params = Vec::new();

        if self.current() == &Token::RParen {
            return Ok(params);
        }

        loop {
            match self.current() {
                Token::Identifier(name) => {
                    params.push(name.clone());
                    self.advance();
                }
                _ => return Err("Expected parameter name".to_string()),
            }

            if self.current() == &Token::Comma {
                self.advance();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::LBrace)?;
        let body = self.block_body()?;
        self.expect(Token::RBrace)?;
        Ok(Stmt::Block(body))
    }

    fn block_body(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while self.current() != &Token::RBrace && self.current() != &Token::Eof {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::If)?;
        self.expect(Token::LParen)?;
        let test = self.expression()?;
        self.expect(Token::RParen)?;

        let consequent = Box::new(self.statement()?);

        let alternate = if self.current() == &Token::Else {
            self.advance();
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If { test, consequent, alternate })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let test = self.expression()?;
        self.expect(Token::RParen)?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { test, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::For)?;
        self.expect(Token::LParen)?;

        let init = if self.current() == &Token::Semicolon {
            self.advance();
            None
        } else {
            let stmt = self.statement()?;
            Some(Box::new(stmt))
        };

        let test = if self.current() == &Token::Semicolon {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect(Token::Semicolon)?;

        let update = if self.current() == &Token::RParen {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect(Token::RParen)?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::For { init, test, update, body })
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Return)?;

        let value = if self.current() == &Token::Semicolon {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume_semicolon();
        Ok(Stmt::Return(value))
    }

    fn throw_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Throw)?;

        // Throw requires an expression (no line terminator allowed)
        let expr = self.expression()?;
        self.consume_semicolon();
        Ok(Stmt::Throw(expr))
    }

    fn try_statement(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Try)?;
        self.expect(Token::LBrace)?;
        let block = self.block_body()?;
        self.expect(Token::RBrace)?;

        let mut catch_param = None;
        let mut catch_block = None;
        let mut finally_block = None;

        if self.current() == &Token::Catch {
            self.advance();

            // Optional catch parameter
            if self.current() == &Token::LParen {
                self.advance();
                if let Token::Identifier(name) = self.current() {
                    catch_param = Some(name.clone());
                    self.advance();
                }
                self.expect(Token::RParen)?;
            }

            self.expect(Token::LBrace)?;
            catch_block = Some(self.block_body()?);
            self.expect(Token::RBrace)?;
        }

        if self.current() == &Token::Finally {
            self.advance();
            self.expect(Token::LBrace)?;
            finally_block = Some(self.block_body()?);
            self.expect(Token::RBrace)?;
        }

        if catch_block.is_none() && finally_block.is_none() {
            return Err("Try statement must have catch or finally block".to_string());
        }

        Ok(Stmt::Try {
            block,
            catch_param,
            catch_block,
            finally_block,
        })
    }

    fn consume_semicolon(&mut self) {
        if self.current() == &Token::Semicolon {
            self.advance();
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.ternary()?;

        // Check for assignment operators
        let op = match self.current() {
            Token::Eq => Some(None), // Regular assignment
            Token::PlusEq => Some(Some(BinOp::Add)),
            Token::MinusEq => Some(Some(BinOp::Sub)),
            Token::StarEq => Some(Some(BinOp::Mul)),
            Token::SlashEq => Some(Some(BinOp::Div)),
            Token::PercentEq => Some(Some(BinOp::Mod)),
            _ => None,
        };

        if let Some(op) = op {
            match expr {
                Expr::Identifier(name) => {
                    self.advance();
                    let value = Box::new(self.assignment()?);

                    return Ok(match op {
                        None => Expr::Assignment { target: name, value },
                        Some(bin_op) => Expr::CompoundAssignment {
                            target: name,
                            op: bin_op,
                            value,
                        },
                    });
                }
                Expr::Member { object, property, computed } => {
                    self.advance();
                    let value = Box::new(self.assignment()?);

                    // For now, only support simple assignment on members
                    // Compound assignment on members would require member compound assignment node
                    if op.is_some() {
                        return Err("Compound assignment on member expressions not yet supported".to_string());
                    }

                    return Ok(Expr::MemberAssignment {
                        object,
                        property,
                        computed,
                        value,
                    });
                }
                _ => {}
            }
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, String> {
        let mut expr = self.logical_or()?;

        if self.current() == &Token::Question {
            self.advance();
            let consequent = Box::new(self.expression()?);
            self.expect(Token::Colon)?;
            let alternate = Box::new(self.expression()?);
            expr = Expr::Conditional {
                test: Box::new(expr),
                consequent,
                alternate,
            };
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.logical_and()?;

        while self.current() == &Token::Or {
            self.advance();
            let right = Box::new(self.logical_and()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::Or,
                right,
            };
        }

        Ok(left)
    }

    fn logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.bitwise_or()?;

        while self.current() == &Token::And {
            self.advance();
            let right = Box::new(self.bitwise_or()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::And,
                right,
            };
        }

        Ok(left)
    }

    fn bitwise_or(&mut self) -> Result<Expr, String> {
        let mut left = self.bitwise_xor()?;

        while self.current() == &Token::BitOr {
            self.advance();
            let right = Box::new(self.bitwise_xor()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::BitOr,
                right,
            };
        }

        Ok(left)
    }

    fn bitwise_xor(&mut self) -> Result<Expr, String> {
        let mut left = self.bitwise_and()?;

        while self.current() == &Token::BitXor {
            self.advance();
            let right = Box::new(self.bitwise_and()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::BitXor,
                right,
            };
        }

        Ok(left)
    }

    fn bitwise_and(&mut self) -> Result<Expr, String> {
        let mut left = self.equality()?;

        while self.current() == &Token::BitAnd {
            self.advance();
            let right = Box::new(self.equality()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::BitAnd,
                right,
            };
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison()?;

        loop {
            let op = match self.current() {
                Token::EqEq => BinOp::Eq,
                Token::Ne => BinOp::Ne,
                Token::EqEqEq => BinOp::StrictEq,
                Token::NeEq => BinOp::StrictNe,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.comparison()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right,
            };
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.shift()?;

        loop {
            let op = match self.current() {
                Token::Lt => BinOp::Lt,
                Token::Le => BinOp::Le,
                Token::Gt => BinOp::Gt,
                Token::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.shift()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right,
            };
        }

        Ok(left)
    }

    fn shift(&mut self) -> Result<Expr, String> {
        let mut left = self.additive()?;

        loop {
            let op = match self.current() {
                Token::Shl => BinOp::Shl,
                Token::Shr => BinOp::Shr,
                Token::UShr => BinOp::UShr,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.additive()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right,
            };
        }

        Ok(left)
    }

    fn additive(&mut self) -> Result<Expr, String> {
        let mut left = self.multiplicative()?;

        loop {
            let op = match self.current() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.multiplicative()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right,
            };
        }

        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.exponentiation()?;

        loop {
            let op = match self.current() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.exponentiation()?);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right,
            };
        }

        Ok(left)
    }

    fn exponentiation(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;

        if self.current() == &Token::StarStar {
            self.advance();
            let right = Box::new(self.exponentiation()?); // Right associative
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::Pow,
                right,
            };
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        // Handle 'new' operator
        if self.current() == &Token::New {
            self.advance();
            let callee = Box::new(self.postfix()?);

            // Check if there are arguments
            let args = if let Expr::Call { callee: inner_callee, args } = *callee {
                // Already parsed as a call, use those args
                return Ok(Expr::New {
                    callee: inner_callee,
                    args,
                });
            } else {
                // No arguments, use empty list
                Vec::new()
            };

            return Ok(Expr::New { callee, args });
        }

        // Handle prefix increment/decrement
        let update_op = match self.current() {
            Token::PlusPlus => Some(UpdateOp::Increment),
            Token::MinusMinus => Some(UpdateOp::Decrement),
            _ => None,
        };

        if let Some(op) = update_op {
            self.advance();
            let expr = Box::new(self.unary()?);
            return Ok(Expr::Update {
                expr,
                op,
                prefix: true,
            });
        }

        let op = match self.current() {
            Token::Plus => Some(UnaryOp::Plus),
            Token::Minus => Some(UnaryOp::Minus),
            Token::Not => Some(UnaryOp::Not),
            Token::BitNot => Some(UnaryOp::BitNot),
            Token::TypeOf => Some(UnaryOp::TypeOf),
            _ => None,
        };

        if let Some(op) = op {
            self.advance();
            let expr = Box::new(self.unary()?);
            return Ok(Expr::UnaryOp { op, expr });
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            // Check for postfix increment/decrement
            let update_op = match self.current() {
                Token::PlusPlus => Some(UpdateOp::Increment),
                Token::MinusMinus => Some(UpdateOp::Decrement),
                _ => None,
            };

            if let Some(op) = update_op {
                self.advance();
                expr = Expr::Update {
                    expr: Box::new(expr),
                    op,
                    prefix: false,
                };
                continue;
            }

            match self.current() {
                Token::LParen => {
                    self.advance();
                    let args = self.argument_list()?;
                    self.expect(Token::RParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                Token::LBracket => {
                    self.advance();
                    let property = Box::new(self.expression()?);
                    self.expect(Token::RBracket)?;
                    expr = Expr::Member {
                        object: Box::new(expr),
                        property,
                        computed: true,
                    };
                }
                Token::Dot => {
                    self.advance();
                    let property = match self.current() {
                        Token::Identifier(name) => {
                            let name = name.clone();
                            self.advance();
                            Box::new(Expr::String(name))
                        }
                        _ => return Err("Expected property name".to_string()),
                    };
                    expr = Expr::Member {
                        object: Box::new(expr),
                        property,
                        computed: false,
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn argument_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if self.current() == &Token::RParen {
            return Ok(args);
        }

        loop {
            args.push(self.expression()?);
            if self.current() == &Token::Comma {
                self.advance();
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Null)
            }
            Token::Undefined => {
                self.advance();
                Ok(Expr::Undefined)
            }
            Token::This => {
                self.advance();
                Ok(Expr::This)
            }
            Token::Identifier(name) => {
                self.advance();

                // Check for arrow function: (params) => expr
                if self.current() == &Token::Arrow {
                    self.advance();
                    let body = Box::new(self.assignment()?);
                    return Ok(Expr::ArrowFunction {
                        params: vec![name],
                        body,
                    });
                }

                Ok(Expr::Identifier(name))
            }
            Token::LParen => {
                self.advance();

                // Check if this might be an arrow function by looking ahead
                // Try to parse as parameter list first
                let start_pos = self.position;
                let mut is_arrow_params = false;
                let mut params = Vec::new();

                // Try parsing as parameter list
                if self.current() == &Token::RParen {
                    // Empty parameter list ()
                    is_arrow_params = true;
                } else if let Token::Identifier(name) = self.current() {
                    params.push(name.clone());
                    self.advance();

                    while self.current() == &Token::Comma {
                        self.advance();
                        if let Token::Identifier(name) = self.current() {
                            params.push(name.clone());
                            self.advance();
                        } else {
                            // Not a valid parameter list, reset
                            is_arrow_params = false;
                            break;
                        }
                    }

                    if self.current() == &Token::RParen && self.position > start_pos {
                        let saved_pos = self.position;
                        self.advance(); // consume )
                        if self.current() == &Token::Arrow {
                            is_arrow_params = true;
                            self.position = saved_pos; // reset to before )
                        } else {
                            is_arrow_params = false;
                        }
                    } else {
                        is_arrow_params = false;
                    }
                }

                if is_arrow_params {
                    self.expect(Token::RParen)?;
                    self.expect(Token::Arrow)?;
                    let body = Box::new(self.assignment()?);
                    return Ok(Expr::ArrowFunction { params, body });
                }

                // Not an arrow function, parse as grouped expression
                self.position = start_pos;
                let expr = self.expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::LBracket => {
                self.advance();
                let mut elements = Vec::new();

                while self.current() != &Token::RBracket && self.current() != &Token::Eof {
                    elements.push(self.expression()?);
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }

                self.expect(Token::RBracket)?;
                Ok(Expr::Array(elements))
            }
            Token::LBrace => {
                self.advance();
                let mut properties = Vec::new();

                while self.current() != &Token::RBrace && self.current() != &Token::Eof {
                    let key = match self.current() {
                        Token::Identifier(name) => name.clone(),
                        Token::String(s) => s.clone(),
                        _ => return Err("Expected property key".to_string()),
                    };
                    self.advance();

                    self.expect(Token::Colon)?;
                    let value = self.expression()?;
                    properties.push((key, value));

                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }

                self.expect(Token::RBrace)?;
                Ok(Expr::Object(properties))
            }
            Token::Function => {
                self.advance();
                self.expect(Token::LParen)?;
                let params = self.parameter_list()?;
                self.expect(Token::RParen)?;
                self.expect(Token::LBrace)?;
                let body = self.block_body()?;
                self.expect(Token::RBrace)?;
                Ok(Expr::Function { params, body })
            }
            _ => Err(format!("Unexpected token: {:?}", self.current())),
        }
    }
}

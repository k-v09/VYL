use crate::ast::ASTNode;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        
        Ok(ASTNode::Program(statements))
    }
    
    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if self.match_token(TokenType::Use) {
            return self.parse_use_statement();
        } else if self.match_token(TokenType::Return) {
            return self.parse_return_statement();
        } else if self.match_token(TokenType::If) {
            return self.parse_if_statement();
        } else if self.match_token(TokenType::While) {
            return self.parse_while_loop();
        } else if self.match_token(TokenType::For) {
            return self.parse_for_loop();
        } else if self.match_token(TokenType::Try) {
            return self.parse_try_catch();
        } else if self.match_token(TokenType::Class) {
            return self.parse_class_declaration();
        } else if self.match_token(TokenType::Match) {
            return Ok(ASTNode::ExpressionStatement(Box::new(self.parse_match_expression()?)));
        /*} else if self.match_token(TokenType::Interface) {
            return parse_interface_declaration();*/
        } else if self.match_token(TokenType::LeftBrace) {
            return self.parse_block();
        } else if self.check(TokenType::Type) {
            return self.parse_variable_declaration();
        } else if self.check(TokenType::Identifier) {
            let current_position = self.current;
            if self.peek_next().token_type == TokenType::Slash {
                return self.parse_variable_declaration();
            }

            self.current = current_position;
            return self.parse_expression_statement();
        } else if self.match_token(TokenType::Slash) {
            return self.parse_function_declaration();
        }
        
        Err(format!("Unexpected token: {:?}", self.peek()))
    }
    
    fn parse_use_statement(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::LeftBracket, "Expected '[' after 'use'")?;
        
        let package = self.consume(TokenType::Identifier, "Expected package name")?;
        
        self.consume(TokenType::RightBracket, "Expected ']' after package name")?;
        self.consume(TokenType::Semicolon, "Expected ';' after use statement")?;
        
        Ok(ASTNode::UseStatement(package.lexeme))
    }

    fn parse_return_statement(&mut self) -> Result<ASTNode, String> {
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after return statement")?;
    
        Ok(ASTNode::ReturnStatement(value))
    }
    
    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        println!("Current token: {:?}", self.peek());

        let var_type;
        if self.check(TokenType::Type) {
            var_type = self.advance().lexeme;
        } else if self.check(TokenType::Identifier) {
            // treating it as a type anyway cause im lazy rm
            // see it would've been easier to fix it to "rn" instead of writing this whole comment
            // but oh welli guess this is the state of my world at present. How are you? I hope
            // your day is going well. :)
            var_type = self.advance().lexeme;
        } else {
            return Err(format!("Expected type or identifier, got {:?}", self.peek().token_type));
        }

        let type_token = self.peek().clone();
        println!("Found type: {}, token type: {:?}", var_type, type_token.token_type);
        self.consume(TokenType::Slash, "Expected '/' after type name")?;
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;
        self.consume(TokenType::Slash, "Expected '/' after variable name")?;
        self.consume(TokenType::Equal, "Expected '=' after variable declaration")?;
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration")?;
        Ok(ASTNode::VariableDeclaration {
            var_type,
            name: name.lexeme,
            value,
        })
    }

    /*fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        if !self.check(TokenType::Type) && !self.check(TokenType::Identifier) {
            return Err(format!("Expected type name or identifier, got {:?}", self.peek().token_type));
        }

        let var_type = self.advance().lexeme; 
        self.consume(TokenType::Slash, "Expected '/' after type name")?;
        
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;
        self.consume(TokenType::Slash, "Expected '/' after variable name")?;
        self.consume(TokenType::Equal, "Expected '=' after variable declaration")?;
        
        let value = self.parse_expression()?;
        
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration")?;
        
        Ok(ASTNode::VariableDeclaration {
            var_type,
            name: name.lexeme,
            value,
        })
    }*/

    fn parse_expression_statement(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(ASTNode::ExpressionStatement(expr))
    }
    
    // not sure if I like this yet... but one thing's for sure: I hate myself for trying :p
    // also, good luck reading this, hotshot!
    fn parse_expression(&mut self) -> Result<Box<ASTNode>, String> {
        let expr = self.parse_logical_or()?;

        if self.match_token(TokenType::Question) {
            let then_expr = self.parse_expression()?;
            self.consume(TokenType::Colon, "Expected ':' in ternary expression")?;
            let else_expr = self.parse_expression()?;

            return Ok(Box::new(ASTNode::ConditionalExpression {
                condition: expr,
                then_expr,
                else_expr,
            }));
        }
        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_logical_and()?;

        while self.match_token(TokenType::Pipe) {
            let operator = String::from("|");
            let right = self.parse_logical_and()?;
            expr = Box::new(ASTNode::BinaryExpression {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_equality()?;

        while self.match_token(TokenType::Ampersand) {
            let operator = String::from("&");
            let right  = self.parse_equality()?;
            expr = Box::new(ASTNode::BinaryExpression {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(TokenType::DoubleEqual) || self.match_token(TokenType::NotEqual) {
            let operator = if self.previous().token_type == TokenType::DoubleEqual {
                String::from("==")
            } else {
                String::from("!=")
            };
            let right = self.parse_comparison()?;
            expr = Box::new(ASTNode::BinaryExpression {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_term()?;

        while self.match_token(TokenType::LessThan) ||
              self.match_token(TokenType::GreaterThan) ||
              self.match_token(TokenType::LessEqual) ||
              self.match_token(TokenType::GreaterEqual) {
            let operator = match self.previous().token_type {
                TokenType::LessThan => String::from("<"),
                TokenType::GreaterThan => String::from(">"),
                TokenType::LessEqual => String::from("<="),
                TokenType::GreaterEqual => String::from(">="),
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Box::new(ASTNode::BinaryExpression {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_factor()?;

        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator = if self.previous().token_type == TokenType::Plus {
                String::from("+")
            } else {
                String::from("-")
            };
            let right = self.parse_factor()?;
            expr = Box::new(ASTNode::BinaryExpression {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_unary()?;

        while self.match_token(TokenType::Asterisk) || self.match_token(TokenType::Slash) {
            let operator = if self.previous().token_type == TokenType::Asterisk {
                String::from("*")
            } else {
                String::from("/")
            };
            let right = self.parse_unary()?;
            expr = Box::new(ASTNode::BinaryExpression {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Box<ASTNode>, String> {
        if self.match_token(TokenType::Minus) || 
           self.match_token(TokenType::Bang) ||
           self.match_token(TokenType::Tilde) {
            let operator = match self.previous().token_type {
                TokenType::Minus => String::from("-"),
                TokenType::Bang => String::from("!"),
                TokenType::Tilde => String::from("~"),
                _ => unreachable!(),
            };
            let operand = self.parse_unary()?;
            return Ok(Box::new(ASTNode::UnaryExpression {
                operator,
                operand,
            }));
        }

        if self.check(TokenType::Type) && self.peek_next().token_type == TokenType::Slash {
            return Ok(Box::new(self.parse_type_cast()?));
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(TokenType::Dot) {
                let property = self.consume(TokenType::Identifier, "Expected property name after '.'")?;

                if self.match_token(TokenType::LeftParen) {
                    let mut arguments = Vec::new();

                    if !self.check(TokenType::RightParen) {
                        loop {
                            arguments.push(*self.parse_expression()?);

                            if !self.match_token(TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Expected ')' after method arguments")?;

                    expr = Box::new(ASTNode::MethodCall {
                        object: Box::new(*expr),
                        method: property.lexeme,
                        arguments,
                    });
                } else {
                    expr = Box::new(ASTNode::PropertyAccess{
                        object: Box::new(*expr),
                        property: property.lexeme,
                    });
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /*fn parse_expression(&mut self) -> Result<Box<ASTNode>, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(TokenType::Asterisk) {
                let right = self.parse_primary()?;
                expr = Box::new(ASTNode::BinaryExpression {
                    left: expr,
                    operator: String::from("*"),
                    right,
                });
            } else if self.match_token(TokenType::Dot) {
                let property = self.consume(TokenType::Identifier, "Expected property name after '.'")?;

                if self.match_token(TokenType::LeftParen) {
                    let mut arguments = Vec::new();

                    if !self.check(TokenType::RightParen) {
                        loop {
                            arguments.push(*self.parse_expression()?);

                            if !self.match_token(TokenType::Comma) {
                                break;
                            }
                        }
                    }

                    self.consume(TokenType::RightParen, "Expected ')' after method arguments")?;

                    expr = Box::new(ASTNode::MethodCall {
                        object: Box::new(*expr),
                        method: property.lexeme,
                        arguments,
                    });
                } else {
                    expr = Box::new(ASTNode::PropertyAccess {
                        object: Box::new(*expr),
                        property: property.lexeme,
                    });
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }*/

    fn parse_primary(&mut self) -> Result<Box<ASTNode>, String> {
        if self.match_token(TokenType::LeftBracket) {
            return self.parse_array();
        } else if self.match_token(TokenType::LeftBrace) {
            return self.parse_object();
        } else if self.check(TokenType::Number) {
            let value = self.advance().lexeme;
            return Ok(Box::new(ASTNode::Literal(value)));
        } else if self.check(TokenType::String) {
            let value = self.advance().lexeme;
            return Ok(Box::new(ASTNode::Literal(value)));
        } else if self.check(TokenType::Identifier) {
            let name = self.advance().lexeme;
            if self.match_token(TokenType::LeftParen) {
                let mut arguments = Vec::new();
                if !self.check(TokenType::RightParen) {
                    loop {
                        arguments.push(*self.parse_expression()?);

                        if !self.match_token(TokenType::Comma) {
                            break;
                        }
                    }
                }

                self.consume(TokenType::RightParen, "Expected ')' after function arguments")?;
                return Ok(Box::new(ASTNode::FunctionCall {
                    function: name,
                    arguments,
                }));
            } else {
                return Ok(Box::new(ASTNode::Identifier(name)));
            }
        } else {
            return Err(format!("Expected expression, got {:?}", self.peek()));
        }
    }

    fn parse_array(&mut self) -> Result<Box<ASTNode>, String> {
        let mut elements = Vec::new();
        if !self.check(TokenType::RightBracket) {
            loop {
                elements.push(*self.parse_expression()?);
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBracket, "Expected ']' after array elements")?;
        Ok(Box::new(ASTNode::ArrayLiteral(elements)))
    }

    fn parse_object(&mut self) -> Result<Box<ASTNode>, String> {
        let mut properties = Vec::new();
        if !self.check(TokenType::RightBrace) {
            loop {
                let key = self.consume(TokenType::Identifier, "Expected property name")?;
                self.consume(TokenType::Colon, "Expected ':' after property name")?;

                let value = self.parse_expression()?;
                properties.push((key.lexeme, *value));

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after object properties")?;
        Ok(Box::new(ASTNode::ObjectLiteral(properties)))
    }

    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> {
        let name = self.consume(TokenType::Identifier, "Expected function name after '/'")?;
        
        self.consume(TokenType::Slash, "Expected '/' after function name")?;
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            loop {
                let param_type = self.consume(TokenType::Type, "Expected parameter type")?;
                let param_name = self.consume(TokenType::Identifier, "Expected parameter name")?;
                
                params.push((param_name.lexeme, param_type.lexeme));
                
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        let return_type = self.consume(TokenType::Type, "Expected return type")?; 
        self.consume(TokenType::LeftBrace, "Expected '{' before function body")?;
        let mut body = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after function body")?;
        
        Ok(ASTNode::FunctionDeclaration {
            name: name.lexeme,
            params,
            return_type: return_type.lexeme,
            body,
        })
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "expected ')' after if condition")?;

        self.consume(TokenType::LeftBrace, "Expected '{' before if branch")?;
        let mut then_branch = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after if branch")?;

        let mut else_branch = None;
        if self.match_token(TokenType::Else) {
            self.consume(TokenType::LeftBrace, "Expected '{' before else branch")?;
            let mut else_stmts = Vec::new();
            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                else_stmts.push(self.parse_statement()?);
            }
            self.consume(TokenType::RightBrace, "Expected '}' after else branch")?;
            else_branch = Some(else_stmts);
        }
        Ok(ASTNode::IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_loop(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after while condition")?;

        self.consume(TokenType::LeftBrace, "Expected '{' before while body")?;
        let mut body = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after while body")?;

        Ok(ASTNode::WhileLoop {
            condition,
            body,
        })
    }

    fn parse_for_loop(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'")?;

        let initializer = if self.match_token(TokenType::Semicolon) {
            None
        } else {
            let initializer = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after for initializer")?;
            Some(initializer)
        };

        let condition = if self.check(TokenType::Semicolon) {
            None
        } else {
            let condition = self.parse_expression()?;
            Some(condition)
        };
        self.consume(TokenType::Semicolon, "Expected ';' after for condition")?;

        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            let increment = self.parse_expression()?;
            Some(increment)
        };
        self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;

        self.consume(TokenType::LeftBrace, "Expected '{' before for body")?;
        let mut body = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after for body")?;

        Ok(ASTNode::ForLoop {
            initializer,
            condition,
            increment,
            body,
        })
    }

    fn parse_try_catch(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::LeftBrace, "Expected '{' after 'try'")?;
        let mut try_block = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            try_block.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after try block")?;

        self.consume(TokenType::Catch, "Expected 'catch' after try block")?;
        self.consume(TokenType::LeftParen, "Expected '(' after 'catch'")?;

        let mut catch_variable = None;
        if self.check(TokenType::Type) {
            self.advance();
            let error_var = self.consume(TokenType::Identifier, "Expected error variable name")?;
            catch_variable = Some(error_var.lexeme);
        }

        self.consume(TokenType::RightParen, "Expected ')' after catch declaration")?;
        self.consume(TokenType::LeftBrace, "Expected '{' before catch block")?;
        let mut catch_block = Vec::new();
        while !self.check(TokenType::RightBrace) &&!self.is_at_end() {
            catch_block.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after catch block")?;

        let mut finally_block = None;
        if self.match_token(TokenType::Finally) {
            self.consume(TokenType::LeftBrace, "Expected '{' before finally block")?;
            let mut finally_stmts = Vec::new();
            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                finally_stmts.push(self.parse_statement()?);
            }
            self.consume(TokenType::RightBrace, "Expected '}' after finally block")?;
            finally_block = Some(finally_stmts);
        }

        Ok(ASTNode::TryCatch {
            try_block,
            catch_variable,
            catch_block,
            finally_block,
        })
    }

    fn parse_class_declaration(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::Slash, "Expected '/' after 'Class'")?;
        let name = self.consume(TokenType::Identifier, "Expected class name")?;
        self.consume(TokenType::Slash, "Expected '/' after class name")?;

        let mut extends = None;
        if self.match_token(TokenType::Identifier) && self.previous().lexeme == "extends" {
            let parent = self.consume(TokenType::Type, "Expected parent class name")?;
            extends = Some(parent.lexeme);
        }

        let mut implements = Vec::new();
        if self.match_token(TokenType::Identifier) && self.previous().lexeme == "implements" {
            loop {
                let interface = self.consume(TokenType::Type, "Expected interface name")?;
                implements.push(interface.lexeme);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::LeftBrace, "Expected '{' before class body")?;
        let mut methods = Vec::new();
        let mut properties = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if self.check(TokenType::Type) || self.check(TokenType::Identifier) {
                let property = self.parse_variable_declaration()?;
                properties.push(property);
            } else if self.match_token(TokenType::Slash) {
                let method = self.parse_function_declaration()?;
                methods.push(method);
            } else {
                return Err(format!("Expected property or method declaration, got {:?}", self.peek()));
            }
        }
        self.consume(TokenType::RightBrace, "Expected '}' after class body")?;

        Ok(ASTNode::ClassDeclaration {
            name: name.lexeme,
            extends,
            implements,
            methods,
            properties,
        })
    }

    fn parse_block(&mut self) -> Result<ASTNode, String> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after block")?;

        Ok(ASTNode::Block(stmts))
    }

    fn parse_match_expression(&mut self) -> Result<ASTNode, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'match'")?;
        let expression = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after match expression")?;

        self.consume(TokenType::LeftBrace, "Expected '{' before match cases")?;

        let mut cases = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_expression()?;
            self.consume(TokenType::Arrow, "Expected '=>' after match pattern")?;

            self.consume(TokenType::LeftBrace, "Expected '{' before case body")?;
            let mut body = Vec::new();
            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                body.push(self.parse_statement()?);
            }
            self.consume(TokenType::RightBrace, "Expected '}' after case body")?;

            cases.push((*pattern, body));
            self.match_token(TokenType::Comma);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after match cases")?;

        Ok(ASTNode::MatchExpression {
            expression,
            cases,
        })
    }

    fn parse_type_cast(&mut self) -> Result<ASTNode, String> {
        let target_type = self.consume(TokenType::Type, "Expected type name")?;
        self.consume(TokenType::Slash, "Expected '/' after type name")?;
        self.consume(TokenType::LeftParen, "Expected '(' after type cats")?;
        let expression = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after type cast expression")?;

        Ok(ASTNode::TypeCast {
            expression,
            target_type: target_type.lexeme,
        })
    }
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} - got {:?} instead", error_message, self.peek().token_type))
        }
    }
    
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> &Token {
        if self.current + 1 >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.current + 1]
        }
    }
    
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
}

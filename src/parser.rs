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
    
    fn parse_expression(&mut self) -> Result<Box<ASTNode>, String> {
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
    }

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

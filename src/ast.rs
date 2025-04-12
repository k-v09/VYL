#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    UseStatement(String),
    VariableDeclaration {
        var_type: String,
        name: String,
        value: Box<ASTNode>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<(String, String)>, // (param_name, param_type)
        return_type: String,
        body: Vec<ASTNode>,
    },
    Literal(String),
    Identifier(String),
    ArrayLiteral(Vec<ASTNode>),
    ObjectLiteral(Vec<(String, ASTNode)>),
    PropertyAccess {
        object: Box<ASTNode>,
        property: String,
    },
    FunctionCall {
        function: String,
        arguments: Vec<ASTNode>,
    },
    BinaryExpression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
}

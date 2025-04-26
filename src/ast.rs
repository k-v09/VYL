#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    UseStatement(String),
    ReturnStatement(Box<ASTNode>),
    VariableDeclaration {
        var_type: String,
        name: String,
        value: Box<ASTNode>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<(String, String)>,
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
    MethodCall {
        object: Box<ASTNode>,
        method: String,
        arguments: Vec<ASTNode>,
    },
    ExpressionStatement(Box<ASTNode>),
}

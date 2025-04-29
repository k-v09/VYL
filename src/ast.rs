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
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Vec<ASTNode>>,
    },
    WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    ForLoop {
        initializer: Option<Box<ASTNode>>,
        condition: Option<Box<ASTNode>>,
        increment: Option<Box<ASTNode>>,
        body: Vec<ASTNode>,
    },
    UnaryExpression {
        operator: String,
        operand: Box<ASTNode>,
    },
    TypeCast {
        expression: Box<ASTNode>,
        target_type: String,
    },
    ConditionalExpression {
        condition: Box<ASTNode>,
        then_expr: Box<ASTNode>,
        else_expr: Box<ASTNode>,
    },
    TryCatch {
        try_block: Vec<ASTNode>,
        catch_variable: Option<String>,
        catch_block: Vec<ASTNode>,
        finally_block: Option<Vec<ASTNode>>,
    },
    ClassDeclaration {
        name: String,
        extends: Option<String>,
        implements: Vec<String>,
        methods: Vec<ASTNode>,
        properties: Vec<ASTNode>,
    },
    Block(Vec<ASTNode>),
    MatchExpression {
        expression: Box<ASTNode>,
        cases: Vec<(ASTNode, Vec<ASTNode>)>,
    },
}

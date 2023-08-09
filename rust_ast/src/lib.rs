pub enum Statement {
    ExpressionStatement(Box<ExpressionStatement>),
    EmptyStatement(Box<EmptyStatement>),
}

pub enum Expression {
    BinaryExpression(Box<BinaryExpression>),
    NumberLiteral(Box<NumberLiteral>),
    StringLiteral(Box<StringLiteral>),
    BooleanLiteral(Box<BooleanLiteral>),
    NullLiteral(Box<NullLiteral>),
}

pub enum BinaryExpression {
    AdditionExpression(Box<AdditionExpression>),
    SubtractionExpression(Box<SubtractionExpression>),
    MultiplicationExpression(Box<MultiplicationExpression>),
    DivisionExpression(Box<DivisionExpression>),
    GreaterThanExpression(Box<GreaterThanExpression>),
    GreaterThanOrEqualToExpression(Box<GreaterThanOrEqualToExpression>),
    LessThanExpression(Box<LessThanExpression>),
    LessThanOrEqualToExpression(Box<LessThanOrEqualToExpression>),
    AndExpression(Box<AndExpression>),
    OrExpression(Box<OrExpression>),
}

pub struct EmptyStatement {
    pub span: Span,
}

pub struct ExpressionStatement {
    pub span: Span,
    pub expression: Expression,
}

pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct AstNode {
    pub span: Span,
}

pub struct Identifier {
    pub span: Span,
    pub name: String,
}

pub struct MemberExpression {
    pub span: Span,
    pub base: Expression,
    pub name: Identifier,
}

pub struct CallArgument {
    pub span: Span,
    pub name: Identifier,
    pub value: Expression,
}

pub struct CallExpression {
    pub span: Span,
    pub base: Expression,
    pub arguments: Vec<CallArgument>,
}

pub struct OrExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct AndExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct GreaterThanExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct GreaterThanOrEqualToExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct LessThanExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct LessThanOrEqualToExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct AdditionExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct SubtractionExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct MultiplicationExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct DivisionExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

pub struct StringLiteral {
    pub span: Span,
    pub content: String,
}

pub struct NumberLiteral {
    pub span: Span,
    pub content: usize,
}

pub struct BooleanLiteral {
    pub span: Span,
    pub content: bool,
}

pub struct NullLiteral {
    pub span: Span,
}

// ==== Impl ====

impl Span {
    fn get_location() {}
}

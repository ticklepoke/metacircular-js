#![allow(dead_code)]

use self::literal::Literal;
use serde::Deserialize;

pub mod literal;
pub mod math;

#[derive(Deserialize, Clone)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Deserialize, Clone)]
pub struct SourceLocation {
    source: Option<String>,
    start: Position,
    end: Position,
}

#[derive(Deserialize, Clone)]
pub struct Node {
    loc: Option<SourceLocation>,
    #[serde(flatten)]
    pub kind: NodeKind,
}

// Inheritance chains that do not add extra behaviour
pub type Statement = Box<Node>;
pub type Expression = Box<Node>;
pub type Declaration = Statement;
pub type FunctionBody = BlockStatement;
pub type ThisExpression = Expression;

// es6
pub type Pattern = Box<Node>;
pub type ForOfStatement = ForInStatement;
pub type Super = Box<Node>;
pub type ModuleDeclaration = Box<Node>;
pub type ClassExpression = Class;
pub type ImportDefaultSpecifier = ModuleSpecifier;
pub type ImportNamespaceSpecifier = ModuleSpecifier;

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum NodeKind {
    // es5
    Identifier(Identifier),
    Literal(Literal),
    Program(Program),
    Function(Function),
    ExpressionStatement(ExpressionStatement),
    Directive(Directive),
    BlockStatement(BlockStatement),
    FunctionBody(FunctionBody),
    EmptyStatement(EmptyStatement),
    DebuggerStatement(DebuggerStatement),
    WithStatement(WithStatement),
    ReturnStatement(ReturnStatement),
    LabeledStatement(LabeledStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    IfStatement(IfStatement),
    SwitchStatement(SwitchStatement),
    SwitchCase(SwitchCase),
    ThrowStatement(ThrowStatement),
    TryStatement(TryStatement),
    CatchClause(CatchClause),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ForStatement(ForStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarator(VariableDeclarator),
    ThisExpression(ThisExpression),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
    Property(Property),
    FunctionExpression(FunctionExpression),
    UnaryExpression(UnaryExpression),
    UnaryOperator(UnaryOperator),
    BinaryExpression(BinaryExpression),
    BinaryOperator(BinaryOperator),
    AssignmentExpression(AssignmentExpression),
    AssignmentOperator(AssignmentOperator),
    LogicalExpression(LogicalExpression),
    LogicalOperator(LogicalOperator),
    MemberExpression(MemberExpression),
    ConditionalExpression(ConditionalExpression),
    CallExpression(CallExpression),
    NewExpression(NewExpression),
    SequenceExpression(SequenceExpression),
    // es6
    Super(Super),
    SpreadElement(SpreadElement),
    ArrowFunctionExpression(ArrowFunctionExpression),
    YieldExpression(YieldExpression),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplateExpression(TaggedTemplateExpression),
    TemplateElement(TemplateElement),
    AssignmentProperty(AssignmentProperty),
    ObjectPattern(ObjectPattern),
    ArrayPattern(ArrayPattern),
    RestElement(RestElement),
    AssignmentPattern(AssignmentPattern),
    Class(Class),
    ClassBody(ClassBody),
    MethodDefinition(MethodDefinition),
    ClassDeclaration(ClassDeclaration),
    ClassExpression(ClassExpression),
    MetaProperty(MetaProperty),
    ModuleDeclaration(ModuleDeclaration),
    ModuleSpecifier(ModuleSpecifier),
    ImportDeclaration(ImportDeclaration),
    ImportSpecifier(ImportSpecifier),
    ImportDefaultSpecifier(ImportDefaultSpecifier),
    ImportNamespaceSpecifier(ImportNamespaceSpecifier),
    ExportNamedDeclaration(ExportNamedDeclaration),
    ExportSpecifier(ExportSpecifier),
    ExportDefaultDeclaration(ExportDefaultDeclaration),
    ExportAllDeclaration(ExportAllDeclaration),
}

#[derive(Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Identifier {
    name: String,
}

#[derive(Deserialize, Clone)]
pub struct Program {
    body: Vec<ProgramBody>,
}

#[derive(Deserialize, Clone)]
pub enum ProgramBody {
    ModuleDeclaration(ModuleDeclaration),
    Statement(Statement),
}

// TODO: Change this into trait?
#[derive(Deserialize, Clone)]
pub struct Function {
    id: Option<Identifier>,
    params: Vec<Pattern>,
    // body: FunctionBody,
}

#[derive(Deserialize, Clone)]
pub struct ExpressionStatement {
    expression: Expression,
}

#[derive(Deserialize, Clone)]
pub struct Directive {
    expression: Literal,
    directive: String,
}

#[derive(Deserialize, Clone)]
pub struct BlockStatement {
    pub body: Vec<Node>,
}

// solitary semicolon
#[derive(Deserialize, Clone)]
pub struct EmptyStatement {}

// solitary semicolon
#[derive(Deserialize, Clone)]
pub struct DebuggerStatement {}

#[derive(Deserialize, Clone)]
pub struct WithStatement {
    object: Expression,
    body: Statement,
}

#[derive(Deserialize, Clone)]
pub struct ReturnStatement {
    pub argument: Option<Expression>,
}

#[derive(Deserialize, Clone)]
pub struct LabeledStatement {
    label: Identifier,
    body: Statement,
}

#[derive(Deserialize, Clone)]
pub struct BreakStatement {
    label: Option<Identifier>,
}

#[derive(Deserialize, Clone)]
pub struct ContinueStatement {
    label: Option<Identifier>,
}

#[derive(Deserialize, Clone)]
pub struct IfStatement {
    test: Expression,
    consequent: Statement,
    alternate: Option<Statement>,
}

#[derive(Deserialize, Clone)]
pub struct SwitchStatement {
    discriminant: Expression,
    cases: Vec<SwitchCase>,
}

#[derive(Deserialize, Clone)]
pub struct SwitchCase {
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

#[derive(Deserialize, Clone)]
pub struct ThrowStatement {
    argument: Expression,
}

#[derive(Deserialize, Clone)]
pub struct TryStatement {
    block: BlockStatement,
    handler: Option<CatchClause>,
    finalizer: Option<BlockStatement>,
}

#[derive(Deserialize, Clone)]
pub struct CatchClause {
    param: Pattern,
    body: BlockStatement,
}

#[derive(Deserialize, Clone)]
pub struct WhileStatement {
    test: Expression,
    body: Statement,
}

#[derive(Deserialize, Clone)]
pub struct DoWhileStatement {
    body: Statement,
    test: Expression,
}

#[derive(Deserialize, Clone)]
pub struct ForStatement {
    #[serde(flatten)]
    init: ForInitValue,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Statement,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ForInitValue {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Null,
}

#[derive(Deserialize, Clone)]
pub struct ForInStatement {
    #[serde(flatten)]
    left: ForInLeftValue,
    right: Expression,
    body: Statement,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ForInLeftValue {
    VariableDeclaration(VariableDeclaration),
    Pattern(Pattern),
}

#[derive(Deserialize, Clone)]
pub struct FunctionDeclaration {
    id: Identifier,
    params: Vec<Pattern>,
    body: FunctionBody,
    generator: bool,
}

#[derive(Deserialize, Clone)]
pub struct FunctionExpression {
    id: Option<Identifier>,
    params: Vec<Pattern>,
    body: FunctionBody,
    generator: bool,
}

#[derive(Deserialize, Clone)]
pub struct ArrowFunctionExpression {
    id: Option<Identifier>,
    params: Vec<Pattern>,
    #[serde(flatten)]
    body: ArrowFunctionBody,
    generator: bool, // false
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ArrowFunctionBody {
    FunctionBody(FunctionBody),
    Expression(Expression),
}

#[derive(Deserialize, Clone)]
pub struct VariableDeclaration {
    declarations: Vec<VariableDeclarator>,
    kind: String, // "var" | "let" | "const"
}

#[derive(Deserialize, Clone)]
pub struct VariableDeclarator {
    id: Identifier,
    init: Option<Expression>,
}

#[derive(Deserialize, Clone)]
pub struct ArrayExpression {
    #[serde(flatten)]
    elements: Vec<ArrayElements>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ArrayElements {
    Expression(Expression),
    SpreadElement(SpreadElement),
    Null,
}

#[derive(Deserialize, Clone)]
pub struct ObjectExpression {
    properties: Vec<Property>,
}

#[derive(Deserialize, Clone)]
pub struct Property {
    key: PropertyKey,
    value: Expression,
    kind: String, // "init" | "get" | "set"
    method: bool,
    shorthand: bool,
    computed: bool,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PropertyKey {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Deserialize, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    prefix: bool, // Not in use, this is to distinguish from postfix operators (++, --)
    pub argument: Expression,
}

#[derive(Deserialize, Clone)]
pub enum UnaryOperator {
    #[serde(alias = "+")]
    Minus,
    #[serde(alias = "-")]
    Plus,
    #[serde(alias = "!")]
    Bang,
    #[serde(alias = "typeof")]
    TypeOf,
    #[serde(alias = "void")]
    Void,
    #[serde(alias = "delete")]
    Delete,
}

#[derive(Deserialize, Clone)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Deserialize, Clone)]
pub enum BinaryOperator {
    #[serde(alias = "==")]
    EqEq,
    #[serde(alias = "!=")]
    BangEq,
    #[serde(alias = "===")]
    EqEqEq,
    #[serde(alias = "!==")]
    BangEqEq,
    #[serde(alias = "<")]
    Lt,
    #[serde(alias = "<=")]
    Leq,
    #[serde(alias = ">")]
    Gt,
    #[serde(alias = ">=")]
    Geq,
    #[serde(alias = "<<")]
    LtLt,
    #[serde(alias = ">>")]
    GtGt,
    #[serde(alias = ">>>")]
    GtGtGt,
    #[serde(alias = "+")]
    Plus,
    #[serde(alias = "-")]
    Minus,
    #[serde(alias = "*")]
    Mult,
    #[serde(alias = "/")]
    Div,
    #[serde(alias = "%")]
    Mod,
    #[serde(alias = "|")]
    Pipe,
    #[serde(alias = "^")]
    Caret,
    #[serde(alias = "&")]
    And,
    #[serde(alias = "in")]
    In,
    #[serde(alias = "instanceof")]
    Instanceof,
}

#[derive(Deserialize, Clone)]
pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: Pattern,
    right: Expression,
}

#[derive(Deserialize, Clone)]
pub enum AssignmentOperator {
    #[serde(alias = "=")]
    Eq,
    #[serde(alias = "+=")]
    PlusEq,
    #[serde(alias = "-=")]
    MinusEq,
    #[serde(alias = "*=")]
    MultEq,
    #[serde(alias = "/=")]
    DivEq,
    #[serde(alias = "%=")]
    ModEq,
    #[serde(alias = "<<=")]
    LtLtEq,
    #[serde(alias = ">>=")]
    GtGtEq,
    #[serde(alias = ">>>=")]
    GtGtGtEq,
    #[serde(alias = "|=")]
    PipeEq,
    #[serde(alias = "^=")]
    CaretEq,
    #[serde(alias = "&=")]
    AndEq,
}

#[derive(Deserialize, Clone)]
pub struct AssignmentProperty {
    value: Pattern,
    kind: String, // "init"
    method: bool, // false
}

#[derive(Deserialize, Clone)]
pub struct LogicalExpression {
    operator: LogicalOperator,
    left: Expression,
    right: Expression,
}

#[derive(Deserialize, Clone)]
pub enum LogicalOperator {
    #[serde(alias = "||")]
    Or,
    #[serde(alias = "&&")]
    And,
}

#[derive(Deserialize, Clone)]
pub struct MemberExpression {
    object: Expression,
    property: Expression,
    computed: bool,
}

#[derive(Deserialize, Clone)]
pub struct ConditionalExpression {
    test: Expression,
    alternate: Expression,
    consequent: Expression,
}

#[derive(Deserialize, Clone)]
pub struct CallExpression {
    callee: MemberIdentifier,
    arguments: Vec<Expression>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MemberIdentifier {
    MemberExpression(MemberExpression),
    Expression(Expression),
    Super(Super),
    Identifier(Identifier),
}

#[derive(Deserialize, Clone)]
pub struct NewExpression {
    callee: Expression,
    #[serde(flatten)]
    arguments: Vec<NewExpressionArguments>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum NewExpressionArguments {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Deserialize, Clone)]
pub struct SequenceExpression {
    expressions: Vec<Node>,
}

#[derive(Deserialize, Clone)]
pub struct SpreadElement {
    argument: Expression,
}

#[derive(Deserialize, Clone)]
pub struct YieldExpression {
    argument: Option<Expression>,
    delegate: bool,
}

#[derive(Deserialize, Clone)]
pub struct TemplateLiteral {
    quasis: Vec<TemplateElement>,
    expressions: Vec<Expression>,
}

#[derive(Deserialize, Clone)]
pub struct TaggedTemplateExpression {
    tag: Expression,
    quasi: TemplateLiteral,
}

#[derive(Deserialize, Clone)]
pub struct TemplateElement {
    tail: bool,
    #[serde(flatten)]
    value: TemplateElementValue,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub struct TemplateElementValue {
    cooked: String,
    raw: String,
}

#[derive(Deserialize, Clone)]
pub struct ObjectPattern {
    properties: Vec<AssignmentProperty>,
}

#[derive(Deserialize, Clone)]
pub struct ArrayPattern {
    elements: Vec<Option<Pattern>>,
}

#[derive(Deserialize, Clone)]
pub struct RestElement {
    argument: Pattern,
}

#[derive(Deserialize, Clone)]
pub struct AssignmentPattern {
    left: Pattern,
    right: Expression,
}

#[derive(Deserialize, Clone)]
pub struct Class {
    id: Option<Identifier>,
    super_class: Option<Expression>,
    body: ClassBody,
}

#[derive(Deserialize, Clone)]
pub struct ClassBody {
    body: Vec<MethodDefinition>,
}

#[derive(Deserialize, Clone)]
pub struct MethodDefinition {
    key: Expression,
    value: FunctionExpression,
    kind: String, // "constructor" | "method" | "get" | "set"
    coputed: bool,
    r#static: bool,
}

#[derive(Deserialize, Clone)]
pub struct ClassDeclaration {
    id: Identifier,
    super_class: Option<Expression>,
    body: ClassBody,
}

#[derive(Deserialize, Clone)]
pub struct MetaProperty {
    meta: Identifier,
    property: Identifier,
}

#[derive(Deserialize, Clone)]
pub struct ModuleSpecifier {
    local: Identifier,
}

#[derive(Deserialize, Clone)]
pub struct ImportDeclaration {
    #[serde(flatten)]
    specifiers: ImportDeclarationSpecifiers,
    source: Literal,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ImportDeclarationSpecifiers {
    ImportSpecifier(ImportSpecifier),
    ImportDefaultSpecifier(ImportDefaultSpecifier),
    ImportNamespaceSpecifier(ImportNamespaceSpecifier),
}

#[derive(Deserialize, Clone)]
pub struct ImportSpecifier {
    imported: Identifier,
}

#[derive(Deserialize, Clone)]
pub struct ExportNamedDeclaration {
    declaration: Option<Declaration>,
    specifiers: Vec<ExportSpecifier>,
    source: Option<Literal>,
}

#[derive(Deserialize, Clone)]
pub struct ExportSpecifier {
    exported: Identifier,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "declaration")]
pub enum ExportDefaultDeclaration {
    FunctionDeclaration(FunctionDeclaration),
    AnonymousDefaultExportedClassDeclaration,
    AnonymousDefaultExportedFunctionDeclaration,
}

#[derive(Deserialize, Clone)]
pub struct ExportAllDeclaration {
    source: Literal,
}

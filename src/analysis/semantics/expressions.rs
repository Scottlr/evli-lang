use super::syntaxvisitor::SyntaxVisitor;

trait SyntaxExpression {
    fn accept(syntax_visitor: SyntaxVisitor);
}

struct ClassDeclaraionExpression;
impl SyntaxExpression for ClassDeclaraionExpression {
    fn accept(syntax_visitor: SyntaxVisitor){

    }
}

struct LetExpression;
impl SyntaxExpression for LetExpression {
    fn accept(syntax_visitor: SyntaxVisitor){

    }
}
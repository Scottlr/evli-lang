
pub trait SyntaxVisitor {
    fn evaluate_let(&mut self, node: SyntaxNode);
    fn evaluate_openbrace(&mut self, node: SyntaxNode);
} 

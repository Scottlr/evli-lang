use super::token::Token;

struct SyntaxNode {
    token: Token,
    leftNode: SyntaxTreeNode,
    rightNode: SyntaxTreeNode
}

struct AbstractSyntaxTree {
    rootNode: SyntaxNode;
}

pub trait SyntaxVisitor {
    fn evaluate_let(&mut self, node: SyntaxNode);
    fn evaluate_openbrace(&mut self, node: SyntaxNode);
} 

impl AbstractSyntaxTree {
    pub fn parse(self, tokens: Vec<Token>) -> AbstractSyntaxTree {
        AbstractSyntaxTree
    }
}

impl SyntaxVisitor for AbstractSyntaxTree {
    
}
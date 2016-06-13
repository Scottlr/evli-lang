use super::token::Token;

struct SyntaxNode {
    token: Token,
    leftNode: SyntaxTreeNode,
    rightNode: SyntaxTreeNode
}

struct AbstractSyntaxTree {
    rootNode: SyntaxNode;
}


import { Node } from "acorn";

export interface BlockNode extends Node {
	body: Node[];
}

export interface ExpressionNode extends Node {
	expression: Node;
}

export interface BinaryExpressionNode extends Node {
	left: Node;
	right: Node;
	operator: string;
}

export interface LiteralExpressionNode extends Node {
	raw: string;
}

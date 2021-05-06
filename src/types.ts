import { Node } from "acorn";

export type DeclarationKind = 'const' | 'let' | 'var'

export type Env = {
	[key: string]: {
		value: any;
		kind: DeclarationKind;
	};
};
export interface BlockNode extends Node {
	body: Node[];
}

export interface ExpressionNode extends Node {
	expression: Node;
}

export interface UnaryExpressionNode extends Node {
	operator: string;
	argument: Node;
}

export interface BinaryExpressionNode extends Node {
	left: Node;
	right: Node;
	operator: string;
}

export interface LogicalExpressionNode extends Node {
	left: Node;
	right: Node;
	operator: string;
}

export interface LiteralExpressionNode extends Node {
	raw: string;
	value: number | string | boolean | undefined | null;
}

export interface VariableDeclarationNode extends Node {
	declarations: VariableDeclaratorNode[];
	kind: DeclarationKind;
}

export interface VariableDeclaratorNode extends Node {
	id: IdentifierNode;
	init: Node;
}

export interface IdentifierNode extends Node {
	name: string;
}

export interface AssignmentExpressionNode extends Node {
	operator: string;
	left: IdentifierNode;
	right: Node;
}

import { Node } from "acorn";

export type DeclarationKind = "const" | "let" | "var";

export type EnvironmentVariable = {
	value: any;
	kind: DeclarationKind;
};

export type Env = {
	[key: string]: EnvironmentVariable | Env | undefined;
	"^parent"?: Env; // This notation is a valid object key, and allows us to safely point to a parent object
	// and allows us to use a "parent" variable name in the environment
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
	left: IdentifierNode | MemberExpressionNode;
	right: Node;
}

export interface CallExpressionNode extends Node {
	callee: IdentifierNode | MemberExpressionNode; // Identifier or Member Expression
	arguments: Node[];
	optional: boolean;
}

export interface MemberExpressionNode extends Node {
	object: IdentifierNode;
	property: Node;
	computed: boolean;
	optional: boolean;
}

export interface ObjectExpressionNode extends Node {
	properties: PropertyNode[];
}

export interface PropertyNode extends Node {
	method: boolean;
	shorthand: false; // refers to shorthands like { something }
	computed: false; // if the key is a variable { [b] : ... }
	key: IdentifierNode | LiteralExpressionNode;
	value: Node;
	kind: "init";
}

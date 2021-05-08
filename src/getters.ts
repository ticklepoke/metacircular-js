import { Node } from "acorn";

export function isReturnValue(node: Node): boolean {
	return node.type === "ReturnValue";
}

export function isIdentifier(node: Node): boolean {
	return node.type === "Identifier";
}

export function isMemberExpression(node: Node): boolean {
	return node.type === "MemberExpression";
}

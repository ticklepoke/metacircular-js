import { Node } from "acorn";

export function isReturnValue(node: Node): boolean {
	return node.type === "ReturnValue";
}

export function isNumeric(str: string): boolean {
	return !isNaN(Number(str));
}

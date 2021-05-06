import { Node } from "acorn";

export function isReturnValue(node: Node): boolean {
	return node.type === "ReturnValue";
}


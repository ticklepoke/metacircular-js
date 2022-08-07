import { Node, Options, parse as acornParse } from "acorn";

export function parse(sourceCode: string): Node {
	const acornOptions: Options = {
		ecmaVersion: "latest",
	};

	const ast: Node = acornParse(sourceCode, acornOptions);

	// Change top level program to a block statement
	ast.type = "BlockStatemenet";

	return ast;
}

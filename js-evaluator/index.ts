import { Node, Options, parse } from "acorn";
import * as fs from "fs";

import { evaluate } from "@src/evaluator";

function main() {
	const args = process.argv;

	if (args.length !== 3 || !args[2].includes(".js")) {
		console.error("Usage: yarn [start|dev] <filename.js>");
		process.exit(1);
	}

	const filename = args[2];

	let SOURCE_CODE;
	try {
		SOURCE_CODE = fs.readFileSync(filename, { encoding: "utf-8" });
	} catch (e) {
		console.error("Unable to read file");
	}

	if (!SOURCE_CODE) {
		console.error("NO SOURCE CODE");
		process.exit(1);
	}

	const acornOptions: Options = {
		ecmaVersion: "latest",
	};

	const ast: Node = parse(SOURCE_CODE, acornOptions);

	// Change top level program to a block statement
	ast.type = "BlockStatement";

	const env = {};

	try {
		const evalOutput = evaluate(ast, env);
		console.log("Evaluation Result: ", evalOutput);
	} catch (err) {
		console.error("Evaluation Error: ", (err as Error).message);
	}
}

main();

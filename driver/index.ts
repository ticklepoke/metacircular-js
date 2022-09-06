import { parse } from "parser";
import * as fs from "fs";

import { loadWasm } from "./src/loadWasm";

// TODO support repl and cli
async function main() {
	const args = process.argv;
	const { DEBUG } = process.env;

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

	const ast = parse(SOURCE_CODE);
	const serializedAst = JSON.stringify(ast);

	try {
		DEBUG && console.log(`Parsed AST:\n\n${serializedAst}\n`);
		// evaluate using rust
		const wasm = await loadWasm();
		const x = wasm.evaluate(serializedAst);
		console.log(`Evaluator Result: ${x}`);
	} catch (e) {
		console.error("Error evaluating", e);
		process.exit(1);
	}
}

main();

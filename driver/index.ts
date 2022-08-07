import { parse } from "parser";
import * as fs from "fs";

import { loadWasm } from "./src/loadWasm";

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

	const ast = parse(SOURCE_CODE);

	// evaluate using rust
	loadWasm().then(wasm => {
		wasm.give("hello");
	});
}

main();

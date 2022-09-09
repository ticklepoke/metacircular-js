import { parse } from "parser";
import * as fs from "fs";

import { evaluate } from "./evaluator";

export async function syntheticCli(args: string[]) {
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

	await evaluate(serializedAst);
}

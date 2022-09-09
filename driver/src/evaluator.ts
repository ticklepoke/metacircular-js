import { loadWasm } from "./loadWasm";

export async function evaluate(serializedAst: string) {
	try {
		const { DEBUG } = process.env;
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

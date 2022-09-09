import { parse } from "parser";
import repl, { Recoverable } from "repl";
import { evaluate } from "./evaluator";

export function syntheticRepl() {
	repl.start({
		prompt: "meta-js>",
		eval: async (src, context, file, cb) => {
			try {
				const ast = parse(src);
				const serializedAst = JSON.stringify(ast);
				await evaluate(serializedAst);
			} catch (e) {
				cb(new Recoverable(new Error()), null);
			}
		},
	});
}

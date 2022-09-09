#!/usr/bin/env node

import { syntheticCli } from "./src/cli";
import { syntheticRepl } from "./src/repl";

async function main() {
	const args = process.argv;

	if (args.length === 2) {
		syntheticRepl();
	} else if (args.length === 3 && args[2].includes(".js")) {
		await syntheticCli(args);
	} else {
		console.error("Usage: yarn [start|dev] <filename.js>");
		process.exit(1);
	}
}

main();

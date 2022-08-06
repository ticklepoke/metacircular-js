"use strict";
exports.__esModule = true;
var acorn_1 = require("acorn");
var fs = require("fs");
var evaluator_1 = require("@src/evaluator");
function main() {
    var args = process.argv;
    if (args.length !== 3 || !args[2].includes(".js")) {
        console.error("Usage: yarn [start|dev] <filename.js>");
        process.exit(1);
    }
    var filename = args[2];
    var SOURCE_CODE;
    try {
        SOURCE_CODE = fs.readFileSync(filename, { encoding: "utf-8" });
    }
    catch (e) {
        console.error("Unable to read file");
    }
    if (!SOURCE_CODE) {
        console.error("NO SOURCE CODE");
        process.exit(1);
    }
    var acornOptions = {
        ecmaVersion: "latest"
    };
    var ast = (0, acorn_1.parse)(SOURCE_CODE, acornOptions);
    // Change top level program to a block statement
    ast.type = "BlockStatement";
    var env = {};
    try {
        var evalOutput = (0, evaluator_1.evaluate)(ast, env);
        console.log("Evaluation Result: ", evalOutput);
    }
    catch (err) {
        console.error("Evaluation Error: ", err.message);
    }
}
main();

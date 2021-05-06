/* eslint-disable @typescript-eslint/no-explicit-any */
import { Node, Options, parse } from "acorn";

import { isReturnValue } from "@src/getters";
import { getPrimitiveBinaryFunction, getPrimitiveUnaryFunction } from "@src/primitives";
import {
	BinaryExpressionNode,
	BlockNode,
	ExpressionNode,
	LiteralExpressionNode,
	LogicalExpressionNode,
	UnaryExpressionNode,
} from "@src/types";

const SOURCE_CODE = `
typeof 1
`;

const acornOptions: Options = {
	ecmaVersion: "latest",
};

const ast: Node = parse(SOURCE_CODE, acornOptions);

// Change top level program to a block statement
ast.type = "BlockStatement";

console.log(evaluate(ast));

function evaluate(node: Node): any {
	const { type } = node;
	// console.log(node)

	// Handling clearly labelled nodes
	switch (type) {
		case "Program":
			// noop
			break;

		case "BlockStatement":
			return evalBlock(node as BlockNode);

		case "ExpressionStatement":
			return evaluate((node as ExpressionNode).expression);

		case "UnaryExpression":
			return evalUnaryExpression(node as UnaryExpressionNode);

		case "BinaryExpression":
			return evalBinaryExpression(node as BinaryExpressionNode);

		case "LogicalExpression":
			return evalLogicalExpression(node as LogicalExpressionNode);

		case "Literal":
			return evalLiteral(node as LiteralExpressionNode);

		case "VariableDeclaration":
			console.log("here");
			break;

		default:
			break;
	}
}

function apply(fn: (...args: any[]) => any, args: any[]) {
	return fn(...args);
}

// Eval Handlers
function evalBlock(node: BlockNode) {
	const body = node.body;
	return evalSequence(body);
}

function evalSequence(nodes: Array<Node>): any | void {
	if (nodes.length === 0) {
		return undefined;
	} else if (nodes.length === 1) {
		return evaluate(nodes[0]);
	} else {
		const firstStatementValue = evaluate(nodes[0]);
		// if is return value, just return
		if (isReturnValue(firstStatementValue)) {
			console.log("return value found");
			return firstStatementValue;
		} else {
			nodes.shift();
			return evalSequence(nodes);
		}
	}
}

function evalUnaryExpression(node: UnaryExpressionNode) {
	const { operator, argument } = node;
	const operatorFunction = getPrimitiveUnaryFunction(operator);
	const argValue = evaluate(argument);
	if (operatorFunction) {
		return apply(operatorFunction, [argValue]);
	}
}

function evalBinaryExpression(node: BinaryExpressionNode) {
	const { left, right, operator } = node;
	const primitiveFunction = getPrimitiveBinaryFunction(operator);
	const leftValue = evaluate(left);
	const rightValue = evaluate(right);
	if (primitiveFunction) {
		return apply(primitiveFunction, [leftValue, rightValue]);
	}
}

function evalLogicalExpression(node: LogicalExpressionNode) {
	const { left, right, operator } = node;
	const primitiveFunction = getPrimitiveBinaryFunction(operator);
	const leftValue = evaluate(left);
	const rightValue = evaluate(right);
	if (primitiveFunction) {
		return apply(primitiveFunction, [leftValue, rightValue]);
	}
}

function evalLiteral(node: LiteralExpressionNode) {
	const { value } = node;
	return value;
}

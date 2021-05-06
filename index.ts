/* eslint-disable @typescript-eslint/no-explicit-any */
import { Node, Options, parse } from "acorn";

import { isReturnValue } from "@src/getters";
import { getPrimitiveBinaryFunction, getPrimitiveUnaryFunction } from "@src/primitives";
import {
	AssignmentExpressionNode,
	BinaryExpressionNode,
	BlockNode,
	DeclarationKind,
	Env,
	EnvironmentVariable,
	ExpressionNode,
	IdentifierNode,
	LiteralExpressionNode,
	LogicalExpressionNode,
	UnaryExpressionNode,
	VariableDeclarationNode,
	VariableDeclaratorNode,
} from "@src/types";

const SOURCE_CODE = `
let b = 1
{
b = 2;
b;
}
`;

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

function evaluate(node: Node, env: Env): any {
	const { type } = node;

	// Handling clearly labelled nodes
	switch (type) {
		case "Program":
			// noop
			break;

		case "BlockStatement":
			return evalBlock(node as BlockNode, env);

		case "ExpressionStatement":
			return evaluate((node as ExpressionNode).expression, env);

		case "UnaryExpression":
			return evalUnaryExpression(node as UnaryExpressionNode, env);

		case "BinaryExpression":
			return evalBinaryExpression(node as BinaryExpressionNode, env);

		case "LogicalExpression":
			return evalLogicalExpression(node as LogicalExpressionNode, env);

		case "Literal":
			return evalLiteral(node as LiteralExpressionNode);

		case "VariableDeclaration":
			return evalVariableDeclaration(node as VariableDeclarationNode, env);

		case "Identifier":
			return evalIdentifier(node as IdentifierNode, env);

		case "AssignmentExpression":
			return evalAssignmentExpression(node as AssignmentExpressionNode, env);

		default:
			break;
	}
}

function apply(fn: (...args: any[]) => any, args: any[]) {
	return fn(...args);
}

// Eval Handlers
function evalBlock(node: BlockNode, env: Env) {
	const body = node.body;
	const innerScope: Env = {
		parent: env,
	};
	return evalSequence(body, innerScope);
}

function evalSequence(nodes: Array<Node>, env: Env): any | void {
	if (nodes.length === 0) {
		return undefined;
	} else if (nodes.length === 1) {
		return evaluate(nodes[0], env);
	} else {
		const firstStatementValue = evaluate(nodes[0], env);
		// if is return value, just return
		if (firstStatementValue && isReturnValue(firstStatementValue)) {
			console.log("return value found");
			return firstStatementValue;
		} else {
			nodes.shift();
			return evalSequence(nodes, env);
		}
	}
}

function evalUnaryExpression(node: UnaryExpressionNode, env: Env) {
	const { operator, argument } = node;
	const operatorFunction = getPrimitiveUnaryFunction(operator);
	const argValue = evaluate(argument, env);
	if (operatorFunction) {
		return apply(operatorFunction, [argValue]);
	}
}

function evalBinaryExpression(node: BinaryExpressionNode, env: Env) {
	const { left, right, operator } = node;
	const primitiveFunction = getPrimitiveBinaryFunction(operator);
	const leftValue = evaluate(left, env);
	const rightValue = evaluate(right, env);
	if (primitiveFunction) {
		return apply(primitiveFunction, [leftValue, rightValue]);
	}
}

function evalLogicalExpression(node: LogicalExpressionNode, env: Env) {
	const { left, right, operator } = node;
	const primitiveFunction = getPrimitiveBinaryFunction(operator);
	const leftValue = evaluate(left, env);
	const rightValue = evaluate(right, env);
	if (primitiveFunction) {
		return apply(primitiveFunction, [leftValue, rightValue]);
	}
}

function evalLiteral(node: LiteralExpressionNode) {
	const { value } = node;
	return value;
}

function evalVariableDeclaration(node: VariableDeclarationNode, env: Env) {
	const { declarations, kind } = node;
	declarations.forEach(decl => evalVariableDeclarator(decl, kind, env));
}

function evalVariableDeclarator(node: VariableDeclaratorNode, kind: DeclarationKind, env: Env) {
	const {
		id: { name },
		init,
	} = node;
	const initValue = evaluate(init, env);

	if (env[name]) {
		throw new Error("Duplicate variable declaration");
	}

	env[name] = { value: initValue, kind };
}

function evalIdentifier(node: IdentifierNode, env: Env) {
	const { name } = node;

	function lookupParentScope(target: string, env?: Env): any {
		if (!env) {
			throw new Error("Reference to undeclared variable");
		} else if (env[target]) {
			return env[target];
		} else {
			return lookupParentScope(target, env.parent);
		}
	}

	if (env[name]) {
		return env[name]?.value;
	}
	const lookupValue = lookupParentScope(name, env);
	if (lookupValue) {
		return lookupValue.value;
	}
}

// TODO: shadowed vars should updated and remain in the outer scope
function evalAssignmentExpression(node: AssignmentExpressionNode, env: Env) {
	const {
		left: { name },
		right,
		operator,
	} = node;

	function lookupParentScope(lookupTarget: string, env?: Env): any {
		if (!env) {
			throw new Error("Assignment to undeclared variable");
		} else if (env[lookupTarget]) {
			if ((env[lookupTarget] as EnvironmentVariable).kind === "const") {
				throw new Error("Assignment to const variable");
			}
			env[lookupTarget] = { value: rightValue, kind: (env[lookupTarget] as EnvironmentVariable).kind };
		} else {
			return lookupParentScope(lookupTarget, env.parent);
		}
	}

	if (operator !== "=") {
		throw new Error("Unsupported assignment operator");
	}

	if (!env[name]) {
		return lookupParentScope(name, env);
	}

	if ((env[name] as EnvironmentVariable).kind === "const") {
		throw new Error("Assignment to const variable");
	}

	const rightValue = evaluate(right, env);

	env[name] = { value: rightValue, kind: (env[name] as EnvironmentVariable).kind };
}

export function getPrimitiveBinaryFunction(operator: string): ((a: any, b: any) => any) | null {
	switch (operator) {
		case "+":
			return (a: any, b: any) => a + b;
		case "-":
			return (a: any, b: any) => a - b;
		case "*":
			return (a: any, b: any) => a * b;
		case "/":
			return (a: any, b: any) => a / b;
		case "%":
			return (a: any, b: any) => a % b;
		case "<<":
			return (a: any, b: any) => a << b;
		case ">>":
			return (a: any, b: any) => a >> b;
		case ">>>":
			return (a: any, b: any) => a >>> b;
		case "<":
			return (a: any, b: any) => a < b;
		case ">":
			return (a: any, b: any) => a > b;
		case "<=":
			return (a: any, b: any) => a <= b;
		case ">=":
			return (a: any, b: any) => a >= b;
		case "instanceof":
			return (a: any, b: any) => a instanceof b;
		case "in":
			return (a: any, b: any) => a in b;
		case "==":
			return (a: any, b: any) => a == b;
		case "===":
			return (a: any, b: any) => a === b;
		case "!=":
			return (a: any, b: any) => a != b;
		case "!==":
			return (a: any, b: any) => a !== b;
		case "&":
			return (a: any, b: any) => a & b;
		case "^":
			return (a: any, b: any) => a ^ b;
		case "|":
			return (a: any, b: any) => a | b;
		case "&&":
			return (a: any, b: any) => a && b;
		case "||":
			return (a: any, b: any) => a || b;
		default:
			return null;
	}
}

export function getPrimitiveUnaryFunction(operator: string): ((a: any) => any) | null {
	switch (operator) {
		case "!":
			return (a: any) => !a;
		case "-":
			return (a: any) => -a;
		case "+":
			return (a: any) => +a;
		case "~":
			return (a: any) => ~a;
		case "typeof":
			return (a: any) => typeof a;
		default:
			return null;
	}
}

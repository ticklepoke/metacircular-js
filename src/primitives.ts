export function getPrimitiveFunction(operator: string): ((a: any, b: any) => any) | null {
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

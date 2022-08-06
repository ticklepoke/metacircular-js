<p align="center">
  <img width="400" height="569" src="/assets/batman-joker.jpg" alt="you complete me">
</p>

# Metacircular-JS

A metacircular evaluator for Javascript, based on the SICP textbook.

The metacircular evaluator is a story of two recursively mutual functions: `evaluate()` and `apply()`. `evalute()` recursively walks and interprets an AST, while `apply()` invokes function calls.

**Note**: A rust based interpreter is currently being built. The Javascript evaluator can still be found in `./js-evaluator`.

<p align="center">
  <img src="/assets/overview.png" alt="overview">
</p>

## Dynamic Semantics

The evaluator adopts a "while there is work to do, do it" philosophy. This results in a recursive evaluation of the AST. In the following example, we recursively "work" on the expression until there is no more work that can be done:

```
    1 / (1 + (2 * 3))
=>  1 / (1 + 6)
=>  1 / 7
```

## Invoking the host language

The host language is used to support features of the interpreted language. This is easier to implement when the host language is similar to the interpreted language. For example, when evaluating and applying an addition `BinaryExpression` in the interpreted language, we can apply it using the underlying host runtime:

```js
const operator = '+';

const primitiveBinaryFunctions = {
	'+': (a, b) => a + b,
	// ... other binary functions
};

const callFn = primitiveBinaryFunctions[operator];

apply(callFn, a, b);
```

## Environment Frames

> The terms "environment frame" and "environment object" are used interchangeably in this project.

Without a formal memory model, we maintain variables by passing a context object known as the envrionment (`env`):

```js
{
    ^parent: // parent frame

    // variables defined in the scope
    a: 1,
    b: 2,
}
```

When creating inner scopes, we wrap the existing environment frame with an outer environment frame. Each nested lexical scope corresponds to a layer of outer frame in the environment object. The inner most environment object corresponds to the global scope.

The parent environment object is accessed using `env["^parent"]`. This was a deliberate choice as it is a valid object key in typescript, while it is invalid as a variable name, making it a safe choice to use as a reference to the outer scope. This also frees up the `parent` keyword to be a valid variable name: `let parent = 1`. All valid Javascript variable names can thus be supported using this evaluator.

## Lexical Scope

Lexical scope is supported in this evaluator. The environemnt object can be recursively traversed to search for a variable declaration if the current scope does not contain the required variable. One simply has to follow the `^parent` reference to the outer scope.

## Shadowing

Nested scopes are able to "capture" variables from the outer scope:

```js
let a = 1;
{
	a = 2; // a becomes a variable in this scope
	console.log(a); // 2;
}
console.log(a); // 2;
```

Shadowing is thus implemented like a "capture". If a variable is referenced to in a particular scope, the corresponding environment frame is checked to see if a variable is already defined in its scope. If the variable does not exist in the current frame, the parent frames are checked recyrsively. The resulting variable frame would then be appended to the original environment frame.

## Closures

This approach of "capturing" variables allows closures to be supported, if a function is invoked in a different locations from where it was defined. The function execution would still be able to remember the variables from its original lexical scope.

## Semantics

The following langauge rules are currently supported.

```
Program     ::=     Block                                   program

Block       ::=     { Statement ... }                       block statement

Statement   ::=     const name = Expression;                constant declaration
            |       let name = Expression;                  variable declaration
            |       Block                                   block statement
            |       Expression;                             expression statement

Assignment  ::=     name = Expression                       variable assignment

Expression  ::=     number                                  number literal
            |       true | false                            boolean literal
            |       string                                  string literal
            |       Expression BinaryOperator Expression    binary operator combination
            |       UnaryOperator Expression                unary operator combination
            |       { ObjectKey: Expression }               object literal
            |       name.name = Expression                  object property assignment

ObjectKey   ::=     string | [ Expression ]

Expressions ::=     Expression (, Expression) ...           multiple expressions

UnaryOperator ::= ! | - | + | ~ | typeof

BinaryOperator ::= + | - | * | / | % | << | >> | >>> | < | > | <= | >=
                | instanceof | in | == | === | != | !== | & | ^ | | | && | || |
```

There are plans for the following rules to be supported.

```
Statement   ::=     function name (parameters) Block        function declaration
            |       return Expression                       return expression
            |       IfStatement                             conditional statement
            |       IfElseStatement                         conditional alternative statement

IfStatement ::=     if (Expression) Block

IfElseStatement ::= IfStatement else
                    (Block | IfStatement | IfElseStatement)

Expression  ::=     (parameters) => Expression | Block      arrow function
            |       Expression ? Expression : Expression    ternary conditional
            |       [ Expressions ]                         array literal
            |       Expression [ Expression ]               array access / object access
            |       Expression [ Expression ] = Expression  array assignment
            |       name.name                               object access

```

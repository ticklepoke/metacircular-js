<p align="center">
  <img width="400" height="569" src="/assets/batman-joker.jpg" alt="you complete me">
</p>

# Metacircular-JS

A metacircular evaluator for Javascript, based on the SICP textbook. 

The metacircular evaluator is a story of two recursively mutual functions: `evaluate()` and `apply()`. They recursively evaluate an AST and apply the function using an underlying language runtime. 

## Running

The project can be run by editing the `SOURCE_CODE` variable in `/index.ts`. Valid Javascript is supported according to the semantics below. There are future plans to support reading file inputs or a repl style input.

### Install dependencies

```sh
yarn install
```

### Run the evaluator
```sh
yarn dev
```

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
            |       { ObjectKey: Expression }               object literal
            |       name.name                               object access
            |       name.name = Expression                  object property assignment

ObjectKey   ::=     string | [ Expression ]                 

Expressions ::=     Expression (, Expression) ...           multiple expressions
```
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
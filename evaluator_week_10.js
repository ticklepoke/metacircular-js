// SICP JS 4.1.4

// credits: original record implementation by Thomas Tan

// functions from SICP JS 4.1.1

// Make minimal progress and ensure bookkeeping is done (e.g. defining bindings)
function evaluate(component, env) {
  return is_literal(component)
    ? literal_value(component)
    : is_property_name(component)
    ? property_name_value(component)
    : is_name(component)
    ? lookup_symbol_value(symbol_of_name(component), env)
    : is_application(component)
    ? apply(evaluate(function_expression(component), env),
            list_of_values(arg_expressions(component), env))
    : is_operator_combination(component)
    ? evaluate(operator_combination_to_application(component), env)
    : is_conditional(component)
    ? eval_conditional(component, env)
    : is_lambda_expression(component)
    ? make_function(
        lambda_parameter_symbols(component),
        lambda_body(component),
        env
      )
    : is_sequence(component)
    ? eval_sequence(sequence_statements(component), env)
    : is_block(component)
    ? eval_block(component, env)
    : is_return_statement(component)
    ? eval_return_statement(component, env)
    : is_function_declaration(component)
    ? evaluate(function_decl_to_constant_decl(component), env)
    : is_declaration(component)
    ? eval_declaration(component, env)
    : is_assignment(component)
    ? eval_assignment(component, env)
    : is_object_expression(component)
    ? eval_object_expression(component, env)
    : is_object_access(component)
    ? eval_object_access(component, env)
    : is_object_assignment(component)
    ? eval_object_assignment(component, env)
    : is_name_property(component)
    ? evaluate(make_name(property_name_value(component)), env)
    : is_property(component)
    ? list_ref(component, 1)
    : error(component, "unknown syntax -- evaluate");
}

function apply(fun, args) {
   if (is_primitive_function(fun)) {
      return apply_primitive_function(fun, args);
   } else if (is_compound_function(fun)) {
      const result = evaluate(function_body(fun),
                              extend_environment(
                                  function_parameters(fun),
                                  args,
                                  function_environment(fun)));
      return is_return_value(result)
             ? return_value_content(result)
             : undefined;
   } else {
      error(fun, "unknown function type -- apply");
   }
}


function list_of_values(exps, env) {
     return map(arg => evaluate(arg, env), exps);
}

function eval_conditional(component, env) {
  return is_truthy(evaluate(conditional_predicate(component), env))
    ? evaluate(conditional_consequent(component), env)
    : evaluate(conditional_alternative(component), env);
}

function eval_sequence(stmts, env) {
  if (is_empty_sequence(stmts)) {
    return undefined;
  } else if (is_last_statement(stmts)) {
    return evaluate(first_statement(stmts), env);
  } else {
    const first_stmt_value = evaluate(first_statement(stmts), env);
    if (is_return_value(first_stmt_value)) {
      return first_stmt_value;
    } else {
      return eval_sequence(rest_statements(stmts), env);
    }
  }
}

function list_of_unassigned(symbols) {
  return map((symbol_) => "*unassigned*", symbols);
}

function scan_out_declarations(component) {
  return is_sequence(component)
    ? accumulate(
        append,
        null,
        map(scan_out_declarations, sequence_statements(component))
      )
    : is_declaration(component)
    ? list(declaration_symbol(component))
    : null;
}

function eval_block(component, env) {
  const body = block_body(component);
  const locals = scan_out_declarations(body);
  const unassigneds = list_of_unassigned(locals);
  return evaluate(body, extend_environment(locals, unassigneds, env));
}

function eval_return_statement(component, env) {
  return make_return_value(evaluate(return_expression(component), env));
}

function eval_assignment(component, env) {
  const value = evaluate(assignment_value_expression(component), env);
  assign_symbol_value(assignment_symbol(component), value, env);
  return value;
}

function eval_declaration(component, env) {
    assign_symbol_value(declaration_symbol(component), 
                        evaluate(declaration_value_expression(component),
                                 env),
                        env);
    return undefined;
}

function eval_object_expression(component, env) {
  const prop_exprs = object_expression_properties(component);
  const props = map(
    (prop_expr) =>
      is_number(prop_expr) ? prop_expr : evaluate(prop_expr, env),
    prop_exprs
  );
  const val_exprs = object_expression_values(component);
  const vals = map((val_expr) => evaluate(val_expr, env), val_exprs);
  const record = make_record(props, vals);
  // display_list(record, "DEBUG eval_object_expression:");
  return record;
}

function eval_object_access(component, env) {
  const obj_expr = object_access_object(component);
  const prop_expr = object_access_property(component);
  const obj = evaluate(obj_expr, env);
  const prop = evaluate(prop_expr, env);
  if (!is_record(obj)) {
      error("expected record in object access");
  } else {
    const val = scan_pair_of_lists(
      record_props(obj),
      record_vals(obj),
      prop,
      () => undefined
    );
    // display_list(val, "DEBUG eval_object_access:");
    return val;
  }
}

function eval_object_assignment(component, env) {
  const obj_expr = object_assignment_object(component);
  const prop_expr = object_assignment_property(component);
  const val_expr = object_assignment_value_expression(component);
  const obj = evaluate(obj_expr, env);
  const prop = evaluate(prop_expr, env);
  const val = evaluate(val_expr, env);
  if (!is_record(obj)) {
    error("expected record in object access");
  } else {
    assign_pair_of_lists(
      record_props(obj),
      record_vals(obj),
      prop,
      () => undefined,
      val
    );
    // display_list(val, "DEBUG eval_object_access:");
    return val;
  }
}

// functions from SICP JS 4.1.2

function is_tagged_list(component, the_tag) {
  return is_pair(component) && head(component) === the_tag;
}

function is_literal(component) {
  return is_tagged_list(component, "literal");
}
function literal_value(component) {
  return head(tail(component));
}

function make_literal(value) {
  return list("literal", value);
}

function is_name(component) {
  return is_tagged_list(component, "name");
}

function make_name(symbol) {
  return list("name", symbol);
}
function is_property(component) {
  return is_tagged_list(component, "property");
}
      
function is_property_name(component) {
  if (is_tagged_list(component, "name")) {
      const first_char = char_at(symbol_of_name(component), 0);
      return first_char >= "A" && first_char <= "Z";
  } else {
      return false;
  }
}

function is_name_property(component) {
  if (is_tagged_list(component, "property")) {
      const first_char = char_at(symbol_of_name(component), 0);
      return first_char >= "a" && first_char <= "z";
  } else {
      return false;
  }
}

function property_name_value(component) {
    return head(tail(component));
}

function make_property(symbol) {
  return list("property", symbol);
}

function symbol_of_name(component) {
  return head(tail(component));
}

function is_assignment(component) {
  return is_tagged_list(component, "assignment");
}
function assignment_symbol(component) {
  return head(tail(head(tail(component))));
}
function assignment_value_expression(component) {
  return head(tail(tail(component)));
}

function is_declaration(component) {
  return (
    is_tagged_list(component, "constant_declaration") ||
    is_tagged_list(component, "variable_declaration") ||
    is_tagged_list(component, "function_declaration")
  );
}

function declaration_symbol(component) {
  return symbol_of_name(head(tail(component)));
}
function declaration_value_expression(component) {
  return head(tail(tail(component)));
}

function make_constant_declaration(name, value_expression) {
  return list("constant_declaration", name, value_expression);
}

function is_lambda_expression(component) {
  return is_tagged_list(component, "lambda_expression");
}
function lambda_parameter_symbols(component) {
  return map(symbol_of_name, head(tail(component)));
}
function lambda_body(component) {
  return head(tail(tail(component)));
}

function make_lambda_expression(parameters, body) {
  return list("lambda_expression", parameters, body);
}

function is_function_declaration(component) {
  return is_tagged_list(component, "function_declaration");
}
function function_declaration_name(component) {
  return list_ref(component, 1);
}
function function_declaration_parameters(component) {
  return list_ref(component, 2);
}
function function_declaration_body(component) {
  return list_ref(component, 3);
}
function function_decl_to_constant_decl(component) {
  return make_constant_declaration(
    function_declaration_name(component),
    make_lambda_expression(
      function_declaration_parameters(component),
      function_declaration_body(component)
    )
  );
}

function is_return_statement(component) {
  return is_tagged_list(component, "return_statement");
}
function return_expression(component) {
  return head(tail(component));
}

function is_conditional(component) {
  return (
    is_tagged_list(component, "conditional_expression") ||
    is_tagged_list(component, "conditional_statement")
  );
}
function conditional_predicate(component) {
  return list_ref(component, 1);
}
function conditional_consequent(component) {
  return list_ref(component, 2);
}
function conditional_alternative(component) {
  return list_ref(component, 3);
}

function is_sequence(stmt) {
  return is_tagged_list(stmt, "sequence");
}
function sequence_statements(stmt) {
  return head(tail(stmt));
}
function first_statement(stmts) {
  return head(stmts);
}
function rest_statements(stmts) {
  return tail(stmts);
}
function is_empty_sequence(stmts) {
  return is_null(stmts);
}
function is_last_statement(stmts) {
  return is_null(tail(stmts));
}

function is_block(component) {
  return is_tagged_list(component, "block");
}
function block_body(component) {
  return head(tail(component));
}
function make_block(statement) {
  return list("block", statement);
}

function is_operator_combination(component) {
  return (
    is_unary_operator_combination(component) ||
    is_binary_operator_combination(component)
  );
}
function is_unary_operator_combination(component) {
  return is_tagged_list(component, "unary_operator_combination");
}
function is_binary_operator_combination(component) {
  return is_tagged_list(component, "binary_operator_combination");
}
function operator_symbol(component) {
  return list_ref(component, 1);
}
function first_operand(component) {
  return list_ref(component, 2);
}
function second_operand(component) {
  return list_ref(component, 3);
}

function make_application(function_expression, argument_expressions) {
  return list("application", function_expression, argument_expressions);
}

function operator_combination_to_application(component) {
  const operator = operator_symbol(component);
  return is_unary_operator_combination(component)
    ? make_application(make_name(operator), list(first_operand(component)))
    : make_application(
        make_name(operator),
        list(first_operand(component), second_operand(component))
      );
}

function is_application(component) {
  return is_tagged_list(component, "application");
}
function function_expression(component) {
  return head(tail(component));
}
function arg_expressions(component) {
  return head(tail(tail(component)));
}

// object support

function is_object_expression(component) {
  return is_tagged_list(component, "object_expression");
}
function object_expression_properties(component) {
  return map(head, list_ref(component, 1));
}
function object_expression_values(component) {
  return map(tail, list_ref(component, 1));
}

function is_object_access(component) {
  return is_tagged_list(component, "object_access");
}
function object_access_object(component) {
  return list_ref(component, 1);
}
function object_access_property(component) {
  return list_ref(component, 2);
}

function is_object_assignment(component) {
  return is_tagged_list(component, "object_assignment");
}
function object_assignment_object(component) {
  return list_ref(list_ref(component, 1), 1);
}
function object_assignment_property(component) {
  return list_ref(list_ref(component, 1), 2);
}
function object_assignment_value_expression(component) {
  return list_ref(component, 2);
}

function property_name(component) {
  return list_ref(component, 1);
}

// functions from SICP JS 4.1.3

function is_truthy(x) {
  return is_boolean(x) ? x : error(x, "boolean expected, received");
}
function is_falsy(x) {
  return !is_truthy(x);
}

function make_function(parameters, body, env) {
  return list("compound_function", parameters, body, env);
}
function is_compound_function(f) {
  return is_tagged_list(f, "compound_function");
}
function function_parameters(f) {
  return list_ref(f, 1);
}
function function_body(f) {
  return list_ref(f, 2);
}
function function_environment(f) {
  return list_ref(f, 3);
}

function make_return_value(content) {
  return list("return_value", content);
}
function is_return_value(value) {
  return is_tagged_list(value, "return_value");
}
function return_value_content(value) {
  return head(tail(value));
}

function make_record(props, vals) {
  return list("record", props, vals);
}
function is_record(r) {
  return is_tagged_list(r, "record");
}
function record_props(r) {
  return list_ref(r, 1);
}
function record_vals(r) {
  return list_ref(r, 2);
}



function enclosing_environment(env) {
  return tail(env);
}
function first_frame(env) {
  return head(env);
}
const the_empty_environment = null;

function make_frame(symbols, values) {
  return pair(symbols, values);
}
function frame_symbols(frame) {
  return head(frame);
}
function frame_values(frame) {
  return tail(frame);
}

function extend_environment(symbols, vals, base_env) {
  return length(symbols) === length(vals)
    ? pair(make_frame(symbols, vals), base_env)
    : length(symbols) < length(vals)
    ? error(
        "too many arguments supplied: " +
          stringify(symbols) +
          ", " +
          stringify(vals)
      )
    : error(
        "too few arguments supplied: " +
          stringify(symbols) +
          ", " +
          stringify(vals)
      );
}

function scan_pair_of_lists(symbols, vals, symbol, not_found_cb) {
  return is_null(symbols)
    ? not_found_cb()
    : symbol === head(symbols)
    ? head(vals)
    : scan_pair_of_lists(tail(symbols), tail(vals), symbol, not_found_cb);
}

function assign_pair_of_lists(symbols, vals, symbol, not_found_cb, val) {
  if (is_null(symbols)) {
      return not_found_cb();
  } else if (symbol === head(symbols)) {
      set_head(vals, val);
  } else {
      return assign_pair_of_lists(tail(symbols), tail(vals), symbol, 
                                  not_found_cb, val);
  }
}

function lookup_symbol_value(symbol, env) {
  function env_loop(env) {
    if (env === the_empty_environment) {
      error(symbol, "unbound name");
    } else {
      const frame = first_frame(env);
      return scan_pair_of_lists(
        frame_symbols(frame),
        frame_values(frame),
        symbol,
        () => env_loop(enclosing_environment(env))
      );
    }
  }
  return env_loop(env);
}

function assign_symbol_value(symbol, val, env) {
  function env_loop(env) {
    function scan(symbols, vals) {
      return is_null(symbols)
        ? env_loop(enclosing_environment(env))
        : symbol === head(symbols)
        ? set_head(vals, val)
        : scan(tail(symbols), tail(vals));
    }
    if (env === the_empty_environment) {
      error(symbol, "unbound name -- assignment");
    } else {
      const frame = first_frame(env);
      return scan(frame_symbols(frame), frame_values(frame));
    }
  }
  return env_loop(env);
}

// functions from SICP JS 4.1.4

function is_primitive_function(fun) {
  return is_tagged_list(fun, "primitive");
}
function primitive_implementation(fun) {
  return head(tail(fun));
}
function primitive_should_eval_args(fun) {
  return head(tail(tail(fun)));
}

const primitive_functions = list(
  list("pair", pair, false),
  list("list", list, false),
  list("head", head, true),
  list("tail", tail, true),
  list("is_null", is_null, true),
  list("display", display, true),
  list("stringify", stringify, true),
  list("error", error, true),
  list("math_abs", math_abs, true),
  list("+", (x, y) => x + y, true),
  list("-", (x, y) => x - y, true),
  list("-unary", (x) => -x, true),
  list("*", (x, y) => x * y, true),
  list("/", (x, y) => x / y, true),
  list("%", (x, y) => x % y, true),
  list("===", (x, y) => x === y, true),
  list("!==", (x, y) => x !== y, true),
  list("<", (x, y) => x < y, true),
  list("<=", (x, y) => x <= y, true),
  list(">", (x, y) => x > y, true),
  list(">=", (x, y) => x >= y, true),
  list("!", (x) => !x, true)
);
const primitive_function_symbols = map(head, primitive_functions);
const primitive_function_objects = map(
  (fun) => list("primitive", head(tail(fun)), head(tail(tail(fun)))),
  primitive_functions
);

const primitive_constants = list(
  list("undefined", undefined),
  list("Infinity", Infinity),
  list("math_PI", math_PI),
  list("math_E", math_E),
  list("NaN", NaN)
);
const primitive_constant_symbols = map((c) => head(c), primitive_constants);
const primitive_constant_values = map(
  (c) => head(tail(c)),
  primitive_constants
);

function apply_primitive_function(fun, arglist) {
    return apply_in_underlying_javascript(
                primitive_implementation(fun),
                arglist);     
}

function setup_environment() {
  return extend_environment(
    append(primitive_function_symbols, primitive_constant_symbols),
    append(primitive_function_objects, primitive_constant_values),
    the_empty_environment
  );
}

let the_global_environment = setup_environment();

function user_print(prompt_string, object) {
  function to_string(object) {
    return is_compound_function(object)
      ? "<compound-function>"
      : is_primitive_function(object)
      ? "<primitive-function>"
      : is_pair(object)
      ? "[" + to_string(head(object)) + ", " + to_string(tail(object)) + "]"
      : stringify(object);
  }
  display(
    "----------------------------",
    prompt_string + "\n" + to_string(object) + "\n"
  );
}

function user_read(prompt_string) {
  return prompt(prompt_string);
}

const input_prompt = "M-evaluate input: ";
const output_prompt = "M-evaluate value: ";

function driver_loop(env) {
  const input = user_read(input_prompt);
  if (is_null(input)) {
    display("--- evaluator terminated ---", "");
  } else {
    display("----------------------------", input_prompt + "\n" + input + "\n");
    const program = parse(input);
    const locals = scan_out_declarations(program);
    const unassigneds = list_of_unassigned(locals);
    const program_env = extend_environment(locals, unassigneds, env);
    const output = evaluate(program, program_env);
    user_print(output_prompt, output);
    return driver_loop(program_env);
  }
}
// display("metacircular evaluator loaded");

function parse_and_evaluate(string) {
  return evaluate(parse("{ " + string + " }"), the_global_environment);
}

// driver_loop(the_global_environment);

// display(
//   parse_and_evaluate(`
// display('factorial');
// function factorial(n) {
//     return n === 1
//            ? 1
//            : n * factorial(n - 1);
// }
// display(factorial(5));
// ':D';
// `)
// );

// // these tests output a lot of shit because of nested thunks

// // display(
// //   parse_and_evaluate(`
// // function append(xs, ys) {
// //     return is_null(xs)
// //            ? ys
// //            : pair(head(xs), append(tail(xs), ys));
// // }
// // append(list('a', 'b'), list('c', 'd'));
// // `)
// // );

// // display(
// //   parse_and_evaluate(`
// // function map(f, xs) {
// //     return is_null(xs)
// //            ? null
// //            : pair(f(head(xs)), map(f, tail(xs)));
// // }
// // tail(map(x => x + 1, list(1, 2, 3, 4)));
// // `)
// // );

// display(
//   parse_and_evaluate(`
// display('append');
// function append(xs, ys) {
//     return is_null(xs)
//            ? ys
//            : pair(head(xs), append(tail(xs), ys));
// }
// const appended = append(list('a', 'b'), list('c', 'd'));
// function list_ref(xs, n) {
//     return n === 0 ? head(xs) : list_ref(tail(xs), n - 1);
// }
// display(list_ref(appended, 0));
// display(list_ref(appended, 1));
// display(list_ref(appended, 2));
// display(list_ref(appended, 3));
// ':D';
// `)
// );

// // display(
// //   parse_and_evaluate(`
// // function map(f, xs) {
// //     return is_null(xs)
// //            ? null
// //            : pair(f(head(xs)), map(f, tail(xs)));
// // }
// // tail(map(x => x + 1, list(1, 2, 3, 4)));
// // `)
// // );

// display(
//   parse_and_evaluate(`
// display('map');
// function map(f, xs) {
//     return is_null(xs)
//            ? null
//            : pair(f(head(xs)), map(f, tail(xs)));
// }
// const mapped_and_tailed = tail(map(x => x + 1, list(1, 2, 3, 4)));
// function list_ref(xs, n) {
//     return n === 0 ? head(xs) : list_ref(tail(xs), n - 1);
// }
// display(list_ref(mapped_and_tailed, 0));
// display(list_ref(mapped_and_tailed, 1));
// display(list_ref(mapped_and_tailed, 2));
// ':D';
// `)
// );

// display(
//   parse_and_evaluate(`
// display('object construction and access');
// const x = { a: 1, b: 2 };
// const y = { b: 3, c: 4 };
// const z = { a: 5, b: { b: 6, c: 7 } };
// const i = { 0: 8, 1: 9 };
// display(x.a);
// display(x['b' + '']);
// display(y.b);
// display(y.c);
// display(z.a);
// display(z.b);
// display(z.b.b);
// display(z.b.c);
// display(i[0]);
// display(i[1]);
// ':D';
// `)
// );

// display(
//   parse_and_evaluate(`
// display('infinite object construction and access');
// const infinite_object = { x: 1, y: infinite_object };
// display(infinite_object.x);
// display(infinite_object.y.x);
// display(infinite_object.y.y.x);
// ':D';
// `)
// );

// display(
//   parse_and_evaluate(`
// display('infinite list construction and access');
// const ones = pair(1, ones);
// function list_ref(xs, n) {
//     return n === 0 ? head(xs) : list_ref(tail(xs), n - 1);
// }
// display(list_ref(ones, 0));
// display(list_ref(ones, 1));
// display(list_ref(ones, 2));
// display(list_ref(ones, 3));
// ':D';
// `)
// );

// display(
//   parse_and_evaluate(`
// display('infinite list manipulation');
// function list_ref(xs, n) {
//     return n === 0 ? head(xs) : list_ref(tail(xs), n - 1);
// }
// function map(f, xs) {
//     return is_null(xs)
//            ? null
//            : pair(f(head(xs)), map(f, tail(xs)));
// }

// // This implementation "maps" an infinite list,
// // definitely proves that pair does NOT fully evaluate its arguments
// // despite being a builtin.
// function integers_from(start) {
//     const stream = pair(start, map(x => x + 1, stream));
//     return stream;
// }

// // This version is the more straightforward one.
// function integers_from2(start) {
//     return pair(start, integers_from2(start + 1));
// }

// const from_zero = integers_from(0);

// display(list_ref(from_zero, 0));
// display(list_ref(from_zero, 1));
// display(list_ref(from_zero, 2));
// display(list_ref(from_zero, 3));

// const from_twenty = integers_from2(20);

// display(list_ref(from_twenty, 0));
// display(list_ref(from_twenty, 1));
// display(list_ref(from_twenty, 2));
// display(list_ref(from_twenty, 3));
// ':D';
// `)
// );

// display(
//   parse_and_evaluate(`
// display('trying to display a thunk is bad news');
// display(1 + 1);
// // the stringify at least makes it single line
// // the 2nd arg of stringify specifies that the indent should be 0
// display(stringify(
//     pair(1 + 1, 2 + 2),
//     0
// ));
// ':D';
// `)
// );

display(
  parse_and_evaluate(`
  function lookupInClass(theClass, methodname) {
    return theClass.methodname !== undefined
        ? theClass.methodname
        : lookupInClass(theClass.Parent, methodname);
}
function lookup(object, methodname) {
    return lookupInClass(object.Class, methodname);
}
let c0 = {M0: 1, Parent: undefined};
let c1 = {M1: 2, Parent: c0};
let c2 = {M2: 3, Parent: c1};
let x = {Prop: 4, Class: c2};
lookup(x, M1);
`)
);


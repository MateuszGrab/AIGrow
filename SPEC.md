# AIGrow Language Specification

## Comments
- Single-line comments: `// This is a single-line comment`
- Multi-line comments: `/* This is a multi-line comment */`

## Data Types
- Integers: `int`
- Floating-point numbers: `float`
- Booleans: `bool`
- Strings: `string`
- Tuples: `(int, string, bool)`
- Lists: `list[int]`, `list[string]`
- Maps: `map[int, string]`, `map[string, bool]`
- Option and Result types for error handling: `Option<T>`, `Result<T, E>`

## Variables and Constants
- Variable declaration and assignment: `let x: int = 5`
- Constant declaration and assignment: `const y: float = 3.14`

## Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`
- Other: `is`, `in`, `.. (range)`

## Control Structures
- If-else statement:
    ```
    if condition {
        // code
    } else if another_condition {
        // code
    } else {
        // code
    }
    ```
- While loop:
    ```
    while condition {
        // code
    }
    ```
- For loop:
    ```
    for x in 0..10 {
        // code
    }
    ```
- Match statement for pattern matching:
    ```
    match value {
        pattern1 => {
            // code
        },
        pattern2 => {
            // code
        },
        _ => {
            // code (default case)
        }
    }
    ```

## Functions
- Function declaration:
    ```
    fn function_name(parameter1: type1, parameter2: type2) -> return_type {
        // code
    }
    ```
- Pure function declaration (no side effects):
    ```
    pure fn function_name(parameter1: type1, parameter2: type2) -> return_type {
        // code
    }
    ```
- Function call:
    ```
    function_name(arg1, arg2)
    ```
- Async function declaration:
    ```
    async fn async_function() -> int {
        // code
    }
    ```
- Await async function:
    ```
    let result: int = await async_function();
    ```
## Modules and Imports
- Module declaration:
    ```
    mod module_name {
        // code
    }
    ```
- Importing modules: `import path::to::module_name`

## Custom Data Types
- Struct declaration:
    ```
    struct Point {
        x: float,
        y: float,
    }
    ```
- Enum declaration:
    ```
    enum Direction {
        North,
        East,
        South,
        West,
    }
    ```

## Concurrency
- Parallelism support using threads:
    ```
    thread::spawn(|| {
        // code
    });

## Type System and Constraints
- Type alias:
    ```
    type NewType = ExistingType
    ```
- Type parameters:
    ```
    struct Container<T> { value: T }
    ```
- Constraints:
    ```
    constraint constraint_name<T>(x: T) -> bool {
        // code
    }
    ```

## Idempotent Operations
- Marking a function as idempotent:
    ```
    idempotent fn idempotent_function(parameter: type) -> return_type {
        // code
    }
    ```

## Nested Functions
- Defining a nested function:
    ```
    fn outer_function() {
        fn nested_function() {
            // code
        }

        nested_function();
    }
    ```
## Recursion
- Marking a function as recursive:
    ```
    recursive fn recursive_function(parameter: type) -> return_type {
        // code
    }
    ```

## Lazy Evaluation
- Defining a lazy expression:
    ```
    lazy val lazy_expression = {
        // code
    }
    ```

- Force evaluation of a lazy expression:
    ```
    let result = lazy_expression.force();
    ```

## Standard Library
- Predefined functions for common tasks:
    ```
    import std::collections::list;
    import std::collections::map;
    import std::io;
    import std::math;
    import std::string;
    ```

## Iterators and Generators
- Iterator trait and implementation:
    ```
    trait Iterator<Item> {
        fn next(&mut self) -> Option<Item>;
    }
    ```

- Generator function:
    ```
    generator fn generator_function() -> impl Iterator<Item = Type> {
        // code using yield keyword
    }
    ```

## Invariants
- Defining an invariant:
    ```
    invariant invariant_name<T>(x: T) -> bool {
        // code
    }
    ```
## Macros
- Defining a macro:
    ```
    macro_rules! macro_name {
        (pattern) => {
            // code
        };
    }
    ```

- Using a macro: 
    ```
    macro_name!(arguments)
    ```

## Attributes
- Defining custom attributes:
    ```
    attribute attribute_name {
        // code
    }
    ```

- Using attributes:
    ```
    #[attribute_name(parameters)]
    fn function_with_attribute() {
        // code
    }
    ```

## Object-Oriented Programming (Optional)
- Class declaration:
    ```
    class ClassName {
        field1: type1,
        field2: type2,

        fn method1(&self) -> return_type {
            // code
        }
    }
    ```

- Inheritance:
    ```
    class ChildClass: ClassName {
        // additional fields and methods
    }
    ```

- Trait declaration and implementation:
    ```
    trait TraitName {
        fn trait_method(&self) -> return_type;
    }

    impl TraitName for ClassName {
        fn trait_method(&self) -> return_type {
            // code
        }
    }
    ```

This is the complete AIGrow language syntax in Markdown format.

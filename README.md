> This repository contains code for AIGrow, written mostly by GPT4 as a language for more native AI language.

IT IS WIP, 
Builds are failing, but we (Me and GPT4) are working on it

# Language name: AIGrow

AIGrow is a programming language specifically designed for the growth and development of AI models. It focuses on optimizing various aspects of traditional programming languages to better cater to the complexities and unique requirements of AI model creation and optimization.

## Table of Contents

- [Introduction](#introduction)
- [Problem Solved](#problem-solved)
- [Benefits over existing programming languages](#benefits-over-existing-programming-languages)
- [Example of implementation approach](#example-of-implementation-approach)
- [Folder and files structure for Compiler](#folder-and-files-structure-for-compiler)
- [AIGrow Language Specification](#aigrow-language-specification)

## Introduction:

## Problem solved: 
AIGrow addresses the challenges of working with complex logic and intricate data structures in AI model creation. By incorporating features like:  
    - pure functions,   
    - idempotent operations,   
    - pattern matching,   
    - and strong typing,   
it simplifies the process of building and maintaining AI models while reducing the potential for errors and unexpected behavior.

## Benefits over existing programming languages:

| Language Feature                                  | Benefits                                                                                                            |
|---------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|
| Enhanced readability and maintainability          | - Improved code organization<br>- Easier code understanding<br>- More maintainable AI models                          |
| Simplified logic                                  | - Efficient handling of complex logic<br>- Reduced error potential<br>- More straightforward code updates             |
| Strong typing and constraint handling             | - Reduced type-related errors<br>- Avoid unexpected type conversions<br>- Enforced constraints and requirements      |
| Lazy evaluation                                   | - Optimized resource usage<br>- Improved performance through deferred computation                                     |
| Built-in optimization and concurrency support     | - Efficient use of multi-core processors<br>- Simplified implementation of optimization algorithms                    |



## Example of implementation Approach

| Keyword              | Abbreviation | Description                                                                                   |
|----------------------|--------------|-----------------------------------------------------------------------------------------------|
| pure_function        | pf           | Encourages the creation of pure functions to improve code maintainability and predictability. |
| idempotent_operation | idop         | Reduces the risk of unexpected behavior and simplifies code logic.                            |
| constraint_declaration| cdec         | Allows for easy enforcement of constraints throughout a program's lifecycle.                  |
| layered_composition  | lcomp        | Improves code readability and organization by using layers of abstraction.                    |
| pattern_matching     | pmatch       | Provides an elegant way to work with complex data structures.                                 |
| recursive_function   | rfunc        | Promotes the use of recursion for managing repetitive operations and simplifying code.        |
| nested_function      | nfunc        | Improves code readability by organizing functions into a hierarchy.                           |
| strong_typing        | stype        | Minimizes the likelihood of unexpected type conversions and related errors.                   |
| lazy_evaluation      | leval        | Delays computation until it is actually needed, potentially improving performance.            |
| optimization_library | optlib       | Facilitates the use of pre-built optimization algorithms and the creation of custom ones.     |
| constraint_handling  | chand        | Simplifies the creation of constraints to guide optimization processes.                       |
| concurrency_support  | concsup      | Enables the efficient use of multi-core processors and asynchronous task execution.           |



### Example Code    

```
// Importing necessary modules
import ai.aigrow.neural_network as nn
import ai.aigrow.optimization as opt
import ai.aigrow.dataset as ds

// Defining the neural network architecture
model: nn.Sequential {
    layer1: nn.Linear(input_size=784, output_size=128, activation=nn.ReLU),
    layer2: nn.Linear(input_size=128, output_size=64, activation=nn.ReLU),
    output: nn.Linear(input_size=64, output_size=10, activation=nn.Softmax)
}

// Loading the dataset
train_data, test_data: ds.load_mnist_data()

// Setting the optimization algorithm and loss function
optimizer: opt.Adam(lr=0.001, model_params=model.parameters())
loss_fn: nn.CrossEntropyLoss()

// Training the model
epochs: 10
for epoch in range(epochs) {
    for x, y in train_data {
        // Forward pass
        y_pred: model(x)

        // Computing the loss
        loss: loss_fn(y_pred, y)

        // Backward pass
        optimizer.zero_gradients()
        loss.backward()
        optimizer.step()
    }
    print(f"Epoch {epoch + 1} completed. Loss: {loss}")
}

// Making predictions
predictions: []
for x, y in test_data {
    y_pred: model(x)
    predictions.append(y_pred.argmax())
}

// Evaluating the model's performance
accuracy: ds.calculate_accuracy(predictions, test_data.labels)
print(f"Model accuracy: {accuracy * 100}%")
```


## Folder and files structure for Compiler:

```
    aig_compiler
    ├── src
    │   ├── compiler
    │   │   ├── mod.rs
    │   │   ├── parser.rs
    │   │   ├── ast.rs
    │   │   ├── optimizer.rs
    │   │   └── codegen.rs
    │   ├── aigrow
    │   │   ├── mod.rs
    │   │   ├── types.rs
    │   │   └── functions.rs
    │   └── main.rs
    ├── tests
    │   └── compiler_test.rs
    ├── Cargo.toml
    └── README.md
```
*Important Folders:*
| Directory/File Name  | Description                                                                                                          |
|----------------------|----------------------------------------------------------------------------------------------------------------------|
| src                  | Contains the Rust source code for the compiler.                                                                      |
| src/compiler         | Contains the main components of the compiler, such as the parser, abstract syntax tree (AST) definition, optimizer, |
|                      | and code generator.                                                                                                  |
| src/aigrow           | Contains the AIGrow language-specific code, such as types and functions.                                             |
| src/main.rs          | The main entry point of the compiler program.                                                                         |
| tests                | Contains the unit and integration tests for the compiler.                                                             |
| Cargo.toml           | The Rust package manifest file, which includes the project's dependencies and other metadata.                         |
| README.md            | The documentation file describing the project and its usage.                                                         |

*Key files:*
| File Name                          | Description                                                                                                                                                       |
|-----------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| src/compiler/parser.rs            | Contains the code to parse AIGrow source code into an AST.                                                                                                        |
| src/compiler/ast.rs               | Defines the AST structure for the AIGrow language.                                                                                                                |
| src/compiler/optimizer.rs         | Contains the code to optimize the AST for efficient TensorFlow execution.                                                                                         |
| src/compiler/codegen.rs           | Contains the code to generate TensorFlow-compatible code from the optimized AST.                                                                                  |
| src/aigrow/types.rs               | Defines the AIGrow-specific types used in the language.                                                                                                           |
| src/aigrow/functions.rs           | Defines the AIGrow-specific functions, such as neural network layers and activation functions.                                                                    |
| src/main.rs                        | Ties everything together, reads the AIGrow source code as input, and produces TensorFlow-compatible code as output.                                              |


## AIGrow Language Specification

| Section | Description |
| --- | --- |
| [Comments](SPEC.md#comments) | Single-line and multi-line comments |
| [Data Types](SPEC.md#data-types) | int, float, bool, string, tuples, lists, maps, Option, Result |
| [Variables and Constants](SPEC.md#variables-and-constants) | Variable and constant declaration and assignment |
| [Operators](SPEC.md#operators) | Arithmetic, comparison, logical, assignment, other |
| [Control Structures](SPEC.md#control-structures) | if-else statement, while loop, for loop, match statement |
| [Functions](SPEC.md#functions) | Function declaration, pure functions, async functions, function call |
| [Modules and Imports](SPEC.md#modules-and-imports) | Module declaration, importing modules |
| [Custom Data Types](SPEC.md#custom-data-types) | Struct and enum declaration |
| [Concurrency](SPEC.md#concurrency) | Parallelism support using threads |
| [Type System and Constraints](SPEC.md#type-system-and-constraints) | Type alias, type parameters, constraints |
| [Idempotent Operations](SPEC.md#idempotent-operations) | Marking a function as idempotent |
| [Nested Functions](SPEC.md#nested-functions) | Defining a nested function |
| [Recursion](SPEC.md#recursion) | Marking a function as recursive |
| [Lazy Evaluation](SPEC.md#lazy-evaluation) | Defining a lazy expression and force evaluation |
| [Standard Library](SPEC.md#standard-library) | Predefined functions for common tasks |
| [Iterators and Generators](SPEC.md#iterators-and-generators) | Iterator trait and implementation, generator function |
| [Invariants](SPEC.md#invariants) | Defining an invariant |
| [Macros](SPEC.md#macros) | Defining and using macros |
| [Attributes](SPEC.md#attributes) | Defining and using custom attributes |
| [Object-Oriented Programming (Optional)](SPEC.md#object-oriented-programming-optional) | Class declaration, inheritance, trait declaration and implementation |

[Full Specification](SPEC.md)

# Getting Started

This section provides instructions on setting up your project locally.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Install the AIGrow Compiler and Runtime. You can download it from the official website.
- You will need a code editor or an Integrated Development Environment (IDE) with support for the AIGrow programming language.
- Ensure you have a modern operating system with support for AIGrow (Windows, macOS, or Linux).

## Installation

1. Install the AIGrow Compiler and Runtime by following the instructions on the official website.
2. Set up your code editor or IDE with the necessary AIGrow extensions or plugins.
3. Clone the project repository or download the project files from the source.
4. Open the project folder in your code editor or IDE.
5. To build the project, open a terminal in the project directory and run the following command:
```
aigrow build
```

6. To run the project, execute the following command:
```
aigrow run
```

## Usage

AIGrow can be used for a variety of tasks, such as data processing, web development, scientific computing, and more. In this section, we provide a few examples and resources to help you get started with AIGrow.

### Example 1: Hello, World!

Create a new file called `main.aig` and add the following code:

```aigrow
fn main() {
 println!("Hello, World!");
}
```

Build and run the program using the commands mentioned in the Installation section.
### Example 2: Fibonacci Sequence

Create a new file called `fibonacci.aig` and add the following code:

```aigrow

recursive fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let n: int = 10;
    println!("Fibonacci({}) = {}", n, fibonacci(n));
}
```
Build and run the program as mentioned earlier.

### Resources

- AIGrow Official Documentation: Comprehensive documentation for the AIGrow programming language.
- AIGrow Community Forum: A platform to ask questions, share projects, and collaborate with other AIGrow developers.
- AIGrow Standard Library Reference: Detailed documentation on the standard library functions and modules.
- AIGrow Tutorials and Examples: A collection of tutorials and examples to help you learn AIGrow and apply it to various projects.

--- PLAN

Aby zbudować kompilator dla języka AIGrow, wykonajmy następujące kroki:

    Lexer: W pierwszym kroku analizy leksykalnej, lexer będzie tokenizować źródło, czyli rozbić je na pojedyncze jednostki leksykalne, takie jak słowa kluczowe, identyfikatory, literały itp.

    Parser: Następnie parser będzie analizować sekwencję tokenów generowanych przez lexer, konstruując drzewo składniowe (AST) dla kodu źródłowego.

    Analiza semantyczna: Po utworzeniu AST, kompilator będzie przeprowadzać analizę semantyczną, sprawdzając poprawność typów, deklaracji zmiennych, zgodność ze zdefiniowanymi niezmiennikami i ograniczeniami itp.

    Optymalizacja: Po analizie semantycznej, kompilator będzie przeprowadzał optymalizację AST, aby uzyskać bardziej wydajny kod, wykorzystując techniki takie jak redukcja wyrażeń, leniwe obliczenia, eliminacja rekurencji ogonowej itp.

    Generowanie kodu: Na koniec kompilator będzie generował kod wynikowy, który może być wykonywany przez interpreter lub kompilowany do innego języka, np. Pythona czy Rusta.

Oto plan na napisanie kompilatora:

    Rozszerz moduł compiler o następujące komponenty:
        Lexer: Tokenizacja kodu źródłowego.
        Parser: Budowanie AST z sekwencji tokenów.
        SemanticAnalyzer: Analiza semantyczna i walidacja AST.
        Optimizer: Optymalizacja AST w celu uzyskania lepszej wydajności.
        CodeGenerator: Generowanie kodu wynikowego.

    Zaimplementuj funkcje i struktury danych związane z wymaganiami języka:
        Czyste funkcje.
        Idempotentne operacje.
        Deklaracje ograniczeń i niezmienników.
        Wyrażenia i pattern matching.
        Rekursja i zagnieżdżone funkcje.
        Silne typowanie i leniwe obliczenia.

    Zaimplementuj moduły biblioteki standardowej, takie jak funkcje optymalizacji, obsługa ograniczeń i współbieżności.

    Przeprowadź testy jednostkowe i integracyjne, aby sprawdzić poprawność i wydajność kompilatora.

    Napisz dokumentację i przykłady użycia dla języka AIGrow.
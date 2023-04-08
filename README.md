> This repository contains code for AIGrow, written mostly by GPT4 as a language for more native AI language.

# Language name: AIGrow

AIGrow is a programming language specifically designed for the growth and development of AI models. It focuses on optimizing various aspects of traditional programming languages to better cater to the complexities and unique requirements of AI model creation and optimization.

## Table of Contents

- [Introduction](#introduction)
- [Problem Solved](#problem-solved)
- [Benefits over existing programming languages](#benefits-over-existing-programming-languages)
- [Example of implementation approach](#example-of-implementation-approach)
- [Folder and files structure for Compiler](#folder-and-files-structure-for-compiler)
- [AIGrow Language Specification](#aigrow-language-specification)
  - [Comments](#comments)
  - [Data Types](#data-types)
  - [Variables and Constants](#variables-and-constants)
  - [Operators](#operators)
  - [Control Structures](#control-structures)
  - [Functions](#functions)
  - [Modules and Imports](#modules-and-imports)
  - [Custom Data Types](#custom-data-types)
  - [Concurrency](#concurrency)
  - [Type System and Constraints](#type-system-and-constraints)
  - [Idempotent Operations](#idempotent-operations)
  - [Nested Functions](#nested-functions)
  - [Recursion](#recursion)
  - [Lazy Evaluation](#lazy-evaluation)
  - [Standard Library](#standard-library)
  - [Iterators and Generators](#iterators-and-generators)
  - [Invariants](#invariants)
  - [Macros](#macros)
  - [Attributes](#attributes)
  - [Object-Oriented Programming (Optional)](#object-oriented-programming-optional)


## Problem solved: 
AIGrow addresses the challenges of working with complex logic and intricate data structures in AI model creation. By incorporating features like:  
    - pure functions,   
    - idempotent operations,   
    - pattern matching,   
    - and strong typing,   
it simplifies the process of building and maintaining AI models while reducing the potential for errors and unexpected behavior.

## Benefits over existing programming languages:

| Language Feature                                  | Description                                                                                                            |
|---------------------------------------------------|------------------------------------------------------------------------------------------------------------------------|
| Enhanced readability and maintainability          | AIGrow encourages the use of pure functions, nested functions, and layered composition, which promote code organization |
|                                                   | and ease of understanding. This makes AI models developed using AIGrow more maintainable over time.                      |
| Simplified logic                                  | By supporting idempotent operations, pattern matching, and recursive functions, AIGrow enables developers to work with |
|                                                   | complex logic more efficiently and elegantly, reducing the potential for errors and making code more straightforward to |
|                                                   | understand and update.                                                                                                  |
| Strong typing and constraint handling             | AIGrow's strong typing system reduces the risk of type-related errors and unexpected type conversions. Additionally,  |
|                                                   | its support for constraint declarations allows developers to enforce constraints more easily, ensuring that AI models   |
|                                                   | adhere to predefined requirements.                                                                                      |
| Lazy evaluation                                   | AIGrow's lazy evaluation feature allows for the deferral of computation until it is actually needed, potentially        |
|                                                   | improving performance by optimizing resource usage.                                                                    |
| Built-in optimization and concurrency support     | AIGrow comes with a standard optimization library and support for concurrent and parallel task execution, enabling     |
|                                                   | efficient use of multi-core processors and simplifying the implementation of optimization algorithms.                  |



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



## Example Code    

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


# AIGrow Language Specification

| Section | Description |
| --- | --- |
| Comments | Single-line and multi-line comments |
| Data Types | int, float, bool, string, tuples, lists, maps, Option, Result |
| Variables and Constants | Variable and constant declaration and assignment |
| Operators | Arithmetic, comparison, logical, assignment, other |
| Control Structures | if-else statement, while loop, for loop, match statement |
| Functions | Function declaration, pure functions, async functions, function call |
| Modules and Imports | Module declaration, importing modules |
| Custom Data Types | Struct and enum declaration |
| Concurrency | Parallelism support using threads |
| Type System and Constraints | Type alias, type parameters, constraints |
| Idempotent Operations | Marking a function as idempotent |
| Nested Functions | Defining a nested function |
| Recursion | Marking a function as recursive |
| Lazy Evaluation | Defining a lazy expression and force evaluation |
| Standard Library | Predefined functions for common tasks |
| Iterators and Generators | Iterator trait and implementation, generator function |
| Invariants | Defining an invariant |
| Macros | Defining and using macros |
| Attributes | Defining and using custom attributes |
| Object-Oriented Programming (Optional) | Class declaration, inheritance, trait declaration and implementation |



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
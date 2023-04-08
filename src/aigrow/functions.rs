use crate::aigrow::types::{AIGrowKeyword, AIGrowTypes};

pub fn create_aigrow_keywords() -> Vec<AIGrowTypes> {
    vec![
        AIGrowTypes::new(
            AIGrowKeyword::PureFunction,
            "pf",
            "Encourages the creation of pure functions to improve code maintainability and predictability.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::IdempotentOperation,
            "idop",
            "Reduces the risk of unexpected behavior and simplifies code logic.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::ConstraintDeclaration,
            "cdec",
            "Allows for easy enforcement of constraints throughout a program's lifecycle.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::LayeredComposition,
            "lcomp",
            "Improves code readability and organization by using layers of abstraction.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::PatternMatching,
            "pmatch",
            "Provides an elegant way to work with complex data structures.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::RecursiveFunction,
            "rfunc",
            "Promotes the use of recursion for managing repetitive operations and simplifying code.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::NestedFunction,
            "nfunc",
            "Improves code readability by organizing functions into a hierarchy.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::StrongTyping,
            "stype",
            "Minimizes the likelihood of unexpected type conversions and related errors.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::LazyEvaluation,
            "leval",
            "Delays computation until it is actually needed, potentially improving performance.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::OptimizationLibrary,
            "optlib",
            "Facilitates the use of pre-built optimization algorithms and the creation of custom ones.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::ConstraintHandling,
            "chand",
            "Simplifies the creation of constraints to guide optimization processes.",
        ),
        AIGrowTypes::new(
            AIGrowKeyword::ConcurrencySupport,
            "concsup",
            "Enables the efficient use of multi-core processors and asynchronous task execution.",
        ),
    ]
}

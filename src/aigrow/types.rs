#[derive(Debug)]
pub enum AIGrowKeyword {
    PureFunction,
    IdempotentOperation,
    ConstraintDeclaration,
    LayeredComposition,
    PatternMatching,
    RecursiveFunction,
    NestedFunction,
    StrongTyping,
    LazyEvaluation,
    OptimizationLibrary,
    ConstraintHandling,
    ConcurrencySupport,
}

pub struct AIGrowTypes {
    pub keyword: AIGrowKeyword,
    pub abbreviation: String,
    pub description: String,
}

impl AIGrowTypes {
    pub fn new(keyword: AIGrowKeyword, abbreviation: &str, description: &str) -> Self {
        AIGrowTypes {
            keyword,
            abbreviation: abbreviation.to_string(),
            description: description.to_string(),
        }
    }
}

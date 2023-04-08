use std::fs;
use std::path::PathBuf;
use aig_compiler::aigrow::types::{AIGrowKeyword, AIGrowTypes};
use aig_compiler::aigrow::functions::create_aigrow_keywords;


fn get_test_file_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("test_files");
    path.push(file_name);
    path
}

#[test]
fn test_load_aigrow_file() {
    let file_name = "example_program.aig";
    let file_path = get_test_file_path(file_name);
    let source_code = fs::read_to_string(file_path).expect("Failed to read test file");

    assert!(!source_code.is_empty(), "Loaded AIGrow source code should not be empty");
    assert!(
        source_code.contains("pf calculate_sum"),
        "Loaded AIGrow source code should contain the 'calculate_sum' function"
    );
    assert!(
        source_code.contains("pf main"),
        "Loaded AIGrow source code should contain the 'main' function"
    );
}


#[test]
fn test_create_aigrow_keywords() {
    let aigrow_keywords = create_aigrow_keywords();

    assert_eq!(aigrow_keywords.len(), 12);

    for aigrow_type in aigrow_keywords {
        assert!(!aigrow_type.abbreviation.is_empty());
        assert!(!aigrow_type.description.is_empty());

        match aigrow_type.keyword {
            AIGrowKeyword::PureFunction => assert_eq!(aigrow_type.abbreviation, "pf"),
            AIGrowKeyword::IdempotentOperation => assert_eq!(aigrow_type.abbreviation, "idop"),
            AIGrowKeyword::ConstraintDeclaration => assert_eq!(aigrow_type.abbreviation, "cdec"),
            AIGrowKeyword::LayeredComposition => assert_eq!(aigrow_type.abbreviation, "lcomp"),
            AIGrowKeyword::PatternMatching => assert_eq!(aigrow_type.abbreviation, "pmatch"),
            AIGrowKeyword::RecursiveFunction => assert_eq!(aigrow_type.abbreviation, "rfunc"),
            AIGrowKeyword::NestedFunction => assert_eq!(aigrow_type.abbreviation, "nfunc"),
            AIGrowKeyword::StrongTyping => assert_eq!(aigrow_type.abbreviation, "stype"),
            AIGrowKeyword::LazyEvaluation => assert_eq!(aigrow_type.abbreviation, "leval"),
            AIGrowKeyword::OptimizationLibrary => assert_eq!(aigrow_type.abbreviation, "optlib"),
            AIGrowKeyword::ConstraintHandling => assert_eq!(aigrow_type.abbreviation, "chand"),
            AIGrowKeyword::ConcurrencySupport => assert_eq!(aigrow_type.abbreviation, "concsup"),
        }
    }
}

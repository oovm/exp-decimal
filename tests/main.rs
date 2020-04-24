use math_ast::AST;
use std::collections::{BTreeSet, HashSet};

#[test]
fn test() {
    println!("{}", AST::Number(1) + AST::Number(2));
    println!("{}", AST::Number(1) - AST::Number(2));
}

#[test]
fn ord() {
    let mut a: BTreeSet<_> = Default::default();
    a.insert(AST::Number(1));
}

#[test]
fn hash() {
    let mut a: HashSet<_> = Default::default();
    a.insert(AST::Number(1));
}

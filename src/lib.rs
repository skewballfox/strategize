#![allow(dead_code)]
pub mod potential;
pub mod variable;

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

use dashmap::{DashMap, DashSet};
use spaces::{Card, Space};
enum InfluenceNode<T> {
    Decision(Variable<T>),
    ///Maps to Observations
    Chance(Option<Variable<T>>),
    Reward(Variable<T>),
}

pub struct InfluenceDiagram<VarKey, StateKey> {
    variables: DashMap<VarKey, Variable<StateKey>>,
}

///Stores information relevant to humans but superfluous
/// for computation
struct VariableInfo {
    ///The variables being used in the influence diagram
    /// the index is the *first* key for the Variable Struct
    variables: Vec<String>,
    ///The states a variable can exist in
    /// the index is the *second* key for the Variable struct
    state_names: HashMap<usize, Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}

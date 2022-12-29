use num_traits::{real::Real, Zero};
use num_traits::{NumAssignOps, NumOps};
use spaces::{Card, Space};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div},
};

use crate::Variable;
#[derive(Clone)]
struct TreeNode<VarKey, StateKey, RealNumber> {
    children: BTreeMap<(VarKey, StateKey), TreeNode<VarKey, StateKey, RealNumber>>,
    value: RealNumber,
}

impl<VarKey, StateKey, RealNumber> TreeNode<VarKey, StateKey, RealNumber>
where
    VarKey: Ord + Clone,
    StateKey: Ord + Clone,
    RealNumber: Real + NumOps + NumAssignOps + Zero + Clone,
{
    fn new(value: RealNumber) -> Self {
        Self {
            children: BTreeMap::new(),
            value,
        }
    }
    fn default() -> Self {
        Self {
            children: BTreeMap::new(),
            value: RealNumber::zero(),
        }
    }
    fn with_value(mut self, val: RealNumber) -> Self {
        self.value = val;
        self.clone()
    }
    fn set_value(&mut self, val: RealNumber) {
        self.value = val;
    }
    fn normalize(&mut self) {
        let mut sum: RealNumber = RealNumber::zero();
        for child in self.children.values() {
            sum += child.value;
        }
        for child in self.children.values_mut() {
            child.value /= sum;
        }
    }
}

impl<VarKey, StateKey, RealNumber> TreeNode<VarKey, StateKey, RealNumber> {}
pub struct Potential<VarKey, StateKey, RealNumber> {
    domain: BTreeSet<VarKey>,
    prob_tree: BTreeMap<(VarKey, StateKey), TreeNode<VarKey, StateKey, RealNumber>>,
}

impl<VarKey, StateKey, RealNumber> Potential<VarKey, StateKey, RealNumber>
where
    VarKey: Ord + Clone,
    StateKey: Ord + Clone,
    RealNumber: Real + NumOps + NumAssignOps + Zero + Clone,
{
    fn add_var(&mut self, k: VarKey, var: Variable<StateKey>, values: Vec<RealNumber>) {
        if var.states.len() != values.len() {
            panic!("The number of values must be equal to the number of possible states")
        }
        self.domain.insert(k.clone());
        //let node = TreeNode::<VarKey, StateKey, RealNumber>::default();
        (var.states.into_iter())
            .zip(values)
            .for_each(move |(state, val)| {
                self.prob_tree
                    .insert(
                        (k.clone(), state),
                        TreeNode::<VarKey, StateKey, RealNumber>::new(val),
                    )
                    .unwrap();
            });
    }

    fn add_chain_conditional(
        &mut self,
        keys: Vec<VarKey>,
        var: Variable<StateKey>,
        values: Vec<RealNumber>,
    ) {
        todo!()
        // if var.states.len() != values.len() {
        //     panic!("The number of values must be equal to the number of possible states")
        // }

        // self.domain.insert(k.clone());
        // //let node = TreeNode::<VarKey, StateKey, RealNumber>::default();
        // (var.states.into_iter())
        //     .zip(values)
        //     .for_each(move |(state, val)| {
        //         self.prob_tree
        //             .insert(
        //                 (k.clone(), state),
        //                 TreeNode::<VarKey, StateKey, RealNumber>::new(val),
        //             )
        //             .unwrap();
        //     });
    }
}

impl<VarKey, StateKey, RealNumber> Space for Potential<VarKey, StateKey, RealNumber>
where
    VarKey: Ord + Clone + Eq + Hash,
    StateKey: Ord + Clone + Eq + Hash,
{
    const DIM: usize = 1;

    type Value = VarKey;

    fn card(&self) -> Card {
        return Card::Finite(self.domain.len());
    }

    fn contains(&self, val: &Self::Value) -> bool {
        self.domain.contains(val)
    }
}

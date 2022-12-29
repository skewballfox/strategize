use spaces::{Card, Space};
use std::collections::BTreeSet;
use std::hash::Hash;
//let Y_n be the set of chance variables observed before
//Decision D_n is made. Let X be unobserved chance variables
//Y1 ≺ D1 ≺ Y2 ≺ D2 ≺ . . . ≺ Yn ≺ Dn ≺ X
///A Categorical Distribution
#[derive(Clone)]
pub struct Variable<State> {
    ///the possible states instances can exist in
    states: BTreeSet<State>,
    ///the order of the variable, equal to the number of parents
    ord: usize,
}

impl<State> Space for Variable<State>
where
    State: Ord + Clone + Eq + Hash,
{
    const DIM: usize = 1;

    type Value = State;

    fn card(&self) -> Card {
        return Card::Finite(self.states.len());
    }

    fn contains(&self, val: &Self::Value) -> bool {
        self.states.contains(val)
    }
}

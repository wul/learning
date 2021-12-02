#![allow(non_snake_case)]
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug)]
pub struct NFAState <S, T> {
    state: T,
    accepting: bool,
    paths: HashMap<S, HashSet<NFAState<S, T>>>,
}



impl<S, T> PartialEq for NFAState<S, T> where
    S: Eq + Hash,
    T: Eq + Hash 

{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<S: Eq+Hash, T: Eq+Hash> Eq for NFAState<S, T> {}


impl<S, T> Hash for NFAState<S, T> where
    S: Eq + Hash,
    T: Eq + Hash 

{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}



impl <S , T> NFAState <S, T> where 
    S: Eq + Hash,
    T: Eq + Hash { 
    pub fn new(state: T, accepting: bool) -> NFAState<S, T> {
        let ret = NFAState {
            state: state, 
            accepting: accepting,
            paths: HashMap::new(),
        };
        
        ret
    }
    pub fn add_transition(&mut self, symbol: S, to: NFAState<S, T>) {
        if let Some(group) = self.paths.get_mut(&symbol) {
            group.insert(to);
        } else {
            self.paths.insert(symbol, HashSet::from([to]));
        }     
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nfa2dfa () {
        let state: NFAState<char, i32> = NFAState::new(2, true);
        let state2: NFAState<char, i32> = NFAState::new(1, false);
        assert_eq!(state, state2);
    }

    
}

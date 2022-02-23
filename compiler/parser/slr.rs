mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON, ENDMARKER};
use std::collections::{HashSet, HashMap};


struct SLR <'a> {
    cfg: &'a CFG<'a>,
    _ACTION: HashMap<(i32, Symbol<'a>), (String, i32, Option<Item<'a>>)>,
    _GOTO: HashMap<(i32, Symbol<'a>), i32>,
    _state:Vec<State<'a>>,
    state_index: i32,
}


fn are_states_same(me: &State, other: &State) -> bool {
    if me.len() != other.len() {
	return false;
    }
    
    let mut idx = 0;
    for idx in 0..me.len() {
	if me[idx] != other[idx] {
	    return false
	}
    }

    return true;

}
impl<'a> SLR <'a>{
    fn new(cfg: &'a CFG<'a>) -> Self{
	SLR {
	    cfg, 
	    _ACTION: HashMap::new(),
	    _GOTO: HashMap::new(),
	    _state: Vec::new(),
	    state_index: 0,
	}
    }


    fn closure(&self, items: &mut Vec<Item<'a>>) {
	loop {
	    let mut new_items = Vec::<Item>::new();
	    for item in items.iter() {
		if item.dot >= item.body.len() {
		    continue;
		}

		let symbol = item.body[item.dot];
		if self.cfg.is_non_terminal(symbol) {
		    let production = self.cfg.get_production(symbol);
		    let item_lst = production.deconstruct();

		    for i in item_lst.into_iter() {
			if !items.contains(&i) {
			    new_items.push(i);
			}
		    }
		}
	    }

	    if new_items.is_empty() {
		break;
	    } else {
		new_items.into_iter().for_each(|x| {items.push(x);});
	    }
	}
    }

    fn closure2(&self, nt: Symbol) -> State{
	let production = self.cfg.get_production(nt);
	let mut state = production.deconstruct();

	self.closure(&mut state);
	return state;
    }
    
    fn get_state_idx(&self, state: &State) -> i32 {
	let mut idx = -1;
	for st in self._state.iter() {
	    idx += 1;
	    if st == state {
		return idx as i32;
	    }
	}
	
	return -1;
    }

    fn get_next_goto_X(&self, state: & State<'a>) -> Vec<Symbol<'a>>{
	let mut Xs = Vec::<Symbol>::new();
	for item in state.iter() {
	    let body = &item.body;
	    if item.dot < body.len() && body[item.dot] != EPSILON {
		let t = body[item.dot];
		if !Xs.contains(&t) {
		    Xs.push(t);
		}
	    }
	}
	return Xs
    }
    fn gotoT(&self, state: State<'a>, X:Symbol)->State<'a> {
	let mut st =  Vec::new();


	for item in state.iter() {
	    if item.dot == item.body.len() {
		continue;
	    }

	    if item.body[item.dot] == X {
		st.push(Item{head:item.head,
				body:item.body.clone(),
				dot:item.dot+1,
				lookahead:vec![],
		});

	    }
	}

	self.closure(&mut st);
	return st;
    }

    fn goto(&self, state: &'a State, X: Symbol) -> State<'a> {
	let mut st = Vec::new();

	for item in state.iter() {
	    if item.dot == item.body.len() {
		continue;
	    }

	    if item.body[item.dot] == X {
		let item = Item{head:item.head,
				body:item.body.clone(),
				dot:item.dot+1,
				lookahead:vec![],
		};
		st.push(item);
	    }
	}
	self.closure(&mut st);
	return st;
    }

    fn traversal_state(&mut self) {
	let mut pool = vec![self.state_index];
	let mut processed = HashSet::<i32>::new();

	loop {
	    let mut new_states = Vec::<i32>::new();
	    for &state_idx in pool.iter() {
		if processed.contains(&state_idx) {
		    continue;
		}
		
		let state = &self._state[state_idx as usize];
		let mut discovered_states = Vec::new();
		for item in state.iter() {
		    if item.body.len() == item.dot {
			
			if item.head == "S'" && item.body.last() == Some(&self.cfg.S) {
			    self._ACTION.insert((state_idx, ENDMARKER), ("Accept".to_string(), -1, None));
			    continue;
			}
		    }
		    
		    for &t in self.cfg.FOLLOW(item.head).iter() {
			self._ACTION.insert((state_idx, t), ("Reduce".to_string(), -1, Some(item.clone())));
		    }

		    let next_symbols = self.get_next_goto_X(state);
		    
		    for X in next_symbols.into_iter() {
			//let new_state = self.goto(state, X);
			//
			// WHY HAVE TO USE CLONE
			//
			let new_state = self.gotoT(state.clone(), X);
			let mut new_state_idx = self.get_state_idx(&new_state);

			if new_state_idx == -1 {
			    self.state_index += 1;
			    new_state_idx = self.state_index;
			    //self._state.push(new_state);
			    discovered_states.push(new_state);
			    new_states.push(new_state_idx);
			}

			if self.cfg.is_terminal(X) {
			    self._ACTION.insert((state_idx, X), ("Shift".to_string(), new_state_idx, None));
			} else {
			    self._GOTO.insert((state_idx, X), new_state_idx);
			}
		    }

		    processed.insert(state_idx);
		}

		discovered_states.into_iter().for_each(|x| self._state.push(x));
		    
	    }
	    if new_states.is_empty() {
		break;
	    } else {
		new_states.into_iter().for_each(|x| pool.push(x));
	    }
	}
    }
			
							    
    
    /*
    fn GOTO(&self, state: &'a State, X: Symbol) -> &'a State<'a> {
	return self
    }
     */
}

fn main() {
}

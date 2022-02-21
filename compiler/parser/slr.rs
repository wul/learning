mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON};
use std::collections::{HashSet, HashMap};


struct SLR <'a> {
    cfg: &'a CFG<'a>,
    _ACTION: HashMap<(u32, Symbol<'a>), (String, Option<u32>)>,
    _GOTO: HashMap<(u32, Symbol<'a>), u32>,
    _state:Vec<State<'a>>,
    state_index: u32,
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
    fn traversal_state(&self) {
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

    fn get_next_goto_X(&self, state: State<'a>) -> Vec<Symbol<'a>>{
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
	
}

fn main() {
}

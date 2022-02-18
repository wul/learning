mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON};
use std::collections::{HashSet, HashMap};


struct SLR <'b, 'a:'b> {
    cfg: &'b CFG<'a>,
    _ACTION: HashMap<(u32, Symbol<'a>), (String, Option<u32>)>,
    _GOTO: HashMap<(u32, Symbol<'a>), u32>,
    _state:Vec<State<'a>>,
    state_index: u32,
}

impl<'b, 'a> SLR <'b, 'a>{
    fn new(cfg: &'b CFG<'a>) -> Self{
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

    fn closure(self, items: Vec<Item>) -> State {
	loop {
	    let new_items = Vec::<Item>::new();
	    for item in items {
		if item.dot >= item.body.len() {
		    continue;
		}

		let symbol = item.body[item.dot];
		if self.cfg.is_non_terminal(symbol) {
		    let production = self.cfg.get_production(symbol);
		    let item_lst = production.deconstruct();
		}
	    }
	}
		

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

    fn get_next_goto_X(&self, state: &'b State) -> Vec<Symbol<'b>>{
	let Xs = Vec::<Symbol>::new();
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
	
}

fn main() {
}

mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON, ENDMARKER};
use std::collections::{HashSet, HashMap};

mod stack;
use stack::Stack;

use std::fmt::{Display, Write};

struct SLR <'a> {
    cfg: &'a CFG,
    _ACTION: HashMap<(i32, Symbol), (String, i32, Option<Item>)>,
    _GOTO: HashMap<(i32, Symbol), i32>,
    _state:Vec<State>,
    state_index: i32,
}



fn repr_right<T>(items: &Vec<T>) -> String
where T: std::fmt::Display,
{
    //format!("| {}", Itertools::join(&mut items.iter(), " "))
    //"".to_string()
    let mut out = "| ".to_string();
    items.iter().map(|x| {out = out.to_owned() + &x.to_string() + " "});
    out
	
}

fn repr_left(items: &Vec<Symbol>) -> String{
    let mut cloned = items.clone();
    cloned.reverse();
    format!("{}", cloned.join(" "))
}

impl<'a> SLR <'a>{
    fn new(cfg: &'a CFG) -> Self{
	SLR {
	    cfg, 
	    _ACTION: HashMap::new(),
	    _GOTO: HashMap::new(),
	    _state: Vec::new(),
	    state_index:0,
	}
    }


    fn closure(&self, items: &mut State) {
	loop {
	    let mut new_items = Vec::<Item>::new();
	    for item in items.iter() {
		if item.dot >= item.body.len() {
		    continue;
		}

		let symbol = &item.body[item.dot];
		if self.cfg.is_non_terminal(&symbol) {
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

    fn closure2(&self, nt: &Symbol) -> State{
	let production = self.cfg.get_production(nt);
	let mut state = State::from(production.deconstruct());

	self.closure(&mut state);
	return state;
    }
    fn init(&mut self) {
	self.build_items_sets();
    }

	    
    fn get_state_idx(&self, state: & State) -> i32 {
	let mut idx = -1;
	for st in self._state.iter() {
	    idx += 1;
	    if st == state {
		return idx as i32;
	    }
	}

	return -1;
    }

    fn get_next_goto_X(&self, state: &State) -> Vec<Symbol>{
	let mut Xs = Vec::<Symbol>::new();
	for item in state.iter() {
	    let body = &item.body;
	    if item.dot < body.len() && body[item.dot] != EPSILON {
		let t = &body[item.dot];
		if !Xs.contains(t) {
		    Xs.push(t.clone());
		}
	    }
	}
	return Xs
    }

    fn discover(&self, state_idx: i32, X: &Symbol) -> State {
	let state = &self._state[state_idx as usize];
	let mut st = State::new();

	for item in state.iter() {
	    if item.dot == item.body.len() {
		continue;
	    }

	    if item.body[item.dot] == *X {
		let it = Item{head:item.head.clone(),
				body:item.body.clone(),
				dot :item.dot+1,
				lookahead:HashSet::new(),
		};
		st.push(it);
	    }
	}
	self.closure(&mut st);
	return st;
    }

    fn get_state(&self, state_idx: i32) -> &State{
	&self._state[state_idx as usize]
    }
    fn build_items_sets(&mut self) {
	let mut state = State::from(vec![Item {head: "S'".to_string(),
				   body: vec![self.cfg.S.clone()],
				   dot:0,
				   lookahead:HashSet::new()}]);
	self.closure(&mut state);
	self._state.push(state);
	self.state_index = 0;
	self.traversal_state();
    }
    /*
    fn traversal_state(&mut self) {
	let mut pool = vec![self.state_index];
	let mut processed = HashSet::<i32>::new();

	loop {
	    let mut new_states = Vec::<i32>::new();
	    for &state_idx in pool.iter() {
		if processed.contains(&state_idx) {
		    continue;
		}
		
		let mut discovered_states = Vec::new();
		let state = &self._state[state_idx as usize];
		let next_symbols = self.get_next_goto_X(state);
		
		for item in state.iter() {
		    if item.body.len() == item.dot {
			
			if item.head == "S'" && item.body.last() == Some(&self.cfg.S) {
			    self._ACTION.insert((state_idx, ENDMARKER), ("Accept".to_string(), -1, None));
			} else {
			    for &t in self.cfg.FOLLOW(item.head).iter() {
				self._ACTION.insert((state_idx, t), ("Reduce".to_string(), -1, Some(item.clone())));
			    }
			}
			continue;
		    }


		    for &X in next_symbols.iter() {
			let new_state = self.discover(state_idx, X);
			
			let mut new_state_idx = self.get_state_idx(&new_state);

			if new_state_idx == -1 {
			    self.state_index += 1;
			    new_state_idx = self.state_index;
			    discovered_states.push(new_state);
			    new_states.push(new_state_idx);

			    self.try_add_new_state(new_state);
			    //self._state.push(new_state);
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
     */


    fn traversal_state(&mut self) {
	let mut processed = 0;
	while processed < self._state.len() {

	    println!("--------LOOP BEGIN--processed:{}, size:{}------", processed, self._state.len());

	    let total = self._state.len();
	    let mut top:i32 = (total as i32) - 1;
	    
	    for index in processed .. total {
		println!("Process {}", index);
		let state_idx = index as i32;

		//why not work by using function
		//let state = self.get_state(state_idx as i32);
		let state = &self._state[index];
		
		for item in state.iter() {
		    if item.body.len() == item.dot {
			if item.head == "S'" && item.body.last() == Some(&self.cfg.S) {
			    self._ACTION.insert((state_idx, ENDMARKER.to_string()),
						("Accept".to_string(), -1, None));
			} else {
			    for t in self.cfg.FOLLOW(&item.head).iter() {
				self._ACTION.insert((state_idx, t.clone()),
						    ("Reduce".to_string(), -1, Some(item.clone())));
			    }
			}
		    }
		}		

		let next_symbols = self.get_next_goto_X(state);

		for X in next_symbols.iter() {
		    let new_state = self.discover(state_idx, X);
		    let mut idx = self.get_state_idx(&new_state);
		    if idx == -1 {
			println!("new state\n{:?}", new_state);
			self._state.push(new_state);
			top += 1;
		    }
		    if self.cfg.is_terminal(X) {
			self._ACTION.insert((state_idx, X.clone()), ("Shift".to_string(), top, None));}
		    else {
			self._GOTO.insert((state_idx, X.clone()), top);
		    }

		}
	    } //done of _state loop
	    processed = total;

	}

    }
		

							    
    fn GOTO(&self, state_idx: i32, X: &Symbol) -> Option<i32> {
	self._GOTO.get(&(state_idx, X.clone())).and_then(|x| Some(*x))
    }

    fn ACTION(&self, state_idx: i32, X: &Symbol) -> Option<&(String, i32, Option<Item>)> {
	self._ACTION.get(&(state_idx, X.clone()))
	
    }
    
    fn print_state(&self, stack: &Stack<i32>, stack2: &Stack<Symbol>, production: &str) {
	println!("  {:<20}\t\t{:<40}\t{} ", repr_right(stack.items()), repr_right(stack2.items()), production);
    }
    fn print(&self) {
	println!("Print state table");
	for (state_idx, state) in self._state.iter().enumerate() {
	    println!("{}", state_idx);
	    for item in state.iter() {
		println!("\t{:?}", item);
	    }

	}
	println!("Pirnt GOTO table");
	println!("{:?}", self._GOTO);
	println!("Print ACTION table");
	println!("{:?}", self._ACTION);
    }
    fn parse(&self, s: &str) {
	println!("\nParse string:{}", s);

	let mut ss: Vec<String> = s.split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
	ss.push(ENDMARKER.to_string());
	
	let mut stack = Stack::new();
	let mut stack_symbol = Stack::new();

	let mut cache = vec![0];
	stack.push(0); //push state 0
	stack_symbol.push(ENDMARKER.to_string());

	let mut idx = 0;
	let mut t = &ss[idx];
	loop {
	    if let Some(&state) = stack.peek() {
		let res = self.ACTION(state, &t);
		self.print_state(&stack, &stack_symbol, "");
		match res {
		    Some(act) => {
			if act.0 == "Accept" {
			    //do nothing
			    println!("Done, accepted");
			} else if act.0 == "Reduce" {
			    if let Some(ref item) = act.2 {
				let head = &item.head;
				let body = &item.body;
				let mut length = item.body.len();
				while length > 0 {
				    stack_symbol.pop();
				    stack.pop();
				    length -= 1;
				}

				stack_symbol.push(head.clone());
				if let Some(&state) = stack.peek() {
				    if let Some(next) = self.GOTO(state, head) {
					stack.push(next);
				    } else {
					panic!("GOTO state:{}, symbol: {} lookup failed", state, head);
				    }
				} else {
				    panic!("State stack is empty!");
				}
				    
					
			    } else {
				panic!("Reduce must have specified item given");
			    }
			} else if act.0 == "Shift" {
			    stack_symbol.push(t.clone());
			    stack.push(act.1);
			    idx += 1;
			    if idx >= ss.len() {
				break;
			    }
			    t = &ss[idx];
			    
			}
		    },
		    None => {
			break;
		    }
		}
	    }
	}
    }
		    
	
}

fn main() {
    let productions = "
	E -> E + T | T
    T -> T * F | F
    F -> ( E ) | id
";
    
    let cfg = CFG::new(productions);
    let mut slr = SLR::new(&cfg);
    slr.init();
    slr.print();
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure() {
	let s = "
                 E  -> T E'      
                 E' -> + T E' | ε
                 T  -> F T'
                 T' -> * F T' | ε
                 F  -> ( E ) | id
                ";

	let state = vec![Item { head: "E".to_string(), body: vec!["T".to_string(), "E'".to_string()], dot: 0, lookahead: vec![] },
			 Item { head: "T".to_string(), body: vec!["F".to_string(), "T'".to_string()], dot: 0, lookahead: vec![] },
			 Item { head: "F".to_string(), body: vec!["(".to_string(), "E".to_string(), ")".to_string()], dot: 0, lookahead: vec![] },
			 Item { head: "F".to_string(), body: vec!["id".to_string()], dot: 0, lookahead: vec![] }];
	
	let cfg = CFG::new(s);
	let mut slr = SLR::new(&cfg);
	let items = slr.closure2(&"E".to_string());
	println!("{:?}", items);
	
	assert_eq!(items, state);

    }

    #[test]
    fn test_get_next_goto_X() {
	let s = "
                 E  -> T E'      
                 E' -> + T E' | ε
                 T  -> F T'
                 T' -> * F T' | ε
                 F  -> ( E ) | id
                ";

	let state = vec![Item { head: "E".to_string(), body: vec!["T".to_string(), "E'".to_string()], dot: 0, lookahead: vec![] },
			 Item { head: "T".to_string(), body: vec!["F".to_string(), "T'".to_string()], dot: 0, lookahead: vec![] },
			 Item { head: "F".to_string(), body: vec!["(".to_string(), "E".to_string(), ")".to_string()], dot: 0, lookahead: vec![] },
			 Item { head: "F".to_string(), body: vec!["id".to_string()], dot: 0, lookahead: vec![] }];
	
	let cfg = CFG::new(s);
	let mut slr = SLR::new(&cfg);
	let mut I0 = cfg.get_production(&"E".to_string()).deconstruct();
	I0.insert(0, Item{head:"S'".to_string(), body: vec![cfg.S.clone()], dot:0, lookahead: vec![]});
	slr.closure(&mut I0);

	let next_symbols = slr.get_next_goto_X(&I0);
	println!("Xs:{:?}", next_symbols);
	assert_eq!(&next_symbols, &vec!["E".to_string(), "T".to_string(), "F".to_string(), "(".to_string(), "id".to_string()]);

    }

    #[test]
    fn test_goto() {
	let s = "
                E -> E + T | T
                T -> T * F | F
                F -> ( E ) | id
                ";

	
	let cfg = CFG::new(s);
	let mut slr = SLR::new(&cfg);
	slr.init();

	
	let state0 = slr.get_state(0);
	let state1 = slr.get_state(1);	
	println!("I0:{:?}", &state0);	
	println!("I1:{:?}", &state1);
	assert_eq!(State::from(vec![Item { head: "E".to_string(), body: vec!["T".to_string(), "E'".to_string()], dot: 0, lookahead: vec![] },
				    Item { head: "T".to_string(), body: vec!["F".to_string(), "T'".to_string()], dot: 0, lookahead: vec![] }]),
		   state1);
    }

}

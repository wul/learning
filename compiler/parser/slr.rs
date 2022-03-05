mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON, ENDMARKER};
use std::collections::{HashSet, HashMap};

mod stack;
use stack::Stack;

use std::fmt::{Display, Write};

struct SLR <'text, 'cfg> {
    cfg: &'cfg CFG<'text>,
    _ACTION: HashMap<(i32, Symbol<'text>), (String, i32, Option<Item<'text>>)>,
    _GOTO: HashMap<(i32, Symbol<'text>), i32>,
    _state:Vec<State<'text>>,
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


fn state_in_state_set(state:&State, set: &Vec<State>) -> bool {
    for st in set.iter() {
	if state == st {
	    return true;
	}
    }
    return false;
}
impl<'text, 'cfg> SLR <'text, 'cfg>{
    fn new(cfg: &'cfg CFG<'text>) -> Self{
	SLR {
	    cfg, 
	    _ACTION: HashMap::new(),
	    _GOTO: HashMap::new(),
	    _state: Vec::new(),
	}
    }


    fn closure(&self, items: &'text State<'text>){
	loop {
	    let mut new_items = Vec::new();
	    for item in items.iter() {
		if item.dot >= item.body.len() {
		    continue;
		}

		let symbol = item.body[item.dot];
		if self.cfg.is_non_terminal(symbol) {
		    if let Some(item_lst) = self.cfg.get_items(symbol) {
			for i in item_lst.into_iter() {
			    if !items.contains(&i) {
				new_items.push(i);
			    }
			}
		    }
		}
	    }

	    if new_items.len() == 0 {
		break;
	    } else {
		new_items.into_iter().for_each(|x| {items.push(x);});
	    }
	}
	
    }

    fn closure2(&self, nt: Symbol<'text>) -> State<'text>{

	if let Some(items) = self.cfg.get_items(nt) {
	    let mut state = State::from(items);
	    self.closure(&mut state);
	    return state;
	} else {
	    return State::new();
	}
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

    fn get_next_goto_X<'b>(&self, state: &'b State) -> Vec<Symbol<'b>>{
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

    fn get_next_X(&self, idx: usize) -> Vec<Symbol>{
	let mut Xs = Vec::<Symbol>::new();
	for item in self._state[idx].iter() {
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
    
    fn discover(&self, state_idx: i32, X: Symbol<'text>) -> State<'text> {

	let mut st = Vec::new();

	if let Some(state) = self._state.get(state_idx as usize) {
	    for item in state.iter() {
		if item.dot == item.body.len() {
		    continue;
		}
		
		if item.body[item.dot] == X {
		    let it = Item{head:item.head.clone(),
				  body:item.body.clone(),
				  dot :item.dot+1,
				  lookahead:vec![],
		    };
		    st.push(it);
		}
	    }
	}

	let mut s = State::from(st);
	self.closure(&mut s);
	return s;
    }

    
    fn contains_state(&self, state: &State) -> bool {
	for st in self._state.iter() {
	    if st == state {
		return true
	    }
	}
	return false;
    }
    
    fn get_state_items(&self, state_idx: i32) -> &State{
	&self._state[state_idx as usize]
    }
    fn build_items_sets(&mut self) {
	let mut state = vec![Item {head: "S'", body: vec![self.cfg.S], dot:0, lookahead:vec![]}];
	let state = self.closure(State::from(state));
	self._state.push(state);
	self.traversal_state();
    }

    fn traversal_state(&mut self) {
	let mut processed = 0;

	while processed < self._state.len() {

	    let total = self._state.len();
	    let mut top: i32 = (total as i32) - 1;

	    for index in processed .. total {
		println!("processed {}, total {}", processed,total);
		let state_idx = index as i32;

		//state = &self._state[state_idx as usize];

		//if let Some(state) = self._state.get(state_idx as usize) {
		for item in self._state[state_idx as usize].iter() {
		    if item.body.len() == item.dot {
			let head = item.head;
			/*
			if item.head == "S'" && item.body.last() == Some(&self.cfg.S) {
			    self._ACTION.insert((state_idx, ENDMARKER), ("Accept".to_string(), -1, None));
			} else {
			    for &t in self.cfg.FOLLOW(head).iter() {
				self._ACTION.insert((state_idx, t), ("Reduce".to_string(), -1, Some(item.clone())));
			    }
			}
*/
		    }
		}
		
		let next_symbols = self.get_next_X(state_idx as usize);
		
		for &X in next_symbols.iter() {
		    let new_state = self.discover(state_idx, X);
		    let mut idx = self.get_state_idx(&new_state);
		    if idx == -1 {
			self._state.push(new_state);
			top += 1;
		    }
		    /*
		    if self.cfg.is_terminal(X) {
			self._ACTION.insert((state_idx, X), ("Shift".to_string(), top, None));
		    } else {
			self._GOTO.insert((state_idx, X), top);
		    }
*/
		}

	    } //state loop in pool
	    processed = total;
	    self.print();
	}
    }
		

							    
    fn GOTO(&self, state_idx: i32, X: Symbol) -> Option<i32> {
	self._GOTO.get(&(state_idx, X)).and_then(|x| Some(*x))
    }

    fn ACTION<'a>(&self, state_idx: i32, X: Symbol<'text>) -> Option<&'a (String, i32, Option<Item>)> {
	self._ACTION.get(&(state_idx, X))
	
    }
    
    fn print_state(&self, stack: &Stack<i32>, stack2: &Stack<Symbol>, production: &str) {
	println!("  {:<20}\t\t{:<40}\t{} ", repr_right(stack.items()), repr_right(stack2.items()), production);
    }
    fn print(&self) {
	println!("Print state table");
	for (state_idx, items) in self._state.iter().enumerate() {
	    println!("{}", state_idx);
	    for item in items.iter() {
		println!("\t{:?}", item.to_string());
	    }

	}
	println!("Pirnt GOTO table");
	println!("{:?}", self._GOTO);
	println!("Print ACTION table");
	println!("{:?}", self._ACTION);
    }
    fn parse(&self, s: &str) {
	println!("\nParse string:{}", s);

	let mut ss = s.split_whitespace().collect::<Vec<&str>>();
	ss.push(ENDMARKER);
	
	let mut stack = Stack::new();
	let mut stack_symbol = Stack::new();

	let mut cache = vec![0];
	stack.push(0); //push state 0
	stack_symbol.push(ENDMARKER);

	let mut idx = 0;
	let mut t = ss[idx];
	loop {
	    if let Some(&state) = stack.peek() {
		let res = self.ACTION(state, t);
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

				stack_symbol.push(head);
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
			    stack_symbol.push(t);
			    stack.push(act.1);
			    idx += 1;
			    if idx >= ss.len() {
				break;
			    }
			    t = ss[idx];
			    
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

	let state = vec![Item { head: "E", body: vec!["T", "E'"], dot: 0, lookahead: vec![] },
			 Item { head: "T", body: vec!["F", "T'"], dot: 0, lookahead: vec![] },
			 Item { head: "F", body: vec!["(", "E", ")"], dot: 0, lookahead: vec![] },
			 Item { head: "F", body: vec!["id"], dot: 0, lookahead: vec![] }];
	
	let cfg = CFG::new(s);
	let mut slr = SLR::new(&cfg);
	let items = slr.closure2("E");
	println!("{:?}", items);
	
	assert!(are_states_same(&items, &state));

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

	let state = vec![Item { head: "E", body: vec!["T", "E'"], dot: 0, lookahead: vec![] },
			 Item { head: "T", body: vec!["F", "T'"], dot: 0, lookahead: vec![] },
			 Item { head: "F", body: vec!["(", "E", ")"], dot: 0, lookahead: vec![] },
			 Item { head: "F", body: vec!["id"], dot: 0, lookahead: vec![] }];
	
	let cfg = CFG::new(s);
	let mut slr = SLR::new(&cfg);
	let mut I0 = cfg.get_production("E").deconstruct();
	I0.insert(0, Item{head:"S'", body: vec![cfg.S], dot:0, lookahead: vec![]});
	slr.closure(&mut I0);

	let next_symbols = slr.get_next_goto_X(&I0);
	println!("Xs:{:?}", next_symbols);
	assert_eq!(&next_symbols, &vec!["E", "T", "F", "(", "id"]);

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

	
	let state0 = slr.get_state_items(0);
	let state1 = slr.get_state_items(1);	
	println!("I0:{:?}", &state0);	
	println!("I1:{:?}", &state1);
	assert!(are_states_same(
	    &vec![Item { head: "E", body: vec!["T", "E'"], dot: 0, lookahead: vec![] },
		  Item { head: "T", body: vec!["F", "T'"], dot: 0, lookahead: vec![] }],
	    &state1));
    }

}

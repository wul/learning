mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON, ENDMARKER};
use std::collections::{HashSet, HashMap};

mod stack;
use stack::Stack;

use std::fmt::{Display, Write};

struct SLR <'a> {
    cfg: &'a CFG<'a>,
    _ACTION: HashMap<(i32, Symbol<'a>), (String, i32, Option<Item<'a>>)>,
    _GOTO: HashMap<(i32, Symbol<'a>), i32>,
    _state:Vec<State<'a>>,
}


fn are_states_same(me: &State, other: &State) -> bool {
    let mut lst1: Vec<String>  = other.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    lst1.sort();
    me.iter().for_each(|x| {lst1.push(x.to_string());});
    let mut lst2: Vec<String>  = other.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    lst2.sort();


    let mut ret;
    if lst1 == lst2 {
	ret= true;
    } else {
	ret= false;
    }
    if me.len() == 1 {
	println!("\n\n\n\n\nCompare state\n{:?}\nwith\n{:?}\n, RET={}", me, other, ret);
    }
    return ret;
    
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
	if are_states_same(st, state) {
	    return true;
	}
    }
    return false;
}
impl<'a> SLR <'a>{
    fn new(cfg: &'a CFG) -> Self{
	SLR {
	    cfg, 
	    _ACTION: HashMap::new(),
	    _GOTO: HashMap::new(),
	    _state: Vec::new(),
	}
    }


    fn closure(&self, mut items: Vec<Item<'a>>) -> State<'a>{
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
	return items;
    }

    fn closure2(&self, nt: Symbol<'a>) -> State<'a>{
	let production = self.cfg.get_production(nt);
	let mut state = production.deconstruct();

	return self.closure(state);
	//return state;
    }
    fn init(&mut self) {
	self.build_items_sets();
    }

	    
    fn get_state_idx(&self, state: & State) -> i32 {
	let mut idx = -1;
	for st in self._state.iter() {
	    idx += 1;
	    if are_states_same(st, state) {
		return idx as i32;
	    }
	}

	return -1;
    }

    fn get_next_goto_X(&self, state: &State<'a>) -> Vec<Symbol<'a>>{
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

    fn discover(&self, state_idx: i32, X: Symbol<'a>) -> State<'a> {
	let state = &self._state[state_idx as usize];
	let mut st = Vec::new();

	for item in state.iter() {
	    if item.dot == item.body.len() {
		continue;
	    }

	    if item.body[item.dot] == X {
		let it = Item{head:item.head,
				body:item.body.clone(),
				dot :item.dot+1,
				lookahead:vec![],
		};
		st.push(it);
	    }
	}
	return self.closure(st);
	//return st;
    }

    
    fn contains_state(&self, state: &State) -> bool {
	for st in self._state.iter() {
	    if are_states_same(st, state) {
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
	let state = self.closure(state);
	self._state.push(state);
	self.traversal_state();
    }

    fn traversal_state(&mut self) {
	let mut processed = 0;
	loop {
	    let total = self._state.len() as i32;
	    
	    let mut found_new = false;

	    for idx in processed .. total {
		println!("processed {}, total {}", processed,total);
		let state_idx = idx;

		
		let mut discovered_states = HashMap::<Symbol, State>::new();

		let state = &self._state[state_idx as usize];
		for item in state.iter() {
		    if item.body.len() == item.dot {
			if item.head == "S'" && item.body.last() == Some(&self.cfg.S) {
			    self._ACTION.insert((state_idx, ENDMARKER), ("Accept".to_string(), -1, None));
			} else {
			    for &t in self.cfg.FOLLOW(item.head).iter() {
				self._ACTION.insert((state_idx, t), ("Reduce".to_string(), -1, Some(item.clone())));
			    }
			}
		    }
		}
		
		let next_symbols = self.get_next_goto_X(state);
		
		for &X in next_symbols.iter() {
		    let new_state = self.discover(state_idx, X);
		    println!("Got new state {:?}", new_state);
		    if !state_in_state_set(&new_state, &discovered_states.values().cloned().collect()) {
			discovered_states.insert(X, new_state);
		    }
		    found_new = true;
		}
		println!("got total {} new states", discovered_states.len());

		//for (X, state) in discovered_states.into_iter() {
		let mut cur_idx = idx;
		for &X in next_symbols.iter() {
		    let state = discovered_states.remove(X).unwrap();
		    if !self.contains_state(&state) {
			println!("state not in _state list:\n{:?}", &state);
			cur_idx += 1;
		    } 
		    self._state.push(state);
		    
		    if self.cfg.is_terminal(X) {
			self._ACTION.insert((state_idx, X), ("Shift".to_string(), cur_idx, None));
		    } else {
			self._GOTO.insert((state_idx, X), cur_idx);
		    }
		}
		processed += 1;
	    } //state loop in pool

	    if processed == self._state.len() as i32 {
		break;
	    }
	    self.print();
	}
    }
		

							    
    fn GOTO(&self, state_idx: i32, X: Symbol) -> Option<i32> {
	self._GOTO.get(&(state_idx, X)).and_then(|x| Some(*x))
    }

    fn ACTION(&self, state_idx: i32, X: Symbol<'a>) -> Option<&'a (String, i32, Option<Item>)> {
	self._ACTION.get(&(state_idx, X))
	
    }
    
    fn print_state(&self, stack: &Stack<i32>, stack2: &Stack<Symbol>, production: &str) {
	println!("  {:<20}\t\t{:<40}\t{} ", repr_right(stack.items()), repr_right(stack2.items()), production);
    }
    fn print(&self) {
	println!("Print state table");
	for (state_idx, items) in self._state.iter().enumerate() {
	    println!("{}", state_idx);
	    for item in items {
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

mod cfg;
use cfg::{Symbol, CFG, Item, State, EPSILON, ENDMARKER};
use std::collections::{HashSet, HashMap};

mod stack;
use stack::Stack;


struct SLR <'a> {
    cfg: &'a CFG,
    _action: HashMap<(i32, Symbol), (String, i32, Option<Item>)>,
    _goto: HashMap<(i32, Symbol), i32>,
    _state:Vec<State>,
    state_index: i32,
}



fn repr_right<T>(items: &Vec<T>) -> String
where T: std::fmt::Display,
{
    let mut out = "| ".to_string();
    items.iter().for_each(|x| {out = out.to_owned() + &x.to_string() + " "});
    out
	
}

#[allow(dead_code)]
fn repr_left(items: &Vec<Symbol>) -> String{
    let mut cloned = items.clone();
    cloned.reverse();
    format!("{}", cloned.join(" "))
}

impl<'a> SLR <'a>{
    fn new(cfg: &'a CFG) -> Self{
	SLR {
	    cfg, 
	    _action: HashMap::new(),
	    _goto: HashMap::new(),
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
    #[allow(dead_code)]
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

    fn get_next_goto_x(&self, state: &State) -> Vec<Symbol>{
	let mut xs = Vec::<Symbol>::new();
	for item in state.iter() {
	    let body = &item.body;
	    if item.dot < body.len() && body[item.dot] != EPSILON {
		let t = &body[item.dot];
		if !xs.contains(t) {
		    xs.push(t.clone());
		}
	    }
	}
	return xs
    }

    fn discover(&self, state_idx: i32, edge: &Symbol) -> State {
	let state = &self._state[state_idx as usize];
	let mut st = State::new();

	for item in state.iter() {
	    if item.dot == item.body.len() {
		continue;
	    }

	    if item.body[item.dot] == *edge {
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
    
    #[allow(dead_code)]
    fn get_state(&self, state_idx: i32) -> &State{
	&self._state[state_idx as usize]
    }
    fn build_items_sets(&mut self) {
	let mut state = State::from(vec![Item {head: "S'".to_string(),
				   body: vec![self.cfg.start_symbol.clone()],
				   dot:0,
				   lookahead:HashSet::new()}]);
	self.closure(&mut state);
	self._state.push(state);
	self.state_index = 0;
	self.traversal_state();
    }



    fn traversal_state(&mut self) {
	let mut processed = 0;
	while processed < self._state.len() {

	    let total = self._state.len();
	    let mut top:i32 = (total as i32) - 1;
	    
	    for index in processed .. total {
		let state_idx = index as i32;

		//why not work by using function
		//let state = self.get_state(state_idx as i32);
		let state = &self._state[index];
		
		for item in state.iter() {
		    if item.body.len() == item.dot {
			if item.head == "S'" && item.body.last() == Some(&self.cfg.start_symbol) {
			    self._action.insert((state_idx, ENDMARKER.to_string()),
						("Accept".to_string(), -1, None));
			} else {
			    for t in self.cfg.follow(&item.head).iter() {
				self._action.insert((state_idx, t.clone()),
						    ("Reduce".to_string(), -1, Some(item.clone())));
			    }
			}
		    }
		}		

		let next_symbols = self.get_next_goto_x(state);

		for edge in next_symbols.iter() {
		    let new_state = self.discover(state_idx, edge);
		    let mut idx = self.get_state_idx(&new_state);
		    if idx == -1 {
			self._state.push(new_state);
			top += 1;
			idx = top;
		    }
		    if self.cfg.is_terminal(edge) {
			self._action.insert((state_idx, edge.clone()), ("Shift".to_string(), idx, None));}
		    else {
			self._goto.insert((state_idx, edge.clone()), idx);
		    }

		}
	    } //done of _state loop
	    processed = total;

	}

    }
		

							    
    fn goto(&self, state_idx: i32, x: &Symbol) -> Option<i32> {
	self._goto.get(&(state_idx, x.clone())).and_then(|x| Some(*x))
    }

    fn action(&self, state_idx: i32, x: &Symbol) -> Option<&(String, i32, Option<Item>)> {
	self._action.get(&(state_idx, x.clone()))
	
    }
    
    fn print_state(&self, stack: &Stack<i32>, stack2: &Stack<Symbol>, remained: &[String]) {
	let s = remained.join("");
	println!("  {:<30}\t\t{:<35}\t{} ", repr_right(stack.items()), repr_right(stack2.items()), s);
    }
    fn print(&self) {
	println!("Print state table");
	for (state_idx, state) in self._state.iter().enumerate() {
	    println!("I{}", state_idx);
	    println!("{}", state);
	}
	println!("Pirnt GOTO table");
	for (k, v) in self._goto.iter() {
	    let (state_idx, symbol) = k;
	    println!("\t [I{:<2},{:2}]\t:\t{}", state_idx, symbol, v);
	}

	println!("Print ACTION table");
	for (k, v) in self._action.iter() {
	    let (state_idx, symbol) = k;
	    let act_s;
	    let (ref act, ref st, ref args) = v;
	    if act == "Reduce" {
		act_s =  format!("r - {}", args.as_ref().map(|x| x.to_string()).unwrap());
	    } else if act == "Shift" {
		act_s =  format!("s{}", st);
	    } else { //accept
		act_s =  format!("acc {}", st);
	    } 
		
	    println!("\t [I{:<2},{:2}]\t:\t{:?}", state_idx, symbol, act_s);
	}

    }

    
    fn parse(&self, s: &str) {
	println!("\nParse string:{}", s);

	let mut ss: Vec<String> = s.split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
	ss.push(ENDMARKER.to_string());
	
	let mut stack = Stack::new();
	let mut stack_symbol = Stack::new();

	stack.push(0); //push state 0
	stack_symbol.push(ENDMARKER.to_string());

	let mut idx = 0;
	let mut t = &ss[idx];

	loop {
	    if let Some(&state) = stack.peek() {
		let res = self.action(state, &t);
		let remained = &ss[idx..];		
		self.print_state(&stack, &stack_symbol, remained);
		match res {
		    Some(act) => {
			if act.0 == "Accept" {
			    //do nothing
			    if remained[0] == ENDMARKER {
				println!("Done, accepted");
				break;
			    } else {
				panic!("Accept, but there are still input");
			    }

			} else if act.0 == "Reduce" {
			    if let Some(ref item) = act.2 {
				let head = &item.head;
				let mut length = item.body.len();
				while length > 0 {
				    stack_symbol.pop();
				    stack.pop();
				    length -= 1;
				}

				stack_symbol.push(head.clone());
				if let Some(&state) = stack.peek() {
				    if let Some(next) = self.goto(state, head) {
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
				println!("End of input");
				break;
			    }
			    t = &ss[idx];
			    
			}
		    },
		    None => {
			println!("WHY NONE, t={}", t);
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
    slr.parse("id * ( id + id * ( id * ( id + id ) ) )");


    let s = "
                 E  -> T E'      
                 E' -> + T E' | ε
                 T  -> F T'
                 T' -> * F T' | ε
                 F  -> ( E ) | id
                ";

    let cfg = CFG::new(s);
    let mut slr = SLR::new(&cfg);
    slr.init();
    slr.print();
    slr.parse("id + id + id" );    
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
    fn test_get_next_goto_x() {
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

	let next_symbols = slr.get_next_goto_x(&I0);
	println!("xs:{:?}", next_symbols);
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

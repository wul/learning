use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
mod stack;
use stack::Stack;
use std::mem;



type Symbol = String;
type Body = Vec<Symbol>;

#[derive(Debug, Copy, Clone)]
struct BodyIndex(u32, u32);


const EPSILON:&str   = "ε";
const ENDMARKER:&str = "$";

#[derive(Debug)]
pub struct Production{
    rank: usize,
    head: Symbol, 
    bodies: Vec<Body>,
}

#[derive(Debug)]
pub struct Item{
    head:  Symbol,
    body:  BodyIndex,
    dot:   u32,
    lookahead: Vec<Symbol>,
}

#[derive(Debug)]
pub struct CFG {
    pub S:  Symbol,
    pub T:  HashSet<Symbol>,
    pub NT: HashSet<Symbol>,
    pub P:  Vec<Production>,

    loc : HashMap<Symbol, usize>,
    _FIRST: HashMap<Symbol, HashMap<Symbol, BodyIndex>>,
    _FOLLOW: HashMap<Symbol, HashSet<Symbol>>,
}



const PRODUCTIONS: &str = "
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
";
/*
fn body2item (head: Symbol, body: &Vec<Symbol>) -> Item {
    let mut item = Item {
	head,
	dot: 0,
	lookahead: Vec::new(),
	body: BodyIndex(0,0),
    };
    
    for x in body {
	item.body.push(x.clone());
    }
    item
}
*/

impl Production {
    pub fn new<'a>(rank: usize, s: &'a str) -> Self {
	let mut p = Production {
	    rank,
	    head: "".to_string(),
	    bodies: Vec::new(),
	};

	p.parse(s);
	return p;
    }

    pub fn parse (&mut self, s: &str) {
	let v =  s.split("->").collect::<Vec<&str>>();
	self.head = v[0].trim().to_string();
	for body_str in v[1].trim().split("|") {
	    let mut body = Vec::new();
	    for token_str in body_str.split_whitespace() {
		body.push(token_str.trim().to_string());
	    }
	    self.bodies.push(body);
	}
    }    
}

impl PartialEq for Production {
    fn eq(&self, other: &Production) -> bool{
	return self.rank == other.rank && self.head == other.head;
    }

}

impl Hash for Production {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rank.hash(state);
        self.head.hash(state);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Item)->bool {
	return self.head == other.head &&
	    self.body.0 == other.body.0 &&
	    self.body.1 == other.body.1 &&
	    self.dot == other.dot &&
	    self.lookahead == other.lookahead;
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.head.hash(state);
	self.body.0.hash(state);
	self.body.1.hash(state);
	self.dot.hash(state);
    }
}


impl CFG  {
    fn new(s: &str) -> Self {
	let mut cfg = Self {
	    T: HashSet::new(),
	    NT: HashSet::new(),
	    P: Vec::new(),
	    S: "".to_string(),
	    loc: HashMap::new(),
	    _FIRST:HashMap::new(),
	    _FOLLOW:HashMap::new(),
	};

	cfg.normalize(s);
	return cfg;
    }

    fn normalize(&mut self, s: &str) {
	let mut idx:usize = 0;
	let symbol_set = s.replace("->", "").replace("|", " ").split_whitespace();
	//let symbol_set = symbol_set.clone().collect::<HashSet<Symbol>>();

	for line in s.lines() {
	    let line = line.trim();
	    if line.is_empty() {
		continue;
	    }

	    let production = Production::new(idx, line);
	    
	    if self.S.is_empty() {
		self.S = production.head.clone();
	    }
	    self.NT.insert(production.head.clone());	    
	    self.loc.insert(production.head.clone(), idx);
	    self.P.push(production);

	    idx += 1;
	}

	//self.T = symbol_set.different(self.NT);
	self.calculate_first_set();
	self.calculate_follow_set();
	

    }

    fn is_non_terminal(&self, symbol: &Symbol) -> bool {
	return self.NT.contains(symbol);
    }

    fn is_terminal(&self, symbol: &Symbol) -> bool {
	return !self.is_non_terminal(symbol);
    }

    fn get_production(&self, symbol: &Symbol)-> &Production {
	if let Some(idx) = self.loc.get(symbol) {
	    let production = &self.P[*idx];
	    return production;
	} else {
	    return &self.P[0];
	}
    }

    fn first_set(&self, beta: &Vec<Symbol>) -> HashSet<Symbol> {

	let mut symbols = HashSet::<Symbol>::new();
	for symbol in beta.into_iter() {
	    if self.is_terminal(symbol) {
		symbols.insert(symbol.clone());
		break;
	    } else {
		match self._FIRST.get(symbol) {
		    Some(m) => {
			//does not work
			//m.keys().map(|x| symbols.insert(x.clone()));
			for x in m.keys() {
			    symbols.insert(x.clone());
			}
			if !symbols.contains(&EPSILON.to_string()) {
			    break;
			}
		    }
		    None => {
			break;
		    }
		}
	    }
	}

	return symbols;
    }
    
    fn set_first_set(&mut self, nt: Symbol, symbol_set: HashSet<Symbol>, bi: BodyIndex) {
	
	if let Some(m) = self._FIRST.get_mut(&nt) {
	    for x in symbol_set.into_iter() {
		m.insert(x, bi);
	    };
	} else {
	    let mut  m = HashMap::new();
	    for x in symbol_set.into_iter() {			
		m.insert(x, bi);
	    }
	    self._FIRST.insert(nt.clone(), m);
	}

    }

    fn get_first_set(&self, nt: &Symbol) -> HashSet<Symbol>{
	let mut set = HashSet::<Symbol>::new();
	    
	if let Some(m) = self._FIRST.get(nt) {
	    for x in m.keys() {
		set.insert(x.clone());
	    };
	}

	return set;

    }

    fn print_first(&self) {
	for nt in self.NT.iter() {
	    println!("FIRST of {}\t:\t{:?}", nt, self.get_first_set(nt));
	}
    }
    
    fn calculate_first_set(&mut self) {
	
	let mut non_terminals = HashSet::new();
	//why it does not work
	//self.NT.into_iter().map(|x| non_terminals.insert(x.clone()));
	for x in self.NT.iter() {
	    non_terminals.insert(x.clone());
	}
	println!("{:?}", self.NT);	
	println!("{:?}", non_terminals);
	loop {
	    let mut found_new = false;

	    for X in non_terminals.iter() {

		let mut productions = std::mem::take(&mut self.P);
		let mut pidx = 0;

		for production in productions.iter() {
		    if production.head != *X {
			pidx += 1;
			continue
		    }

		    let mut idx = 0;

		    for body in production.bodies.iter() {
			let beta = body;
			println!("check head {} against {:?}", X, beta);
			let symbol_set = self.first_set(beta);
			println!("first set:{:?}", symbol_set);
			if !symbol_set.is_empty() && !symbol_set.is_subset(&self.get_first_set(X)){
			    let bi = BodyIndex(pidx, idx);
			    self.set_first_set(X.clone(), symbol_set, bi);
			    found_new = true;
			}
			idx += 1;
		    }
		    pidx += 1;
		
		}
		self.P = productions;
		
		
	    }

	    if !found_new  {
		break;
	    }

	}
    }

    fn calculate_follow_set(&self) {
    }
}

fn main() {
    let mut idx = 0;
    for line in PRODUCTIONS.lines() {
	if line.is_empty() {
	    continue;
	}
	let production = Production::new(idx, line);
	println!("{:?}", production);
	idx += 1;
    }
    println!("Build CFG for string:\n{}", PRODUCTIONS);
    let mut cfg = CFG::new(PRODUCTIONS);
    cfg.print_first();



    let productions = "
    E -> E + T | T
    T -> T * F | F
    F -> ( E ) | id
    ";
    let cfg = CFG::new(productions);
    cfg.print_first();

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {

	assert_eq!(1,1);
    }
}

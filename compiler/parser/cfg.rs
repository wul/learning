use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::mem;



pub type Symbol<'a> = &'a str;

type Body<'a> = Vec<Symbol<'a>>;

    
struct Token<'a, T> {
    token_type: u32,
    attr_value: Option<T>,
    symbol:     Symbol<'a>,
}



pub const EPSILON:&str   = "ε";
pub const ENDMARKER:&str = "$";

#[derive(Debug, Clone)]
pub struct Production<'a>{
    rank: usize,
    head: Symbol<'a>, 
    bodies: Vec<Body<'a>>,
}

#[derive(Debug, Eq, Clone)]
pub struct Item<'a>{
    pub head:  Symbol<'a>,
    pub body:  Body<'a>,
    pub dot:   usize,
    pub lookahead: Vec<Symbol<'a>>,
}


pub type State<'a> = Vec<Item<'a>>;

#[derive(Debug)]
pub struct CFG<'a> {
    pub S:  Symbol<'a>,
    pub T:  HashSet<Symbol<'a>>,
    pub NT: HashSet<Symbol<'a>>,
    pub P:  Vec<Production<'a>>,

    loc : HashMap<Symbol<'a>, usize>,
    _FIRST: HashMap<Symbol<'a>, HashMap<Symbol<'a>, Body<'a>>>,
    _FOLLOW: HashMap<Symbol<'a>, HashSet<Symbol<'a>>>,
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

impl<'a> Production<'a> {
    pub fn new(rank: usize, s: &'a str) -> Self {
	let mut p = Production {
	    rank,
	    head: "",
	    bodies: Vec::new(),
	};

	p.parse(s);
	return p;
    }

    pub fn parse (&mut self, s: &'a str) {
	let v =  s.split("->").collect::<Vec<&str>>();
	self.head = v[0].trim();
	for body_str in v[1].trim().split("|") {
	    let mut body = Vec::new();
	    for token_str in body_str.split_whitespace() {
		body.push(token_str.trim());
	    }
	    self.bodies.push(body);
	}
    }

    pub fn deconstruct(&self) -> Vec<Item> {
	self.bodies.iter().map(|x| Item {head:self.head, body:x.clone(), dot:0, lookahead:Vec::new()}).collect::<Vec<Item>>()
    }
}

impl<'a> PartialEq for Production<'a>{
    fn eq(&self, other: &Production) -> bool{
	return self.rank == other.rank && self.head == other.head;
    }

}

impl<'a> Hash for Production<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rank.hash(state);
        self.head.hash(state);
    }
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Item)->bool {
	return self.head == other.head &&
	    self.body ==  other.body &&
	    self.dot == other.dot &&
	    self.lookahead == other.lookahead;
    }
}

impl<'a> Hash for Item<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.head.hash(state);
	for x in self.body.iter() {
	    x.hash(state);
	}

	self.dot.hash(state);
    }
}


impl<'a> CFG<'a>  {
    fn new(s: &'a str) -> Self {
	let mut cfg = Self {
	    T: HashSet::new(),
	    NT: HashSet::new(),
	    P: Vec::new(),
	    S: "",
	    loc: HashMap::new(),
	    _FIRST:HashMap::new(),
	    _FOLLOW:HashMap::new(),
	};

	cfg.normalize(s);
	return cfg;
    }

    pub fn normalize(&mut self, s: &'a str) {
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
		self.S = production.head;
	    }
	    self.NT.insert(production.head);	    
	    self.loc.insert(production.head, idx);
	    self.P.push(production);

	    idx += 1;
	}

	//self.T = symbol_set.different(self.NT);
	self.calculate_first_set();
	self.calculate_follow_set();
	

    }

    pub fn is_non_terminal(&self, symbol: Symbol) -> bool {
	return self.NT.contains(symbol);
    }

    pub fn is_terminal(&self, symbol: Symbol) -> bool {
	return !self.is_non_terminal(symbol);
    }

    pub fn get_production(&self, symbol: Symbol)-> &Production {
	if let Some(idx) = self.loc.get(symbol) {
	    let production = &self.P[*idx];
	    return production;
	} else {
	    return &self.P[0];
	}
    }



    pub fn first_set(&self, beta: &Vec<Symbol<'a>>) -> HashSet<Symbol<'a>> {

	let mut symbols = HashSet::<Symbol>::new();
	for symbol in beta.iter() {
	    if self.is_terminal(symbol) {
		symbols.insert(symbol);
		break;
	    } else {
		match self._FIRST.get(symbol) {
		    Some(m) => {
			//does not work
			//m.keys().map(|x| symbols.insert(x.clone()));
			for x in m.keys() {
			    symbols.insert(x);
			}
			if !symbols.contains(&EPSILON) {
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
    
    pub fn set_first_set(&mut self, nt: Symbol<'a>, symbol_set: HashSet<Symbol<'a>>, body:  Body<'a>) {
	
	if let Some(m) = self._FIRST.get_mut(&nt) {
	    for x in symbol_set.into_iter() {
		m.insert(x, body.clone());
	    };
	} else {
	    let mut  m = HashMap::new();
	    for x in symbol_set.into_iter() {			
		m.insert(x, body.clone());
	    }
	    self._FIRST.insert(nt, m);
	}

    }

    pub fn get_first_set(&self, nt: Symbol) -> HashSet<Symbol>{
	let mut set = HashSet::<Symbol>::new();
	    
	if let Some(m) = self._FIRST.get(nt) {
	    for x in m.keys() {
		set.insert(x);
	    };
	}

	return set;

    }

    pub fn print_first(&self) {
	for nt in self.NT.iter() {
	    println!("FIRST of {}\t:\t{:?}", nt, self.get_first_set(nt));
	}
    }
    
    pub fn calculate_first_set(&mut self) {
	//Get all non-terminals
	let non_terminals = self.NT.iter().map(|x| *x).collect::<HashSet::<Symbol>>().clone();

	loop {
	    let mut found_new = false;

	    for X in non_terminals.iter() {
		let productions = std::mem::take(&mut self.P);

		for production in productions.iter() {
		//for production in self.P.iter() {		    
		    if production.head != *X {
			continue
		    }

		    for body in production.bodies.iter() {
			let beta = body;

			println!("check head {} against {:?}", X, beta);
			let symbol_set = self.first_set(beta);
			println!("first set:{:?}", symbol_set);
			if !symbol_set.is_empty() && !symbol_set.is_subset(&self.get_first_set(*X)){
			    
			    self.set_first_set(*X, symbol_set, beta.clone());

			    found_new = true;
			}

		    }

		}
		self.P = productions;
	    }

	    if !found_new  {
		break;
	    }

	}
    }

    pub fn calculate_follow_set(&self) {
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

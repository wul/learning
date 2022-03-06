use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::slice::Iter;
use std::fmt::{Display, Formatter, Result};


pub type Symbol = String;

type Body = Vec<Symbol>;

#[allow(dead_code)]    
struct Token<T> {
    token_type: u32,
    attr_value: Option<T>,
    symbol:     Symbol,
}



pub const EPSILON:&str   = "ε";
pub const ENDMARKER:&str = "$";

#[derive(Debug, Clone)]
pub struct Production{
    rank: usize,
    head: Symbol, 
    bodies: Vec<Body>,
}

//PartialEq is defined by us
#[derive(Debug, Eq, Clone)]
pub struct Item{
    pub head:      Symbol,
    pub body:      Body,
    pub dot:       usize,
    pub lookahead: HashSet<Symbol>,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
	let mut s = String::from("");
	let len = self.body.len();
	
	for idx in 0..len {
	    if idx == self.dot {
		s += " · ";
	    }
	    s += " ";
	    s += &self.body[idx];
	    s +=  " ";
	}

	if self.dot >= len {
	    s += " ·";
	}
	
	let n = self.lookahead.iter().map(|x| x.clone()).collect::<Vec<Symbol>>().join("/");
	write!(f, "{}\t ->\t {}, {}", self.head, s, n)
    }
}

//pub type State = Vec<Item>;
//pub type State<'a> = Vec<Item<'a>>;
#[derive(Debug, Clone)]
pub struct  State{
    items: Vec<Item>,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
	let v = self.items.iter().fold("".to_string(), |acc, x| acc + "\t" + &x.to_string() + "\n");
	write!(f, "{}", v)
    }
}
impl State {
    pub fn new() -> Self {
	State {
	    items: Vec::<Item>::new(),
	}
    }
    pub fn push(&mut self, item: Item) {
	self.items.push(item);
    }
    pub fn contains(&self, item: &Item) -> bool {
	self.items.contains(item)
    }
	

    pub fn iter(&self) -> Iter<Item> {
	self.items.iter()
	
    }

    /*
    pub fn into_iter(&self) -> IntoIter<Item> {
	self.items.into_iter()
    }
    */
}

impl From<Vec<Item>> for State {
    fn from(items: Vec<Item>) -> Self{
	Self{items}
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
	if self.items.len() != other.items.len() {
	    return false;
	}

	for item in self.items.iter() {
	    if !other.items.contains(item) {
		return false;
	    }
	}

	true
    }
}





#[derive(Debug)]
pub struct CFG {
    pub start_symbol:  Symbol,
    pub terminals:  HashSet<Symbol>,
    pub non_terminals: HashSet<Symbol>,
    pub productions:  Vec<Production>,

    loc : HashMap<Symbol, usize>,
    _first: HashMap<Symbol, HashMap<Symbol, Body>>,
    _follow: HashMap<Symbol, HashSet<Symbol>>,
}


impl Production {
    pub fn new(rank: usize, s: &str) -> Self {
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

    pub fn deconstruct(&self) -> Vec<Item> {
	self.bodies.iter().map(|x| Item {head:self.head.clone(),
					 body:x.clone(),
					 dot:0,
					 lookahead:HashSet::new()}).collect::<Vec<Item>>()

    }
}

impl PartialEq for Production{
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
	    self.body ==  other.body &&
	    self.dot == other.dot &&
	    self.lookahead == other.lookahead;
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.head.hash(state);
	for x in self.body.iter() {
	    x.hash(state);
	}

	self.dot.hash(state);
    }
}


impl CFG  {
    pub fn new(s: &str) -> Self {
	let mut cfg = Self {
	    terminals: HashSet::new(),
	    non_terminals: HashSet::new(),
	    productions: Vec::new(),
	    start_symbol: "".to_string(),
	    loc: HashMap::new(),
	    _first:HashMap::new(),
	    _follow:HashMap::new(),
	};

	cfg.normalize(s);
	return cfg;
    }

    pub fn normalize(&mut self, s: &str) {
	let mut idx:usize = 0;

	let ss = s.to_owned();
	for line in ss.lines() {
	    let line = line.trim();
	    if line.is_empty() {
		continue;
	    }

	    let production = Production::new(idx, line);
	    
	    if self.start_symbol.is_empty() {
		self.start_symbol = production.head.clone();
	    }
	    self.non_terminals.insert(production.head.clone());	    
	    self.loc.insert(production.head.clone(), idx);
	    self.productions.push(production);

	    idx += 1;
	}

	//FIRST should be calculated firstly
	self.calculate_first_set();
	self.calculate_follow_set();


    }

    pub fn is_non_terminal(&self, symbol: &Symbol) -> bool {
	return self.non_terminals.contains(symbol);
    }

    pub fn is_terminal(&self, symbol: &Symbol) -> bool {
	return !self.is_non_terminal(symbol);
    }

    pub fn get_production(&self, symbol: &Symbol)-> &Production {
	if let Some(idx) = self.loc.get(symbol) {
	    let production = &self.productions[*idx];
	    return production;
	} else {
	    return &self.productions[0];
	}
    }



    pub fn first_set(&self, beta: &Vec<Symbol>) -> HashSet<Symbol> {

	let mut symbols = HashSet::<Symbol>::new();
	for symbol in beta.iter() {
	    if self.is_terminal(symbol) {
		symbols.insert(symbol.clone());
		break;
	    } else {
		match self._first.get(symbol) {
		    Some(m) => {
			//does not work
			//m.keys().map(|x| symbols.insert(x.clone()));
			for x in m.keys() {
			    symbols.insert(x.clone());
			}
			if !symbols.contains(EPSILON) {
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
    
    pub fn set_first_set(&mut self, nt: &Symbol, symbol_set: HashSet<Symbol>, body:  Body) {
	
	if let Some(m) = self._first.get_mut(nt) {
	    for x in symbol_set.into_iter() {
		m.insert(x, body.clone());
	    };
	} else {
	    let mut  m = HashMap::new();
	    for x in symbol_set.into_iter() {			
		m.insert(x, body.clone());
	    }
	    self._first.insert(nt.clone(), m);
	}

    }

    pub fn get_first_set(&self, nt: &Symbol) -> HashSet<Symbol>{
	let mut set = HashSet::<Symbol>::new();
	    
	if let Some(m) = self._first.get(nt) {
	    for x in m.keys() {
		set.insert(x.clone());
	    };
	}

	return set;

    }

    fn first2(&self, symbol: &Symbol) -> HashSet<Symbol> {
	let v = vec![symbol.clone()];
	self.first(&v)
    }
	
    fn first(&self, beta: &Vec<Symbol>) -> HashSet<Symbol> {
	let mut ret = HashSet::<Symbol>::new();
	for symbol in beta.iter() {
	    let mut symbols = HashSet::<Symbol>::new();
	    let mut can_exit = false;
	    if self.is_terminal(symbol) {
		symbols.insert(symbol.clone());
	    } else {
		if let Some(dct) = self._first.get(symbol) {
		    //dct is a reference to hashmap
		    symbols = dct.keys().map(|x| x.clone()).collect::<HashSet<Symbol>>();
		    
		    if symbols.contains(EPSILON) {
			can_exit = true;
		    }

		}
	    }

	    symbols.into_iter().for_each(|x| {ret.insert(x);});


	    if can_exit {
		break
	    }
	}

	return ret;
    }
	    
		
	    
    #[allow(dead_code)]	
    fn print_first(&self) {
	for nt in self.non_terminals.iter() {
	    println!("FIRST of {}\t:\t{:?}", nt, self.get_first_set(nt));
	}
    }
    
    pub fn calculate_first_set(&mut self) {
	//Get all non-terminals
	let non_terminals = self.non_terminals.iter().map(|x| x.clone()).collect::<HashSet::<Symbol>>().clone();

	loop {
	    let mut found_new = false;

	    for symbol in non_terminals.iter() {
		let productions = std::mem::take(&mut self.productions);

		for production in productions.iter() {
		//for production in self.productions.iter() {		    
		    if production.head != *symbol {
			continue
		    }

		    for body in production.bodies.iter() {
			let beta = body;

			let symbol_set = self.first_set(beta);
			if !symbol_set.is_empty() && !symbol_set.is_subset(&self.get_first_set(symbol)){
			    
			    self.set_first_set(symbol, symbol_set, beta.clone());

			    found_new = true;
			}

		    }

		}
		self.productions = productions;
	    }

	    if !found_new  {
		break;
	    }

	}
    }
    pub fn calc_follow_set_relations(&self, left: &Symbol, right: &Body, rels: &mut HashMap<Symbol, HashSet::<Symbol>>) {
	//deduct the relations of FOLLOW sets
	for symbol in right.iter().rev() {
	    if self.is_terminal(&symbol) {
		break;
	    } else {
		if left != symbol {
		    if let Some(rel) = rels.get_mut(symbol) {
			rel.insert(left.clone());
		    } else {
			rels.insert(symbol.clone(), HashSet::from([left.clone()]));
		    }
		}
		if !self.first2(symbol).contains(EPSILON) {
		    break;
		}
	    }	    
	}
	
    }
    
    pub fn calc_direct_suffix(&self, right: &Body, cache: &mut HashMap<Symbol, HashSet::<Symbol>>) {
	let mut check_follow = false;
	let mut last_nt = "".to_string();
	for symbol in right.iter() {
	    let mut symbols = HashSet::<Symbol>::new();

	    if self.is_non_terminal(symbol) {
		if check_follow {
		    symbols = self.first2(symbol);
		    symbols.remove(EPSILON);
		    
		    if !last_nt.is_empty() {
			if let Some(ca) = cache.get_mut(&last_nt){
			    symbols.iter().for_each(|x| {ca.insert(x.clone());});
			} else {
			    cache.insert(last_nt.clone(), symbols);
			}
			
		    }
		}
		check_follow = true;
		last_nt = symbol.clone();
		
	    } else { //check it's a T/NT
		if check_follow {
		    symbols.insert(symbol.clone());
		    if !last_nt.is_empty() {
			if let Some(ca) = cache.get_mut(&last_nt) {
			    symbols.iter().for_each(|x| {ca.insert(x.clone());});
			} else {
			    cache.insert(last_nt.clone(), symbols);
			}
		    }
		}
		
		check_follow = false;
	    }
	    
	}	
    }


    pub fn calculate_follow_set(&mut self) {
	let mut rels:HashMap::<Symbol, HashSet<Symbol>> = HashMap::new();
	let mut cache: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();

	//Build direct suffixes and follow set relations first
	for production in self.productions.iter() {
	    for right in production.bodies.iter() {
		self.calc_direct_suffix(right, &mut cache);
		self.calc_follow_set_relations(&production.head, right, &mut rels);
	    }
	}
    
	match cache.get_mut(&self.start_symbol) {
	    Some(ca) => {ca.insert(ENDMARKER.to_string());},
	    None => {cache.insert(self.start_symbol.clone(), HashSet::from([ENDMARKER.to_string()]));},
	    
	}
    
    
	// Calculate all subset of FOLLOW
	for nt in self.non_terminals.iter() {	
	    let mut st = HashSet::new();
	    st.insert(nt.clone());
	    
	    let mut swap = st.clone();
	    loop {
		let tmp = swap.clone();
		swap = HashSet::new();
		for sym in tmp.iter() {
		    if let Some(v) = rels.get(sym) {
		    for x in v.iter() {
			swap.insert(x.clone());
		    }
		    }
		}
		
		if swap.len() == 0 {
		    break;
		} else {
		    swap.iter().for_each(|x| {st.insert(x.clone());});
		}
		
	    }


	    let mut symbol_set = HashSet::new();
	    if *nt == self.start_symbol {
		symbol_set.insert(ENDMARKER.to_string());
	    }
	    
	    cache.get(nt).map(|v| v.iter().for_each(|t| {symbol_set.insert(t.clone());}));

	    for symbol in st.iter() {
		if let Some(v) = cache.get(symbol) {
		    v.iter().for_each(|t| {symbol_set.insert(t.clone());});
		}
	    }
	    self._follow.insert(nt.clone(), symbol_set);	    
        }
    }

    pub fn follow(&self, nt: &Symbol) -> HashSet<Symbol>{
	let mut r = HashSet::new();
	if let Some(v) = self._follow.get(nt) {
	    v.iter().for_each(|x| {r.insert(x.clone()); });
	}

	r
    }
}



/*
const PRODUCTIONS: &str = "
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
";
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {

	assert_eq!(1,1);
    }

    #[test]
    fn test_follow() {
	let productions = "
	    E  -> T E'      
	    E' -> + T E' | ε
	    T  -> F T'
	    T' -> * F T' | ε
	    F  -> ( E ) | id
	    ";
	let cfg = CFG::new(productions);

	let set = cfg.follow(&"E".to_string());
	let set2 = HashSet::from([")".to_string(),
				  "$".to_string()]);
	assert_eq!(set, set2);

	let set = cfg.follow(&"E'".to_string());
	let set2 = HashSet::from([")".to_string(),"$".to_string()]);
	assert_eq!(set, set2);

	let set = cfg.follow(&"T".to_string());
	let set2 = HashSet::from(["+".to_string(), ")".to_string(),"$".to_string()]);
	assert_eq!(set, set2);

	let set = cfg.follow(&"T'".to_string());
	let set2 = HashSet::from(["+".to_string(), ")".to_string(),"$".to_string()]);
	assert_eq!(set, set2);

	let set = cfg.follow(&"F".to_string());
	let set2 = HashSet::from(["*".to_string(), "+".to_string(), ")".to_string(),"$".to_string()]);
	assert_eq!(set, set2);	
	

    }
}

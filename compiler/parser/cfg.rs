use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::mem;
use std::slice::Iter;
use std::vec::IntoIter;

pub type Symbol<'text> = &'text str;

type Body<'text> = Vec<Symbol<'text>>;

    
struct Token<'text, T> {
    token_type: u32,
    attr_value: Option<T>,
    symbol:     Symbol<'text>,
}



pub const EPSILON:&str   = "ε";
pub const ENDMARKER:&str = "$";

#[derive(Debug, Clone)]
pub struct Production<'text>{
    rank: usize,
    head: Symbol<'text>, 
    bodies: Vec<Body<'text>>,
}

#[derive(Debug, Eq, Clone)]
pub struct Item<'text>{
    pub head:      Symbol<'text>,
    pub body:      Body<'text>,
    pub dot:       usize,
    pub lookahead: Vec<Symbol<'text>>,
}

impl<'text> ToString for Item<'text> {
    fn to_string(&self) -> String {
	let mut body = self.body.clone();
	body.insert(self.dot, ".");
	format!("{} -> {}, {} {}", self.head, body.join(" "), self.dot, self.lookahead.join("/"))
    }
}

//pub type State<'text> = Vec<Item<'text>>;
#[derive(Debug, Clone)]
pub struct  State<'text>{
    items: Vec<Item<'text>>,
}


impl<'text> State<'text> {
    pub fn new() -> Self {
	State {
	    items: Vec::<Item<'text>>::new(),
	}
    }
    pub fn push(&mut self, item: Item<'text>) {
	self.items.push(item);
    }
    pub fn contains(&self, item: &Item<'text>) -> bool {
	self.items.contains(item)
    }
	
    pub fn count_items(&self) -> i32 {
	self.items.len() as i32
    }

    pub fn iter(&self) -> Iter<Item> {
	self.items.iter()
	
    }


}

impl<'text> From<Vec<Item<'text>>> for State<'text> {
    fn from(items: Vec<Item<'text>>) -> Self{
	Self{items}
    }
}

impl<'text> PartialEq for State<'text> {
    fn eq(&self, other: &State) -> bool {
	return true;
    }
}

#[derive(Debug)]
pub struct CFG<'text> {
    pub S:  Symbol<'text>,
    pub T:  HashSet<Symbol<'text>>,
    pub NT: HashSet<Symbol<'text>>,
    pub P:  Vec<Production<'text>>,

    loc : HashMap<Symbol<'text>, usize>,
    _FIRST: HashMap<Symbol<'text>, HashMap<Symbol<'text>, Body<'text>>>,
    _FOLLOW: HashMap<Symbol<'text>, HashSet<Symbol<'text>>>,
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

impl<'text> Production<'text> {
    pub fn new(rank: usize, s: &'text str) -> Self {
	let mut p = Production {
	    rank,
	    head: "",
	    bodies: Vec::new(),
	};

	p.parse(s);
	return p;
    }

    pub fn parse (&mut self, s: &'text str) {
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

    pub fn deconstruct(&self) -> Vec<Item<'text>> {
	self.bodies.iter().map(|x| Item {head:self.head, body:x.clone(), dot:0, lookahead:Vec::new()}).collect::<Vec<Item>>()

    }
}

impl<'text> PartialEq for Production<'text>{
    fn eq(&self, other: &Production) -> bool{
	return self.rank == other.rank && self.head == other.head;
    }

}

impl<'text> Hash for Production<'text> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rank.hash(state);
        self.head.hash(state);
    }
}

impl<'text> PartialEq for Item<'text> {
    fn eq(&self, other: &Item)->bool {
	return self.head == other.head &&
	    self.body ==  other.body &&
	    self.dot == other.dot &&
	    self.lookahead == other.lookahead;
    }
}

impl<'text> Hash for Item<'text> {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.head.hash(state);
	for x in self.body.iter() {
	    x.hash(state);
	}

	self.dot.hash(state);
    }
}


impl<'text> CFG<'text>  {
    pub fn new(s: &'text str) -> Self {
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

    pub fn normalize(&mut self, s: &'text str) {
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

	//FIRST should be calculated firstly
	self.calculate_first_set();
	self.calculate_follow_set();


    }

    pub fn is_non_terminal(&self, symbol: Symbol) -> bool {
	return self.NT.contains(symbol);
    }

    pub fn is_terminal(&self, symbol: Symbol) -> bool {
	return !self.is_non_terminal(symbol);
    }

    pub fn get_items(&self, symbol: Symbol<'text>) -> Option<Vec<Item<'text>>> {
	let items = Vec::<Item>::new();
	if let Some(idx) = self.loc.get(symbol) {
	    if let Some(production) = self.P.get(*idx as usize) {
		return Some(production.deconstruct());
	    }
	}

	return None;

    }

    



    pub fn first_set(&self, beta: &Vec<Symbol<'text>>) -> HashSet<Symbol<'text>> {

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
    
    pub fn set_first_set(&mut self, nt: Symbol<'text>, symbol_set: HashSet<Symbol<'text>>, body:  Body<'text>) {
	
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

    fn FIRST2(&self, symbol: Symbol<'text>) -> HashSet<Symbol<'text>> {
	let v = vec![symbol];
	self.FIRST(v)
    }
	
    fn FIRST(&self, beta: Vec<Symbol<'text>>) -> HashSet<Symbol<'text>> {
	let mut ret = HashSet::<Symbol>::new();
	for symbol in beta.iter() {
	    let mut symbols = HashSet::<Symbol>::new();
	    let mut can_exit = false;
	    if self.is_terminal(*symbol) {
		symbols.insert(*symbol);
	    } else {
		if let Some(dct) = self._FIRST.get(symbol) {
		    //dct is a reference to hashmap
		    symbols = dct.keys().map(|x| *x).collect::<HashSet<Symbol>>();
		    
		    if symbols.contains(&EPSILON) {
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
	    
		
	    
	
    fn print_first(&self) {
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
    pub fn calc_follow_set_relations(&self, left: Symbol<'text>, right: &Body<'text>, rels: &mut HashMap<Symbol<'text>, HashSet::<Symbol<'text>>>) {
	//deduct the relations of FOLLOW sets
	for &symbol in right.iter().rev() {
	    if self.is_terminal(symbol) {
		break;
	    } else {
		if left != symbol {
		    if let Some(mut rel) = rels.get_mut(symbol) {
			println!("Insert deps: {} dep {}", symbol, left);				
			rel.insert(left);
		    } else {
			println!("Insert deps: {} dep {}", symbol, left);
			rels.insert(symbol, HashSet::from([left]));
		    }
		}
		if !self.FIRST2(symbol).contains(&EPSILON) {
		    break;
		}
	    }	    
	}
	
    }
    
    pub fn calc_direct_suffix(&self, right: &Body<'text>, cache: &mut HashMap<Symbol<'text>, HashSet::<Symbol<'text>>>) {
	let mut check_follow = false;
	let mut last_nt = "";
	for &symbol in right.iter() {
	    let mut symbols = HashSet::<Symbol<'text>>::new();

	    if self.is_non_terminal(symbol) {
		if check_follow {
		    symbols = self.FIRST2(symbol);
		    symbols.remove(&EPSILON);
		    
		    if !last_nt.is_empty() {
			if let Some(mut ca) = cache.get_mut(&last_nt){
			    symbols.iter().for_each(|&x| {ca.insert(x);});
			} else {
			    cache.insert(last_nt, symbols);
			}
			
		    }
		}
		check_follow = true;
		last_nt = symbol;
		
	    } else { //check it's a T/NT
		if check_follow {
		    symbols.insert(symbol);
		    if !last_nt.is_empty() {
			if let Some(mut ca) = cache.get_mut(&last_nt) {
			    symbols.iter().for_each(|&x| {ca.insert(x);});
			} else {
			    cache.insert(last_nt, symbols);
			}
		    }
		}
		
		check_follow = false;
	    }
	    
	}	
    }


    pub fn calculate_follow_set(&mut self) {
	println!("NT set {:?}", self.NT);	
	let mut rels:HashMap::<Symbol<'text>, HashSet<Symbol<'text>>> = HashMap::new();
	let mut cache: HashMap<Symbol<'text>, HashSet<Symbol<'text>>> = HashMap::new();

	//Build direct suffixes and follow set relations first
	for production in self.P.iter() {
	    for right in production.bodies.iter() {
		self.calc_direct_suffix(right, &mut cache);
		self.calc_follow_set_relations(production.head, right, &mut rels);
	    }
	}
    
	println!("DEPS table are {:?}", rels);
	println!("cache table are {:?}", cache);


	match cache.get_mut(&self.S) {
	    Some(mut ca) => {ca.insert(ENDMARKER);},
	    None => {cache.insert(self.S, HashSet::from([ENDMARKER]));},
	    
	}
    
    
	// Calculate all subset of FOLLOW
	for &nt in self.NT.iter() {	
	    let mut st = HashSet::new();
	    st.insert(nt);
	    
	    let mut swap = st.clone();
	    loop {
		let tmp = swap.clone();
		swap = HashSet::new();
		for &sym in tmp.iter() {
		    if let Some(v) = rels.get(sym) {
		    for &x in v.iter() {
			swap.insert(x);
		    }
		    }
		}
		
		if swap.len() == 0 {
		    break;
		} else {
		    swap.iter().for_each(|&x| {st.insert(x);});
		}
		
	    }


	    let mut symbol_set = HashSet::new();
	    if nt == self.S {
		symbol_set.insert(ENDMARKER);
	    }
	    
	    cache.get(&nt).map(|v| v.iter().for_each(|&t| {symbol_set.insert(t);}));
	    println!("nt {}, st is {:?}", nt, st);
	    for symbol in st.iter() {
		if let Some(v) = cache.get(symbol) {
		    println!("add cache: symbol {}, v {:?}", symbol, v);
		    v.iter().for_each(|&t| {symbol_set.insert(t);});
		}
	    }
	    println!("symbol_set: {:?}", symbol_set);
	    self._FOLLOW.insert(nt, symbol_set);	    
        }
    }

    pub fn FOLLOW(&self, nt: Symbol<'text>) -> HashSet<Symbol<'text>>{
	let mut r = HashSet::new();
	if let Some(v) = self._FOLLOW.get(nt) {
	    v.iter().for_each(|&x| {r.insert(x); });
	}

	r
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

    println!("FIRST for {} \n{:?}", "E", cfg.FIRST(vec!["E"]));


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

	let set = cfg.FOLLOW("E");
	let set2 = HashSet::from([")","$"]);
	assert_eq!(set, set2);

	let set = cfg.FOLLOW("E'");
	let set2 = HashSet::from([")","$"]);
	assert_eq!(set, set2);

	let set = cfg.FOLLOW("T");
	let set2 = HashSet::from(["+", ")","$"]);
	assert_eq!(set, set2);

	let set = cfg.FOLLOW("T'");
	let set2 = HashSet::from(["+", ")","$"]);
	assert_eq!(set, set2);

	let set = cfg.FOLLOW("F");
	let set2 = HashSet::from(["*", "+", ")","$"]);
	assert_eq!(set, set2);	
	

    }
}

use std::hash::{Hash, Hasher};
use std::collections::HashSet;
mod stack;
use stack::Stack;


const EPSILON:char  = 'ε';
const ENDMARKER:char = '$';

type Symbol<'a> = &'a str;

#[derive(Debug)]
pub struct Production<'a>{
    rank: u32,
    head: Symbol<'a>, 
    bodies: Vec<Vec<Symbol<'a>>>,
}

#[derive(Debug)]
pub struct Item<'a>{
    head:  Symbol<'a>,
    body:  Vec<Symbol<'a>>,
    dot:   u32,
    lookahead: Vec<Symbol<'a>>,
}

#[derive(Debug)]
pub struct CFG<'a> {
    S:  Symbol<'a>,
    T:  HashSet<Symbol<'a>>,
    NT: HashSet<Symbol<'a>>,
    P:  Vec<Production<'a>>,
}



const productions: &str = "
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
";

fn body2item<'a> (head: Symbol<'a>, body: &Vec<Symbol<'a>>) -> Item<'a> {
    let mut item = Item {
	head,
	dot: 0,
	lookahead: Vec::new(),
	body: Vec::new(),
    };
    
    for x in body {
	item.body.push(x);
    }
    item
}

impl<'a> Production<'a> {
    pub fn new(rank: u32) -> Self {
	Production {
	    rank,
	    head: "",
	    bodies: Vec::new(),
	}
    }
    pub fn deconsturct(&self) -> Vec<Item> {
	self.bodies.iter().map(|x| body2item(self.head, x)).collect::<Vec<Item>>()
	    
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
}

impl<'a> PartialEq for Production<'a> {
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
	    self.body == other.body &&
	    self.dot == other.dot &&
	    self.lookahead == other.lookahead;
    }
}

impl<'a> Hash for Item<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.head.hash(state);
	self.body.iter().map(|x| x.hash(state));
	self.dot.hash(state);
    }
}


impl<'a> CFG <'a> {
    fn new(s: &'a str) -> Self {
	let mut cfg = Self {
	    T: HashSet::new(),
	    NT: HashSet::new(),
	    P: Vec::new(),
	    S: "",
	};

	cfg.normalize(s);
	return cfg;
    }

    fn normalize(&mut self, s: &'a str) {
	let mut idx = 0;
	let symbol_set = s.replace("->", "").replace("|", " ").split_whitespace();
	//let symbol_set = symbol_set.clone().collect::<HashSet<Symbol<'a>>>();

	for line in s.lines() {
	    let line = line.trim();
	    if line.is_empty() {
		continue;
	    }

	    let production = Production::new(idx);
	    
	    if self.S.is_empty() {
		self.S = production.head;
	    }
	    self.NT.insert(production.head);	    
	    self.P.push(production);
	    idx += 1;
	}

	//self.T = symbol_set.different(self.NT);
	for nt in &(self.NT) {
	    self.calculate_first_set(nt);
	}
	for nt in &self.NT {
	    self.calculate_follow_set(nt);
	}
    }

    fn calculate_first_set(&self, nt: Symbol) {

    }

    fn calculate_follow_set(&self, nt: Symbol) {
    }
}

fn main() {
    let mut idx = 0;
    for line in productions.lines() {
	if line.is_empty() {
	    continue;
	}
	let mut production = Production::new(idx);
	production.parse(line);
	println!("{:?}", production);
	idx += 1;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {

	assert_eq!(1,1);
    }
}

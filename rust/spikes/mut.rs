struct Foo {
    a: u32,
}

impl Foo {
    fn new()->Self {
	Foo {a: 4,}
    }

    fn fn1(&mut self) {self.a += 1;}
    fn fn2(&mut self) {
	self.fn3();
	self.a += 1;
	self.fn3();	
	self.a +=1;
	if self.a < 10 {
	    self.fn3();
	    self.fn2();
	}
	
	
    }
    fn fn3(&self) {
	if self.a > 5 {
	    println!("grater");
	} else {
	    println!("less");
	}
    }
}


fn main() {
    let mut foo = Foo::new();
    let mut idx = 1;
    while idx < 100 {
	foo.a += 2;
	foo.fn1();
	foo.fn3();
	foo.fn2();
	
	idx += 1;
    }
}

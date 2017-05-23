fn main() {

        let f = Foo;
    f.foo();

    foo(&f);
}

struct Foo;
impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}

fn foo(arg: &Foo) {
    // println!("Foo");
    
}
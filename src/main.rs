use pest::iterators::Pair;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "../trr.pest"] // relative to src
struct MyParser;

fn main() {
    let a = MyParser::parse(Rule::code, r#"
let noop ret func = :{
    b.print("hello");
    return this;
};
let z = noop ret func(3);
"#.trim()).unwrap();
    let mut pairs: Vec<Pair<_>> = a.collect();
    while let Some(p) = pairs.pop() {
        println!("{p:?}");
        
        pairs.append(&mut p.into_inner().collect::<Vec<Pair<_>>>());
    }
}

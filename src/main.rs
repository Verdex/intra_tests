
mod atom_tests;

use intra::*;

fn id<T>(t : T) -> T {
    t
}

fn blarg<'a>(x : &'a Vec<u8>) -> &'a [u8] {
    &x[..]
}

fn main() {

    let x = 0;
    atom!(x => { println!("1"); }; { println!("2"); } => { println!("3"); });

    let y = Some(8);
    atom!(y => [Some(x)] x; [a @ (8 | 9)] a; [8] => { println!("{:?}", (x, a))});

    let z = Some((1, 2));
    atom!(z => [Some((a, b))] a, b; [z @ (1|2)] => { println!("{}", z); });

    let w = Some((1, 2));
    atom!(w => id $ [Some((a, b))] a, b; id $ [z @ (1|2)] => { println!("{}", z); });

    let h = Some(vec![1, 2]);
    let mut ret = vec![];
    atom!(h => [Some(ref x)] x; blarg $ [ [a, b] ] => { println!("{}{}", a, b); ret.push(a); ret.push(b); });

    /*atom!(x => { let z = 0; }; { let h = 0; }; [ Some(8) if true ] => { let w = 0; });

    atom!(x => { let z = 0; }; { let h = 0; }; [ Some(8) if true ] a, b, c => { let w = 0; });

    atom!(x => { let z = 0; }; { let h = 0; }; [ Some(8) if true ] a => { let w = 0; });

    atom!(x => { let z = 0; }; { let h = 0; }; x $ [ Some(8) if true ] a => { let w = 0; });

    atom!(x => { let z = 0; }; { let h = 0; }; x $ [ Some(8) if true ] a, b => { let w = 0; });

    atom!(x => { let z = 0; }; { let h = 0; }; [ Some(8) if true ]; x $ [ Some(9) ] => { let w = 0; });*/
    //println!("Hello, world! {}", z);
}

// run-rustfix

fn main() {
    let mut x = vec![1usize];
    x.last_mut().unwrap() = 2usize;
    //~^ ERROR invalid left-hand side of assignment
}

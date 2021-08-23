use rand::seq::SliceRandom;
use rand::thread_rng;

crate::entry_point!("ctf/rusty", main);

fn main() {
    let mut xs = [1, 2, 3, 4];
    let mut rng = thread_rng();
    xs.shuffle(&mut rng);
    println!("{:?}", xs);
}

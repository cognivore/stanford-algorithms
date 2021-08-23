use std::collections::hash_map::RawEntryMut::Occupied;
crate::entry_point!("howto/raw_entry_api", raw_entry_api_demo);
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, Hasher},
};

pub fn mut_pop_map_no_clone<K: Hash, V>(m: &mut HashMap<K, V>) -> Option<(K, V)> {
    let key = m.keys().next()?;

    let mut hasher = m.hasher().build_hasher();
    key.hash(&mut hasher);
    let hash = hasher.finish();

    match m.raw_entry_mut().from_hash(hash, |_| true) {
        Occupied(entry) => Some(entry.remove_entry()),
        _ => unreachable!(),
    }
}

pub fn raw_entry_api_demo() {
    // Thanks, Alexendoo!
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    println!("{:?}", mut_pop_map_no_clone(&mut map));
    println!("{:?}", map);
}

#![feature(hashmap_hasher)]

use std::borrow::Borrow;
use std::hash::Hash;
use std::collections;

pub trait MapLike<'a, K: 'a, V: 'a> {
    type Iter: Iterator<Item=(&'a K, &'a V)>;

    fn get<Q: ?Sized>(&'a self, k: &Q)
        -> Option<&'a V>
        where K: Borrow<Q>,
              Q: Hash + Eq + Ord;

    fn insert(&'a mut self, k: K, v: V) -> Option<V>;
    fn iter(&'a self) -> Self::Iter;
}

impl<'a, K: 'a, V: 'a, S> MapLike<'a, K, V> for collections::HashMap<K, V, S>
where S: collections::hash_state::HashState,
      K: Hash + Eq {
    type Iter = collections::hash_map::Iter<'a, K, V>;

    fn get<Q: ?Sized>(&'a self, k: &Q)
        -> Option<&'a V>
        where K: Borrow<Q>,
              Q: Hash + Eq + Ord
        { collections::HashMap::get(self, k) }
    
    fn insert(&'a mut self, k: K, v: V) -> Option<V> { collections::HashMap::insert(self, k, v) }
    fn iter(&'a self) -> Self::Iter { collections::HashMap::iter(self) }
}

impl<'a, K: 'a, V: 'a> MapLike<'a, K, V> for collections::BTreeMap<K, V>
where K: Ord {
    type Iter = collections::btree_map::Iter<'a, K, V>;

    fn get<Q: ?Sized>(&'a self, k: &Q)
        -> Option<&'a V>
        where K: Borrow<Q>,
              Q: Hash + Eq + Ord
        { collections::BTreeMap::get(self, k) }

    fn insert(&'a mut self, k: K, v: V) -> Option<V> { collections::BTreeMap::insert(self, k, v) }
    fn iter(&'a self) -> Self::Iter { collections::BTreeMap::iter(self) }
}

pub trait MultiMap<'a, K, V> {
    fn get_only<Q: ?Sized>(&'a self, k: &Q)
        -> Option<&'a V>
        where K: Borrow<Q>,
              Q: Hash + Eq + Ord;
}

impl<'a, K: 'a, V: 'a, T: MapLike<'a, K, Vec<V>>> MultiMap<'a, K, V> for T {
    fn get_only<Q: ?Sized>(&'a self, k: &Q)
        -> Option<&'a V>
        where K: Borrow<Q>,
              Q: Hash + Eq + Ord {
        self.get(k)
            .and_then(|v| if v.len() == 1 { Some(&v[0]) } else { None })
    }
}

#[test]
fn basic_usage() {
    fn prepare<'a, T: MapLike<'a, String, Vec<usize>>>(t: &'a mut T) {
        t.insert("foobar".to_owned(), vec![3]);
    }

    fn foo<'a, T: MultiMap<'a, String, usize>>(t: &'a T) {
        assert_eq!(Some(&3), t.get_only("foobar"));
        assert!(t.get_only("baz").is_none());
    }

    let mut t = collections::HashMap::new();
    prepare(&mut t);
    t.insert("baz".to_owned(), vec![1,2,3]);
    foo(&t);
}

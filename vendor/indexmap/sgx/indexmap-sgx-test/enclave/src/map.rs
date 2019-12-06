use indexmap::*;
use indexmap::map::*;
use crate::util::enumerate;
use std::prelude::v1::*;

//#[test]
pub fn it_works() {
    let mut map = IndexMap::new();
    assert_eq!(map.is_empty(), true);
    map.insert(1, ());
    map.insert(1, ());
    assert_eq!(map.len(), 1);
    assert!(map.get(&1).is_some());
    assert_eq!(map.is_empty(), false);
}

//#[test]
pub fn new() {
    let map = IndexMap::<String, String>::new();
    println!("{:?}", map);
    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.is_empty(), true);
}

//#[test]
pub fn insert() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5];
    let not_present = [1, 3, 6, 9, 10];
    let mut map = IndexMap::with_capacity(insert.len());

    for (i, &elt) in enumerate(&insert) {
        assert_eq!(map.len(), i);
        map.insert(elt, elt);
        assert_eq!(map.len(), i + 1);
        assert_eq!(map.get(&elt), Some(&elt));
        assert_eq!(map[&elt], elt);
    }
    println!("{:?}", map);

    for &elt in &not_present {
        assert!(map.get(&elt).is_none());
    }
}

//#[test]
pub fn insert_full() {
    let insert = vec![9, 2, 7, 1, 4, 6, 13];
    let present = vec![1, 6, 2];
    let mut map = IndexMap::with_capacity(insert.len());

    for (i, &elt) in enumerate(&insert) {
        assert_eq!(map.len(), i);
        let (index, existing) = map.insert_full(elt, elt);
        assert_eq!(existing, None);
        assert_eq!(Some(index), map.get_full(&elt).map(|x| x.0));
        assert_eq!(map.len(), i + 1);
    }

    let len = map.len();
    for &elt in &present {
        let (index, existing) = map.insert_full(elt, elt);
        assert_eq!(existing, Some(elt));
        assert_eq!(Some(index), map.get_full(&elt).map(|x| x.0));
        assert_eq!(map.len(), len);
    }
}

//#[test]
pub fn insert_2() {
    let mut map = IndexMap::with_capacity(16);

    let mut keys = vec![];
    keys.extend(0..16);
    keys.extend(128..267);

    for &i in &keys {
        let old_map = map.clone();
        map.insert(i, ());
        for key in old_map.keys() {
            if !map.get(key).is_some() {
                println!("old_map: {:?}", old_map);
                println!("map: {:?}", map);
                panic!("did not find {} in map", key);
            }
        }
    }

    for &i in &keys {
        assert!(map.get(&i).is_some(), "did not find {}", i);
    }
}

//#[test]
pub fn insert_order() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
    let mut map = IndexMap::new();

    for &elt in &insert {
        map.insert(elt, ());
    }

    assert_eq!(map.keys().count(), map.len());
    assert_eq!(map.keys().count(), insert.len());
    for (a, b) in insert.iter().zip(map.keys()) {
        assert_eq!(a, b);
    }
    for (i, k) in (0..insert.len()).zip(map.keys()) {
        assert_eq!(map.get_index(i).unwrap().0, k);
    }
}

//#[test]
pub fn grow() {
    let insert = [0, 4, 2, 12, 8, 7, 11];
    let not_present = [1, 3, 6, 9, 10];
    let mut map = IndexMap::with_capacity(insert.len());


    for (i, &elt) in enumerate(&insert) {
        assert_eq!(map.len(), i);
        map.insert(elt, elt);
        assert_eq!(map.len(), i + 1);
        assert_eq!(map.get(&elt), Some(&elt));
        assert_eq!(map[&elt], elt);
    }

    println!("{:?}", map);
    for &elt in &insert {
        map.insert(elt * 10, elt);
    }
    for &elt in &insert {
        map.insert(elt * 100, elt);
    }
    for (i, &elt) in insert.iter().cycle().enumerate().take(100) {
        map.insert(elt * 100 + i as i32, elt);
    }
    println!("{:?}", map);
    for &elt in &not_present {
        assert!(map.get(&elt).is_none());
    }
}

//#[test]
pub fn remove() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
    let mut map = IndexMap::new();

    for &elt in &insert {
        map.insert(elt, elt);
    }

    assert_eq!(map.keys().count(), map.len());
    assert_eq!(map.keys().count(), insert.len());
    for (a, b) in insert.iter().zip(map.keys()) {
        assert_eq!(a, b);
    }

    let remove_fail = [99, 77];
    let remove = [4, 12, 8, 7];

    for &key in &remove_fail {
        assert!(map.swap_remove_full(&key).is_none());
    }
    println!("{:?}", map);
    for &key in &remove {
    //println!("{:?}", map);
        let index = map.get_full(&key).unwrap().0;
        assert_eq!(map.swap_remove_full(&key), Some((index, key, key)));
    }
    println!("{:?}", map);

    for key in &insert {
        assert_eq!(map.get(key).is_some(), !remove.contains(key));
    }
    assert_eq!(map.len(), insert.len() - remove.len());
    assert_eq!(map.keys().count(), insert.len() - remove.len());
}

//#[test]
pub fn remove_to_empty() {
    let mut map = indexmap! { 0 => 0, 4 => 4, 5 => 5 };
    map.swap_remove(&5).unwrap();
    map.swap_remove(&4).unwrap();
    map.swap_remove(&0).unwrap();
    assert!(map.is_empty());
}

//#[test]
pub fn swap_remove_index() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
    let mut map = IndexMap::new();

    for &elt in &insert {
        map.insert(elt, elt * 2);
    }

    let mut vector = insert.to_vec();
    let remove_sequence = &[3, 3, 10, 4, 5, 4, 3, 0, 1];

    // check that the same swap remove sequence on vec and map
    // have the same result.
    for &rm in remove_sequence {
        let out_vec = vector.swap_remove(rm);
        let (out_map, _) = map.swap_remove_index(rm).unwrap();
        assert_eq!(out_vec, out_map);
    }
    assert_eq!(vector.len(), map.len());
    for (a, b) in vector.iter().zip(map.keys()) {
        assert_eq!(a, b);
    }
}

//#[test]
pub fn partial_eq_and_eq() {
    let mut map_a = IndexMap::new();
    map_a.insert(1, "1");
    map_a.insert(2, "2");
    let mut map_b = map_a.clone();
    assert_eq!(map_a, map_b);
    map_b.remove(&1);
    assert_ne!(map_a, map_b);

    let map_c: IndexMap<_, String> = map_b.into_iter().map(|(k, v)| (k, v.to_owned())).collect();
    assert_ne!(map_a, map_c);
    assert_ne!(map_c, map_a);
}

//#[test]
pub fn extend() {
    let mut map = IndexMap::new();
    map.extend(vec![(&1, &2), (&3, &4)]);
    map.extend(vec![(5, 6)]);
    assert_eq!(map.into_iter().collect::<Vec<_>>(), vec![(1, 2), (3, 4), (5, 6)]);
}

//#[test]
pub fn entry() {
    let mut map = IndexMap::new();
    
    map.insert(1, "1");
    map.insert(2, "2");
    {
        let e = map.entry(3);
        assert_eq!(e.index(), 2);
        let e = e.or_insert("3");
        assert_eq!(e, &"3");
    }
    
    let e = map.entry(2);
    assert_eq!(e.index(), 1);
    assert_eq!(e.key(), &2);
    match e {
        Entry::Occupied(ref e) => assert_eq!(e.get(), &"2"),
        Entry::Vacant(_) => panic!()
    }
    assert_eq!(e.or_insert("4"), &"2");
}

//#[test]
pub fn entry_and_modify() {
    let mut map = IndexMap::new();

    map.insert(1, "1");
    map.entry(1).and_modify(|x| *x = "2");
    assert_eq!(Some(&"2"), map.get(&1));

    map.entry(2).and_modify(|x| *x = "doesn't exist");
    assert_eq!(None, map.get(&2));
}

//#[test]
pub fn entry_or_default() {
    let mut map = IndexMap::new();

    #[derive(Debug, PartialEq)]
    enum TestEnum {
        DefaultValue,
        NonDefaultValue,
    }

    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::DefaultValue
        }
    }

    map.insert(1, TestEnum::NonDefaultValue);
    assert_eq!(&mut TestEnum::NonDefaultValue, map.entry(1).or_default());

    assert_eq!(&mut TestEnum::DefaultValue, map.entry(2).or_default());
}

//#[test]
pub fn keys() {
    let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let map: IndexMap<_, _> = vec.into_iter().collect();
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&3));
}

//#[test]
pub fn values() {
    let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let map: IndexMap<_, _> = vec.into_iter().collect();
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&'a'));
    assert!(values.contains(&'b'));
    assert!(values.contains(&'c'));
}

//#[test]
pub fn values_mut() {
    let vec = vec![(1, 1), (2, 2), (3, 3)];
    let mut map: IndexMap<_, _> = vec.into_iter().collect();
    for value in map.values_mut() {
        *value = (*value) * 2
    }
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&2));
    assert!(values.contains(&4));
    assert!(values.contains(&6));
}

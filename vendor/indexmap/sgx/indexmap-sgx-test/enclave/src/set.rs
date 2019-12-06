//use super::*;
use std::prelude::v1::*;
use indexmap::*;
use crate::util::enumerate;

//#[test]
pub fn it_works() {
    let mut set = IndexSet::new();
    assert_eq!(set.is_empty(), true);
    set.insert(1);
    set.insert(1);
    assert_eq!(set.len(), 1);
    assert!(set.get(&1).is_some());
    assert_eq!(set.is_empty(), false);
}

//#[test]
pub fn new() {
    let set = IndexSet::<String>::new();
    println!("{:?}", set);
    assert_eq!(set.capacity(), 0);
    assert_eq!(set.len(), 0);
    assert_eq!(set.is_empty(), true);
}

//#[test]
pub fn insert() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5];
    let not_present = [1, 3, 6, 9, 10];
    let mut set = IndexSet::with_capacity(insert.len());

    for (i, &elt) in enumerate(&insert) {
        assert_eq!(set.len(), i);
        set.insert(elt);
        assert_eq!(set.len(), i + 1);
        assert_eq!(set.get(&elt), Some(&elt));
    }
    println!("{:?}", set);

    for &elt in &not_present {
        assert!(set.get(&elt).is_none());
    }
}

//#[test]
pub fn insert_full() {
    let insert = vec![9, 2, 7, 1, 4, 6, 13];
    let present = vec![1, 6, 2];
    let mut set = IndexSet::with_capacity(insert.len());

    for (i, &elt) in enumerate(&insert) {
        assert_eq!(set.len(), i);
        let (index, success) = set.insert_full(elt);
        assert!(success);
        assert_eq!(Some(index), set.get_full(&elt).map(|x| x.0));
        assert_eq!(set.len(), i + 1);
    }

    let len = set.len();
    for &elt in &present {
        let (index, success) = set.insert_full(elt);
        assert!(!success);
        assert_eq!(Some(index), set.get_full(&elt).map(|x| x.0));
        assert_eq!(set.len(), len);
    }
}

//#[test]
pub fn insert_2() {
    let mut set = IndexSet::with_capacity(16);

    let mut values = vec![];
    values.extend(0..16);
    values.extend(128..267);

    for &i in &values {
        let old_set = set.clone();
        set.insert(i);
        for value in old_set.iter() {
            if !set.get(value).is_some() {
                println!("old_set: {:?}", old_set);
                println!("set: {:?}", set);
                panic!("did not find {} in set", value);
            }
        }
    }

    for &i in &values {
        assert!(set.get(&i).is_some(), "did not find {}", i);
    }
}

//#[test]
pub fn insert_dup() {
    let mut elements = vec![0, 2, 4, 6, 8];
    let mut set: IndexSet<u8> = elements.drain(..).collect();
    {
        let (i, v) = set.get_full(&0).unwrap();
        assert_eq!(set.len(), 5);
        assert_eq!(i, 0);
        assert_eq!(*v, 0);
    }
    {
        let inserted = set.insert(0);
        let (i, v) = set.get_full(&0).unwrap();
        assert_eq!(set.len(), 5);
        assert_eq!(inserted, false);
        assert_eq!(i, 0);
        assert_eq!(*v, 0);
    }
}

//#[test]
pub fn insert_order() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
    let mut set = IndexSet::new();

    for &elt in &insert {
        set.insert(elt);
    }

    assert_eq!(set.iter().count(), set.len());
    assert_eq!(set.iter().count(), insert.len());
    for (a, b) in insert.iter().zip(set.iter()) {
        assert_eq!(a, b);
    }
    for (i, v) in (0..insert.len()).zip(set.iter()) {
        assert_eq!(set.get_index(i).unwrap(), v);
    }
}

//#[test]
pub fn grow() {
    let insert = [0, 4, 2, 12, 8, 7, 11];
    let not_present = [1, 3, 6, 9, 10];
    let mut set = IndexSet::with_capacity(insert.len());


    for (i, &elt) in enumerate(&insert) {
        assert_eq!(set.len(), i);
        set.insert(elt);
        assert_eq!(set.len(), i + 1);
        assert_eq!(set.get(&elt), Some(&elt));
    }

    println!("{:?}", set);
    for &elt in &insert {
        set.insert(elt * 10);
    }
    for &elt in &insert {
        set.insert(elt * 100);
    }
    for (i, &elt) in insert.iter().cycle().enumerate().take(100) {
        set.insert(elt * 100 + i as i32);
    }
    println!("{:?}", set);
    for &elt in &not_present {
        assert!(set.get(&elt).is_none());
    }
}

//#[test]
pub fn remove() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
    let mut set = IndexSet::new();

    for &elt in &insert {
        set.insert(elt);
    }

    assert_eq!(set.iter().count(), set.len());
    assert_eq!(set.iter().count(), insert.len());
    for (a, b) in insert.iter().zip(set.iter()) {
        assert_eq!(a, b);
    }

    let remove_fail = [99, 77];
    let remove = [4, 12, 8, 7];

    for &value in &remove_fail {
        assert!(set.swap_remove_full(&value).is_none());
    }
    println!("{:?}", set);
    for &value in &remove {
    //println!("{:?}", set);
        let index = set.get_full(&value).unwrap().0;
        assert_eq!(set.swap_remove_full(&value), Some((index, value)));
    }
    println!("{:?}", set);

    for value in &insert {
        assert_eq!(set.get(value).is_some(), !remove.contains(value));
    }
    assert_eq!(set.len(), insert.len() - remove.len());
    assert_eq!(set.iter().count(), insert.len() - remove.len());
}

//#[test]
pub fn swap_remove_index() {
    let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
    let mut set = IndexSet::new();

    for &elt in &insert {
        set.insert(elt);
    }

    let mut vector = insert.to_vec();
    let remove_sequence = &[3, 3, 10, 4, 5, 4, 3, 0, 1];

    // check that the same swap remove sequence on vec and set
    // have the same result.
    for &rm in remove_sequence {
        let out_vec = vector.swap_remove(rm);
        let out_set = set.swap_remove_index(rm).unwrap();
        assert_eq!(out_vec, out_set);
    }
    assert_eq!(vector.len(), set.len());
    for (a, b) in vector.iter().zip(set.iter()) {
        assert_eq!(a, b);
    }
}

//#[test]
pub fn partial_eq_and_eq() {
    let mut set_a = IndexSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b = set_a.clone();
    assert_eq!(set_a, set_b);
    set_b.remove(&1);
    assert_ne!(set_a, set_b);

    let set_c: IndexSet<_> = set_b.into_iter().collect();
    assert_ne!(set_a, set_c);
    assert_ne!(set_c, set_a);
}

//#[test]
pub fn extend() {
    let mut set = IndexSet::new();
    set.extend(vec![&1, &2, &3, &4]);
    set.extend(vec![5, 6]);
    assert_eq!(set.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6]);
}

//#[test]
pub fn comparisons() {
    let set_a: IndexSet<_> = (0..3).collect();
    let set_b: IndexSet<_> = (3..6).collect();
    let set_c: IndexSet<_> = (0..6).collect();
    let set_d: IndexSet<_> = (3..9).collect();

    assert!(!set_a.is_disjoint(&set_a));
    assert!(set_a.is_subset(&set_a));
    assert!(set_a.is_superset(&set_a));

    assert!(set_a.is_disjoint(&set_b));
    assert!(set_b.is_disjoint(&set_a));
    assert!(!set_a.is_subset(&set_b));
    assert!(!set_b.is_subset(&set_a));
    assert!(!set_a.is_superset(&set_b));
    assert!(!set_b.is_superset(&set_a));

    assert!(!set_a.is_disjoint(&set_c));
    assert!(!set_c.is_disjoint(&set_a));
    assert!(set_a.is_subset(&set_c));
    assert!(!set_c.is_subset(&set_a));
    assert!(!set_a.is_superset(&set_c));
    assert!(set_c.is_superset(&set_a));

    assert!(!set_c.is_disjoint(&set_d));
    assert!(!set_d.is_disjoint(&set_c));
    assert!(!set_c.is_subset(&set_d));
    assert!(!set_d.is_subset(&set_c));
    assert!(!set_c.is_superset(&set_d));
    assert!(!set_d.is_superset(&set_c));
}

//#[test]
pub fn iter_comparisons() {
    use std::iter::empty;

    fn check<'a, I1, I2>(iter1: I1, iter2: I2)
        where I1: Iterator<Item = &'a i32>,
              I2: Iterator<Item = i32>,
    {
        assert!(iter1.cloned().eq(iter2));
    }

    let set_a: IndexSet<_> = (0..3).collect();
    let set_b: IndexSet<_> = (3..6).collect();
    let set_c: IndexSet<_> = (0..6).collect();
    let set_d: IndexSet<_> = (3..9).rev().collect();

    check(set_a.difference(&set_a), empty());
    check(set_a.symmetric_difference(&set_a), empty());
    check(set_a.intersection(&set_a), 0..3);
    check(set_a.union(&set_a), 0..3);

    check(set_a.difference(&set_b), 0..3);
    check(set_b.difference(&set_a), 3..6);
    check(set_a.symmetric_difference(&set_b), 0..6);
    check(set_b.symmetric_difference(&set_a), (3..6).chain(0..3));
    check(set_a.intersection(&set_b), empty());
    check(set_b.intersection(&set_a), empty());
    check(set_a.union(&set_b), 0..6);
    check(set_b.union(&set_a), (3..6).chain(0..3));

    check(set_a.difference(&set_c), empty());
    check(set_c.difference(&set_a), 3..6);
    check(set_a.symmetric_difference(&set_c), 3..6);
    check(set_c.symmetric_difference(&set_a), 3..6);
    check(set_a.intersection(&set_c), 0..3);
    check(set_c.intersection(&set_a), 0..3);
    check(set_a.union(&set_c), 0..6);
    check(set_c.union(&set_a), 0..6);

    check(set_c.difference(&set_d), 0..3);
    check(set_d.difference(&set_c), (6..9).rev());
    check(set_c.symmetric_difference(&set_d), (0..3).chain((6..9).rev()));
    check(set_d.symmetric_difference(&set_c), (6..9).rev().chain(0..3));
    check(set_c.intersection(&set_d), 3..6);
    check(set_d.intersection(&set_c), (3..6).rev());
    check(set_c.union(&set_d), (0..6).chain((6..9).rev()));
    check(set_d.union(&set_c), (3..9).rev().chain(0..3));
}

//#[test]
pub fn ops() {
    let empty = IndexSet::<i32>::new();
    let set_a: IndexSet<_> = (0..3).collect();
    let set_b: IndexSet<_> = (3..6).collect();
    let set_c: IndexSet<_> = (0..6).collect();
    let set_d: IndexSet<_> = (3..9).rev().collect();

    assert_eq!(&set_a & &set_a, set_a);
    assert_eq!(&set_a | &set_a, set_a);
    assert_eq!(&set_a ^ &set_a, empty);
    assert_eq!(&set_a - &set_a, empty);

    assert_eq!(&set_a & &set_b, empty);
    assert_eq!(&set_b & &set_a, empty);
    assert_eq!(&set_a | &set_b, set_c);
    assert_eq!(&set_b | &set_a, set_c);
    assert_eq!(&set_a ^ &set_b, set_c);
    assert_eq!(&set_b ^ &set_a, set_c);
    assert_eq!(&set_a - &set_b, set_a);
    assert_eq!(&set_b - &set_a, set_b);

    assert_eq!(&set_a & &set_c, set_a);
    assert_eq!(&set_c & &set_a, set_a);
    assert_eq!(&set_a | &set_c, set_c);
    assert_eq!(&set_c | &set_a, set_c);
    assert_eq!(&set_a ^ &set_c, set_b);
    assert_eq!(&set_c ^ &set_a, set_b);
    assert_eq!(&set_a - &set_c, empty);
    assert_eq!(&set_c - &set_a, set_b);

    assert_eq!(&set_c & &set_d, set_b);
    assert_eq!(&set_d & &set_c, set_b);
    assert_eq!(&set_c | &set_d, &set_a | &set_d);
    assert_eq!(&set_d | &set_c, &set_a | &set_d);
    assert_eq!(&set_c ^ &set_d, &set_a | &(&set_d - &set_b));
    assert_eq!(&set_d ^ &set_c, &set_a | &(&set_d - &set_b));
    assert_eq!(&set_c - &set_d, set_a);
    assert_eq!(&set_d - &set_c, &set_d - &set_b);
}

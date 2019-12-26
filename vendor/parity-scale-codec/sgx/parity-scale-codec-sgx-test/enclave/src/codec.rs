const MAX_PREALLOCATION: usize = 4 * 1024;

use parity_scale_codec::{
    Encode,
    Decode,
    Compact,
    EncodeLike,
    DecodeLength,
    OptionBool,
    WrapperTypeEncode,
    WrapperTypeDecode,
    IoReader,
    Error,
    Input
};

use core::{ops::Deref, iter::FromIterator};

use std::{
    string::String,
    borrow::{Cow, ToOwned},
    vec::Vec,
    collections::{BTreeMap, BTreeSet, VecDeque, LinkedList, BinaryHeap},
};

pub fn vec_is_slicable() {
    let v = b"Hello world".to_vec();
    v.using_encoded(|ref slice|
        assert_eq!(slice, &b"\x2cHello world")
    );
}

pub fn encode_borrowed_tuple() {
    let x = vec![1u8, 2, 3, 4];
    let y = 128i64;

    let encoded = (&x, &y).encode();

    assert_eq!((x, y), Decode::decode(&mut &encoded[..]).unwrap());
}

pub fn cow_works() {
    let x = &[1u32, 2, 3, 4, 5, 6][..];
    let y = Cow::Borrowed(&x);
    assert_eq!(x.encode(), y.encode());

    let z: Cow<'_, [u32]> = Cow::decode(&mut &x.encode()[..]).unwrap();
    assert_eq!(*z, *x);
}

pub fn cow_string_works() {
    let x = "Hello world!";
    let y = Cow::Borrowed(&x);
    assert_eq!(x.encode(), y.encode());

    let z: Cow<'_, str> = Cow::decode(&mut &x.encode()[..]).unwrap();
    assert_eq!(*z, *x);
}

fn hexify(bytes: &[u8]) -> String {
    bytes.iter().map(|ref b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ")
}

pub fn string_encoded_as_expected() {
    let value = String::from("Hello, World!");
    let encoded = value.encode();
    assert_eq!(hexify(&encoded), "34 48 65 6c 6c 6f 2c 20 57 6f 72 6c 64 21");
    assert_eq!(<String>::decode(&mut &encoded[..]).unwrap(), value);
}

pub fn vec_of_u8_encoded_as_expected() {
    let value = vec![0u8, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    let encoded = value.encode();
    assert_eq!(hexify(&encoded), "28 00 01 01 02 03 05 08 0d 15 22");
    assert_eq!(<Vec<u8>>::decode(&mut &encoded[..]).unwrap(), value);
}

pub fn vec_of_i16_encoded_as_expected() {
    let value = vec![0i16, 1, -1, 2, -2, 3, -3];
    let encoded = value.encode();
    assert_eq!(hexify(&encoded), "1c 00 00 01 00 ff ff 02 00 fe ff 03 00 fd ff");
    assert_eq!(<Vec<i16>>::decode(&mut &encoded[..]).unwrap(), value);
}

pub fn vec_of_option_int_encoded_as_expected() {
    let value = vec![Some(1i8), Some(-1), None];
    let encoded = value.encode();
    assert_eq!(hexify(&encoded), "0c 01 01 01 ff 00");
    assert_eq!(<Vec<Option<i8>>>::decode(&mut &encoded[..]).unwrap(), value);
}

pub fn vec_of_option_bool_encoded_as_expected() {
    let value = vec![OptionBool(Some(true)), OptionBool(Some(false)), OptionBool(None)];
    let encoded = value.encode();
    assert_eq!(hexify(&encoded), "0c 01 02 00");
    assert_eq!(<Vec<OptionBool>>::decode(&mut &encoded[..]).unwrap(), value);
}

fn test_encode_length<T: Encode + Decode + DecodeLength>(thing: &T, len: usize) {
    assert_eq!(<T as DecodeLength>::len(&mut &thing.encode()[..]).unwrap(), len);
}

pub fn len_works_for_decode_collection_types() {
    let vector = vec![10; 10];
    let mut btree_map: BTreeMap<u32, u32> = BTreeMap::new();
    btree_map.insert(1, 1);
    btree_map.insert(2, 2);
    let mut btree_set: BTreeSet<u32> = BTreeSet::new();
    btree_set.insert(1);
    btree_set.insert(2);
    let mut vd = VecDeque::new();
    vd.push_front(1);
    vd.push_front(2);
    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(2);
    let mut ll = LinkedList::new();
    ll.push_back(1);
    ll.push_back(2);

    test_encode_length(&vector, 10);
    test_encode_length(&btree_map, 2);
    test_encode_length(&btree_set, 2);
    test_encode_length(&vd, 2);
    test_encode_length(&bh, 2);
    test_encode_length(&ll, 2);
}

pub fn vec_of_string_encoded_as_expected() {
    let value = vec![
        "Hamlet".to_owned(),
        "Война и мир".to_owned(),
        "三国演义".to_owned(),
        "أَلْف لَيْلَة وَلَيْلَة‎".to_owned()
    ];
    let encoded = value.encode();
    assert_eq!(hexify(&encoded), "10 18 48 61 6d 6c 65 74 50 d0 92 d0 be d0 b9 d0 bd d0 b0 20 d0 \
			b8 20 d0 bc d0 b8 d1 80 30 e4 b8 89 e5 9b bd e6 bc 94 e4 b9 89 bc d8 a3 d9 8e d9 84 d9 92 \
			d9 81 20 d9 84 d9 8e d9 8a d9 92 d9 84 d9 8e d8 a9 20 d9 88 d9 8e d9 84 d9 8e d9 8a d9 92 \
			d9 84 d9 8e d8 a9 e2 80 8e");
    assert_eq!(<Vec<String>>::decode(&mut &encoded[..]).unwrap(), value);
}

#[derive(Debug, PartialEq)]
struct MyWrapper(Compact<u32>);
impl Deref for MyWrapper {
    type Target = Compact<u32>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl WrapperTypeEncode for MyWrapper {}

impl From<Compact<u32>> for MyWrapper {
    fn from(c: Compact<u32>) -> Self { MyWrapper(c) }
}
impl WrapperTypeDecode for MyWrapper {
    type Wrapped = Compact<u32>;
}

pub fn should_work_for_wrapper_types() {
    let result = vec![0b1100];

    assert_eq!(MyWrapper(3u32.into()).encode(), result);
    assert_eq!(MyWrapper::decode(&mut &*result).unwrap(), MyWrapper(3_u32.into()));
}

pub fn codec_vec_deque_u8_and_u16() {
    let mut v_u8 = VecDeque::new();
    let mut v_u16 = VecDeque::new();

    for i in 0..50 {
        v_u8.push_front(i as u8);
        v_u16.push_front(i as u16);
    }
    for i in 50..100 {
        v_u8.push_back(i as u8);
        v_u16.push_back(i as u16);
    }

    assert_eq!(Decode::decode(&mut &v_u8.encode()[..]), Ok(v_u8));
    assert_eq!(Decode::decode(&mut &v_u16.encode()[..]), Ok(v_u16));
}

pub fn codec_iterator() {
    let t1: BTreeSet<u32> = FromIterator::from_iter((0..10).flat_map(|i| 0..i));
    let t2: LinkedList<u32> = FromIterator::from_iter((0..10).flat_map(|i| 0..i));
    let t3: BinaryHeap<u32> = FromIterator::from_iter((0..10).flat_map(|i| 0..i));
    let t4: BTreeMap<u16, u32> = FromIterator::from_iter(
        (0..10)
            .flat_map(|i| 0..i)
            .map(|i| (i as u16, i + 10))
    );
    let t5: BTreeSet<Vec<u8>> = FromIterator::from_iter((0..10).map(|i| Vec::from_iter(0..i)));
    let t6: LinkedList<Vec<u8>> = FromIterator::from_iter((0..10).map(|i| Vec::from_iter(0..i)));
    let t7: BinaryHeap<Vec<u8>> = FromIterator::from_iter((0..10).map(|i| Vec::from_iter(0..i)));
    let t8: BTreeMap<Vec<u8>, u32> = FromIterator::from_iter(
        (0..10)
            .map(|i| Vec::from_iter(0..i))
            .map(|i| (i.clone(), i.len() as u32))
    );

    assert_eq!(Decode::decode(&mut &t1.encode()[..]), Ok(t1));
    assert_eq!(Decode::decode(&mut &t2.encode()[..]), Ok(t2));
    assert_eq!(
        Decode::decode(&mut &t3.encode()[..]).map(BinaryHeap::into_sorted_vec),
        Ok(t3.into_sorted_vec()),
    );
    assert_eq!(Decode::decode(&mut &t4.encode()[..]), Ok(t4));
    assert_eq!(Decode::decode(&mut &t5.encode()[..]), Ok(t5));
    assert_eq!(Decode::decode(&mut &t6.encode()[..]), Ok(t6));
    assert_eq!(
        Decode::decode(&mut &t7.encode()[..]).map(BinaryHeap::into_sorted_vec),
        Ok(t7.into_sorted_vec()),
    );
    assert_eq!(Decode::decode(&mut &t8.encode()[..]), Ok(t8));
}

pub fn io_reader() {
    use std::io::{Seek, SeekFrom};

    let mut io_reader = IoReader(std::io::Cursor::new(&[1u8, 2, 3][..]));

    assert_eq!(io_reader.0.seek(SeekFrom::Current(0)).unwrap(), 0);
    assert_eq!(io_reader.remaining_len().unwrap().unwrap(), 3);

    assert_eq!(io_reader.read_byte().unwrap(), 1);
    assert_eq!(io_reader.0.seek(SeekFrom::Current(0)).unwrap(), 1);
    assert_eq!(io_reader.remaining_len().unwrap().unwrap(), 2);

    assert_eq!(io_reader.read_byte().unwrap(), 2);
    assert_eq!(io_reader.read_byte().unwrap(), 3);
    assert_eq!(io_reader.0.seek(SeekFrom::Current(0)).unwrap(), 3);
    assert_eq!(io_reader.remaining_len().unwrap().unwrap(), 0);
}

pub fn shared_references_implement_encode() {
    std::sync::Arc::new(10u32).encode();
//    std::rc::Rc::new(10u32).encode();
}

pub fn not_limit_input_test() {
    struct NoLimit<'a>(&'a [u8]);

    impl<'a> Input for NoLimit<'a> {
        fn remaining_len(&mut self) -> Result<Option<usize>, Error> {
            Ok(None)
        }

        fn read(&mut self, into: &mut [u8]) -> Result<(), Error> {
            self.0.read(into)
        }
    }

    let len = MAX_PREALLOCATION * 2 + 1;
    let mut i = Compact(len as u32).encode();
    i.resize(i.len() + len, 0);
    assert_eq!(<Vec<u8>>::decode(&mut NoLimit(&i[..])).unwrap(), vec![0u8; len]);

    let i = Compact(len as u32).encode();
    assert_eq!(<Vec<u8>>::decode(&mut NoLimit(&i[..])).err().unwrap().what(), "Not enough data to fill buffer");

    let i = Compact(1000u32).encode();
    assert_eq!(<Vec<u8>>::decode(&mut NoLimit(&i[..])).err().unwrap().what(), "Not enough data to fill buffer");
}

pub fn boolean() {
    assert_eq!(true.encode(), vec![1]);
    assert_eq!(false.encode(), vec![0]);
    assert_eq!(bool::decode(&mut &[1][..]).unwrap(), true);
    assert_eq!(bool::decode(&mut &[0][..]).unwrap(), false);
}

pub fn some_encode_like() {
    fn t<B: EncodeLike>() {}
    t::<&[u8]>();
    t::<&str>();
}

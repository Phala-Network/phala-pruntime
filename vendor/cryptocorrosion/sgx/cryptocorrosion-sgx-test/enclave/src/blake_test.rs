use digest::dev::digest_test;

new_test!(blake224, "blake224", blake_hash::Blake224, digest_test);
new_test!(blake256, "blake256", blake_hash::Blake256, digest_test);
new_test!(blake384, "blake384", blake_hash::Blake384, digest_test);
new_test!(blake512, "blake512", blake_hash::Blake512, digest_test);

//pub fn blake384() {
//    use digest::blobby::Blob2Iterator;
//    use blake_hash::Digest;
//    let data = include_bytes!(concat!("data/", "blake384", ".blb"));
//
//    for (i, row) in Blob2Iterator::new(data).unwrap().enumerate() {
//        let input = row[0];
//        let output = row[1];
//        println!("row[0] = {:?}", input);
//        println!("row[1] = {:?}", output);
//        let mut hasher = blake_hash::Blake384::new();
//        hasher.input(input);
//        println!("hasher.result = {:?}", hasher.result().as_slice());
//        if let Some(desc) = digest_test::<blake_hash::Blake384>(input, output) {
//            panic!("\n\
//                Failed test №{}: {}\n\
//                input:\t{:?}\n\
//                output:\t{:?}\n",
//                i, desc, input, output,
//            );
//        }
//    }
//}

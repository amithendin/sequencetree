extern crate rand;

mod binarysearchtree;
mod sequencetree;
use sequencetree::SequenceTree;

#[cfg(test)]
mod tests {
    use rand::Rng;

    fn pretty_print_int(i: u64) -> String {
        let mut s = String::new();
        let i_str = i.to_string();
        let a = i_str.chars().rev().enumerate();
        for (idx, val) in a {
            if idx != 0 && idx % 3 == 0 {
                s.insert(0, ',');
            }
            s.insert(0, val);
        }

        s
    }

    #[test]
    fn accuracy() {
        use rand::{Rng, prelude::{SeedableRng, StdRng} };

        const TEST_SCALE: u64 = 1_000;

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~_-.,[]{}:+|";
        const KEY_MAX_LEN: usize = 30;

        println!("########TEST PARAMS: \n\tkeys: {} \n\tkey_len_range: {} to {} \n\tcharset_len: {}", pretty_print_int(TEST_SCALE), 3, KEY_MAX_LEN, CHARSET.len());

        fn gen_str(rng: &mut StdRng) -> String {
            (0..rng.gen_range(3, KEY_MAX_LEN) )
                .map(|_| {
                    let idx = rng.gen_range(0, CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        }

        let keys_seed = [0u8; 32];
        let mut keys_rng: StdRng = SeedableRng::from_seed(keys_seed);

        let values_seed = [1u8; 32];
        let mut values_rng: StdRng = SeedableRng::from_seed(values_seed);

        use crate::sequencetree::SequenceTree;
        let mut at = SequenceTree::new();

        for _ in 0u64 .. TEST_SCALE {
            at.set(gen_str(&mut keys_rng), gen_str(&mut values_rng));
        }

        keys_rng = SeedableRng::from_seed(keys_seed);
        values_rng = SeedableRng::from_seed(values_seed);
        for _ in 0u64 .. TEST_SCALE {
            match at.get (gen_str(&mut keys_rng)) {
                Some(val) => assert_eq!(val, gen_str(&mut values_rng)),
                None => panic!("key not found")
            };
        }
    }

    #[test]
    fn read_write_speed() {
        use std::time::{Duration, Instant};
        use rand::{Rng, prelude::{SeedableRng, StdRng} };

        const TEST_SCALE: u64 = 10_000;

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~_-.,[]{}:+|";
        const KEY_MAX_LEN: usize = 30;

        fn gen_str(rng: &mut StdRng) -> String {
            (0..rng.gen_range(3, KEY_MAX_LEN) )
                .map(|_| {
                    let idx = rng.gen_range(0, CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        }

        println!("########TEST PARAMS: \n\tkeys: {} \n\tkey_len_range: {} to {} \n\tcharset_len: {}", pretty_print_int(TEST_SCALE), 3, KEY_MAX_LEN, CHARSET.len());

        use std::collections::HashMap;
        use crate::sequencetree::SequenceTree;

        println!("\n--------HASH MAP--------");
        let mut hm: HashMap<String, String> = HashMap::new();

        let seed = [0u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut start: Instant = Instant::now();
        for i in 0u64 .. TEST_SCALE {
            hm.insert(gen_str(&mut rng), format!("value{}",i));
        }
        println!("store time {:?} ms", start.elapsed().as_millis());

        rng = SeedableRng::from_seed(seed);
        start = Instant::now();
        for _ in 0u64 .. TEST_SCALE {
            hm.get(gen_str(&mut rng).as_str());
        }
        println!("fetch time {:?} ms", start.elapsed().as_millis());

        println!("\n--------SEQUENCE TREE--------");
        let mut at = SequenceTree::new();

        rng = SeedableRng::from_seed(seed);
        start = Instant::now();
        for i in 0u64 .. TEST_SCALE {
            at.set(gen_str(&mut rng), format!("value{}",i));
        }
        println!("store time {:?} ms", start.elapsed().as_millis());

        rng = SeedableRng::from_seed(seed);
        start = Instant::now();
        for i in 0u64 .. TEST_SCALE {
            at.get(gen_str(&mut rng));
        }
        println!("fetch time {:?} ms", start.elapsed().as_millis());

        rng = SeedableRng::from_seed(seed);

        let lim = 30;
        let mut key = String::new();
        for _ in 0 .. lim {
            key = gen_str(&mut rng);
        }
    }
}

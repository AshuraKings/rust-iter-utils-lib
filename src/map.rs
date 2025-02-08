use std::{collections::HashMap, hash::Hash, ops::Deref};

trait ReMap<K1: Eq + Hash, V1> {
    fn map<K: Eq + Hash, V, A: Fn(&K1) -> K, B: Fn(&V1) -> V>(
        self,
        kfn: A,
        vfn: B,
    ) -> HashMap<K, V>;
}

impl<K1: Eq + Hash, V1> ReMap<K1, V1> for HashMap<K1, V1> {
    fn map<K: Eq + Hash, V, A: Fn(&K1) -> K, B: Fn(&V1) -> V>(
        self,
        kfn: A,
        vfn: B,
    ) -> HashMap<K, V> {
        let mut dict = HashMap::new();
        for (k, v) in self.iter() {
            let k2 = kfn(k);
            let v2 = vfn(v);
            dict.insert(k2, v2);
        }
        dict
    }
}

trait ReMap1<K1: Eq + Hash + Deref + Copy, V1> {
    fn map1<K, A: Fn(&V1) -> K>(self, kfn: A) -> HashMap<K1, K>;
}

impl<K1: Eq + Hash + Deref + Copy, V1> ReMap1<K1, V1> for HashMap<K1, V1> {
    fn map1<K, A: Fn(&V1) -> K>(self, kfn: A) -> HashMap<K1, K> {
        let mut dict = HashMap::new();
        for (k, v) in self.iter() {
            let v2 = kfn(v);
            dict.insert(*k, v2);
        }
        dict
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;
    use crate::ranges::*;

    #[derive(Debug, Clone, Copy)]
    struct Grouped {
        cat: &'static str,
        val: i32,
    }

    impl std::fmt::Display for Grouped {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Grouped[cat={}, val={}]", self.cat, self.val)
        }
    }

    #[test]
    fn map1() {
        let result = (0..10)
            .map(|i| Grouped {
                cat: if i % 2 == 0 { "Genap" } else { "Ganjil" },
                val: i,
            })
            .group_by(|g| g.cat)
            .map(|i| i.to_string(), |v| format!("{}", v.first().unwrap()));
        println!("Result : {:?}", result);
    }

    #[test]
    fn map2() {
        let result = (0..10)
            .map(|i| Grouped {
                cat: if i % 2 == 0 { "Genap" } else { "Ganjil" },
                val: i,
            })
            .group_by(|g| g.cat)
            .map1(|v| *v.first().unwrap());
        println!("Result : {:?}", result);
    }
}

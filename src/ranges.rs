use std::{collections::HashMap, hash::Hash, ops::Deref};

pub trait ToVec: Iterator {
    fn to_vec(self) -> Vec<Self::Item>;
}

impl<I> ToVec for I
where
    I: Iterator,
{
    fn to_vec(self) -> Vec<Self::Item> {
        let mut result = vec![];
        for i in self {
            result.push(i);
        }
        result
    }
}

pub trait Distinct: Iterator {
    fn distinct(self) -> Vec<Self::Item>;
}

impl<I: Iterator> Distinct for I
where
    I::Item: PartialEq,
{
    fn distinct(self) -> Vec<Self::Item> {
        let mut tmp: Vec<Self::Item> = vec![];
        for i in self {
            if !tmp.contains(&i) {
                tmp.push(i);
            }
        }
        tmp
    }
}

pub trait GroupBy: Iterator {
    fn group_by<T: Hash + Eq, U: Fn(&Self::Item) -> T>(
        self,
        get_key: U,
    ) -> HashMap<T, Vec<Self::Item>>;
}

impl<I: Iterator> GroupBy for I {
    fn group_by<T: Hash + Eq, U: Fn(&Self::Item) -> T>(
        self,
        get_key: U,
    ) -> HashMap<T, Vec<Self::Item>> {
        let mut dict = HashMap::new();
        for v in self {
            let k = get_key(&v);
            match dict.get_mut(&k) {
                None => {
                    let mut list = vec![];
                    list.push(v);
                    dict.insert(k, list);
                }
                Some(list) => list.push(v),
            }
        }
        dict
    }
}

pub trait GroupByAggrCopy: Iterator {
    fn group_by_aggr<
        T: Hash + Eq + Deref + Copy,
        K: Fn(&Self::Item) -> T,
        R,
        A: Fn(&Vec<Self::Item>) -> R,
    >(
        self,
        get_key: K,
        aggrs: HashMap<&str, A>,
    ) -> HashMap<T, HashMap<&str, R>>;
}

impl<I: Iterator> GroupByAggrCopy for I {
    fn group_by_aggr<
        T: Hash + Eq + Deref + Copy,
        K: Fn(&Self::Item) -> T,
        R,
        A: Fn(&Vec<Self::Item>) -> R,
    >(
        self,
        get_key: K,
        aggrs: HashMap<&str, A>,
    ) -> HashMap<T, HashMap<&str, R>> {
        let mut tmp_dict = HashMap::new();
        for v in self {
            let k = get_key(&v);
            match tmp_dict.get_mut(&k) {
                None => {
                    let mut list = vec![];
                    list.push(v);
                    tmp_dict.insert(k, list);
                }
                Some(list) => list.push(v),
            }
        }
        let mut dict: HashMap<T, HashMap<&str, R>> = HashMap::new();
        for (k1, v1) in tmp_dict.iter() {
            let mut sub_dict = HashMap::new();
            for (k2, aggr) in aggrs.iter() {
                let res_aggr = aggr(v1);
                sub_dict.insert(*k2, res_aggr);
            }
            dict.insert(*k1, sub_dict);
        }
        dict
    }
}

pub trait GroupByAggrClone: Iterator {
    fn group_by_aggr_clone<
        T: Hash + Eq + Clone,
        K: Fn(&Self::Item) -> T,
        R,
        A: Fn(&Vec<Self::Item>) -> R,
    >(
        self,
        get_key: K,
        aggrs: HashMap<&str, A>,
    ) -> HashMap<T, HashMap<&str, R>>;
}

impl<I: Iterator> GroupByAggrClone for I {
    fn group_by_aggr_clone<
        T: Hash + Eq + Clone,
        K: Fn(&Self::Item) -> T,
        R,
        A: Fn(&Vec<Self::Item>) -> R,
    >(
        self,
        get_key: K,
        aggrs: HashMap<&str, A>,
    ) -> HashMap<T, HashMap<&str, R>> {
        let mut tmp_dict = HashMap::new();
        for v in self {
            let k = get_key(&v);
            match tmp_dict.get_mut(&k) {
                None => {
                    let mut list = vec![];
                    list.push(v);
                    tmp_dict.insert(k, list);
                }
                Some(list) => list.push(v),
            }
        }
        let mut dict: HashMap<T, HashMap<&str, R>> = HashMap::new();
        for (k1, v1) in tmp_dict.iter() {
            let mut sub_dict = HashMap::new();
            for (k2, aggr) in aggrs.iter() {
                let res_aggr = aggr(v1);
                sub_dict.insert(*k2, res_aggr);
            }
            dict.insert(k1.clone(), sub_dict);
        }
        dict
    }
}

#[cfg(test)]
mod ranges_tests {
    use super::*;

    #[test]
    fn to_vec() {
        let jarak = (0..100).filter(|i| i % 2 == 0).to_vec();
        println!("Result : {:?}", jarak);
    }

    #[test]
    fn distinct() {
        let result = (0..100).map(|x| x % 3).distinct();
        println!("Result : {:?}", result);
    }

    #[derive(Debug)]
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
    fn group_by() {
        let result = (0..10)
            .map(|i| Grouped {
                cat: if i % 2 == 0 { "Genap" } else { "Ganjil" },
                val: i,
            })
            .group_by(|g| g.cat);
        println!("Result : {:?}", result)
    }

    type Aggr = fn(&Vec<Grouped>) -> usize;

    #[test]
    fn group_by_aggr() {
        let mut aggrs: HashMap<&str, Aggr> = HashMap::new();
        aggrs.insert("sum", |a: &Vec<Grouped>| {
            a.iter()
                .map(|g| {
                    let i: usize = g.val.try_into().unwrap();
                    i
                })
                .sum::<usize>()
        });
        aggrs.insert("avg", |a: &Vec<Grouped>| {
            let sum = a
                .iter()
                .map(|g| {
                    let i: usize = g.val.try_into().unwrap();
                    i
                })
                .sum::<usize>();
            sum / a.len()
        });
        aggrs.insert("product", |a: &Vec<Grouped>| {
            a.iter()
                .map(|g| {
                    let i: usize = g.val.try_into().unwrap();
                    i
                })
                .product::<usize>()
        });
        aggrs.insert("max", |a: &Vec<Grouped>| {
            a.iter()
                .map(|g| {
                    let i: usize = g.val.try_into().unwrap();
                    i
                })
                .max()
                .unwrap()
        });
        aggrs.insert("min", |a: &Vec<Grouped>| {
            a.iter()
                .map(|g| {
                    let i: usize = g.val.try_into().unwrap();
                    i
                })
                .min()
                .unwrap()
        });
        aggrs.insert("len", |a| a.len());
        let result = (1..=20)
            .map(|i| Grouped {
                cat: if i % 3 == 0 {
                    "Tiga"
                } else if i % 3 == 2 {
                    "Dua"
                } else {
                    "Satu"
                },
                val: i,
            })
            .group_by_aggr(|g| g.cat, aggrs);
        println!("Result : {:?}", result)
    }
}

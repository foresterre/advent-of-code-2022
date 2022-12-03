use std::collections::HashSet;

pub fn intersect<I, Set>(sets: I) -> u8
where
    I: IntoIterator<Item = Set>,
    Set: IntoIterator<Item = u8>,
{
    let common = sets
        .into_iter()
        .map(|set| {
            let set: HashSet<u8> = HashSet::from_iter(set);
            set
        })
        .reduce(|l, r| &l & &r);

    *common.unwrap().iter().next().unwrap()
}

pub fn priority(item: u8) -> u32 {
    (if item >= b'a' {
        item + 1 - b'a'
    } else {
        item + 27 - b'A'
    }) as u32
}

fn sum(list: &[u32]) -> Option<u32> {
    let mut sum:u32 = 0;
    for item in list.iter() {
        let res = sum.checked_add(*item);
        sum = match res {
            Some(v) => v,
            None => return None
        };
    };
    Some(sum)
}
fn gdc(mut n: u64, mut m: u64) -> u64 {
    asser!(n != 0 && m !=0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
} 
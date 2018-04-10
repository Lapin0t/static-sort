use Net;


pub fn batcher(len: u64) -> Net {
    let mut net = Vec::new();

    let t = 64 - len.leading_zeros();
    let mut p = 1 << (t - 1);

    while p > 0 {
        let mut q = 1 << (t - 1);
        let mut r = 0;
        let mut d = p;

        while d > 0 {
            for i in 0..len - d {
                if i & p == r {
                    net.push((i, i + d));
                }
            }

            d = q - p;
            q >>= 1;
            r = p;
        }

        p >>= 1;
    }

    return net;
}

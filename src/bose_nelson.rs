use Net;


pub fn bose_nelson(len: u64) -> Net {
    let mut net = Vec::new();
    bn_split(&mut net, 0, len);
    return net;
}


fn bn_split(net: &mut Net, i: u64, len: u64) {
    if len >= 2 {
        let mid = len / 2;
        bn_split(net, i, mid);
        bn_split(net, i + mid, len - mid);
        bn_merge(net, i, mid, i + mid, len - mid);
    }
}

fn bn_merge(net: &mut Net, i: u64, len_i: u64, j: u64, len_j: u64) {
    match (len_i, len_j) {
        (1, 1) => net.push((i, j)),
        (1, 2) => {
            net.push((i, j+1));
            net.push((i, j));
        },
        (2, 1) => {
            net.push((i, j));
            net.push((i+1, j));
        },
        _ => {
            let mid_i = len_i / 2;
            let mid_j = if len_i & 1 == 1 {
                len_j / 2
            } else {
                (len_j + 1) / 2
            };
            bn_merge(net, i, mid_i, j, mid_j);
            bn_merge(net, i + mid_i, len_i - mid_i, j + mid_j, len_j - mid_j);
            bn_merge(net, i + mid_i, len_i - mid_i, j, mid_j);
        }
    }
}

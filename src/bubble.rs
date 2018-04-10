use Net;


pub fn bubble(len: u64) -> Net {
    let mut net = Vec::new();

    for i in (0..len).rev() {
        for j in 0..i {
            net.push((j, j+1));
        }
    }

    return net;
}

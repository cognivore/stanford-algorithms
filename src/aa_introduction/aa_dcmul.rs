// Divide and conquer multiplication
// Also known as Karatsuba multiplication
// See: https://en.wikipedia.org/wiki/Karatsuba_algorithm

crate::entry_point!("karatsuba", main);

/*
fn split(x: u64) -> (u64, u64) {
    let mask = 0x00000000ffffffff;
    ((x & mask), (x >> 32))
}
*/

fn main() -> () {
    let x = 1234567;
    let y = 9876543;
    let z = karatsuba(x, y);
    println!("{} * {} = {}", x, y, z);
}

pub fn karatsuba(x: u64, y: u64) -> u64 {
    krec(x, y, 32)
}

fn krec(x: u64, y: u64, n: u8) -> u64 {
    dbg!("{#?} / {#?} @ {#?}", x, y, n);
    if n <= 2 {
        return x * y;
    }
    let n2 = n / 2;
    let (x0, x1) = split(x, n2);
    let (y0, y1) = split(y, n2);
    let z0 = krec(x0, y0, n2);
    let z2 = krec(x1, y1, n2);
    let z1 = krec(x0 + x1, y0 + y1, n2) - z0 - z2;
    (z2 << (2 * n2)) + (z1 << n2) + z0
}

fn split(x: u64, n: u8) -> (u64, u64) {
    let mask = (1 << n) - 1;
    ((x & mask), (x >> n))
}

// Tests!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karatsuba() {
        let f = |x: u64, y: u64| -> () {
            let z = karatsuba(x, y);
            assert_eq!(z, x * y);
        };
        for i in 0..1000 {
            for j in 0..1000 {
                f(i, j);
            }
        }
    }
}

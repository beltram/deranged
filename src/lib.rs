use core::convert::Infallible;
use std::sync::atomic::{AtomicU64, Ordering};

/// Deterministic RNG for snapshot tests
#[derive(Default, Debug)]
pub struct Rng(AtomicU64);

impl rand_core::TryCryptoRng for Rng {}

impl rand_core::TryRng for Rng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        Ok(self.0.fetch_add(1, Ordering::SeqCst) as u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        Ok(self.0.fetch_add(1, Ordering::SeqCst))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
        for item in dest {
            *item = (self.0.fetch_add(1, Ordering::SeqCst) as u32 % u8::MAX as u32) as u8;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::Rng as _;

    #[test]
    fn should_be_deterministic() {
        let mut rng = Rng::default();
        assert_eq!(rng.next_u32(), 0);
        assert_eq!(rng.next_u32(), 1);
        assert_eq!(rng.next_u64(), 2);
        let mut buf = [0; 4];
        rng.fill_bytes(&mut buf);
        assert_eq!(buf, [3, 4, 5, 6]);
    }

    #[test]
    fn rng_should_be_stable() {
        let mut rng = Rng::default();
        insta::assert_snapshot!("rng_u32.txt", hex::encode(rng.next_u32().to_be_bytes()));
        insta::assert_snapshot!("rng_u64.txt", hex::encode(rng.next_u64().to_be_bytes()));
        let mut buf = vec![0; 1024];
        rng.fill_bytes(&mut buf);
        insta::assert_snapshot!("rng_bytes.txt", hex::encode(buf));
    }
}

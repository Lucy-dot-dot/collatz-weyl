#[cfg(feature = "rand-core")]
use std::convert::Infallible;
#[cfg(feature = "rand-core")]
use rand_core::{Rng, TryRng, utils, SeedableRng};

pub struct CollatzWeyl64 {
    x: u64,
    a: u64,
    weyl: u64,
    s: u64, // s must be odd
}

impl CollatzWeyl64 {
    pub fn next(&mut self) -> u64 {
        self.a = self.a.wrapping_add(self.x);
        self.weyl = self.weyl.wrapping_add(self.s);
        self.x = self.x.wrapping_shr(1).wrapping_mul(self.a | 1) ^ self.weyl;
        self.a.wrapping_shr(48) ^ self.x
    }

    pub fn new(seed: u64) -> Self {
        assert_eq!(seed & 1, 1, "seed must be odd");
        let mut generator = Self {
            x: 0,
            a: 0,
            weyl: 0,
            s: seed,
        };

        for _ in 0..48 {
            // warm up
            generator.next();
        }
        generator
    }
}

#[cfg(feature = "rand-core")]
impl TryRng for CollatzWeyl64 {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        utils::fill_bytes_via_next_word(dest, || Ok::<u32, Self::Error>(self.next_u32())).expect("Will never panic");
        Ok(())
    }
}

#[cfg(feature = "rand-core")]
impl SeedableRng for CollatzWeyl64 {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(u64::from_le_bytes(seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        Self::new(state)
    }

    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new(rng.next_u64())
    }

    fn try_from_rng<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        Ok(Self::new(rng.try_next_u64()?))
    }
}


pub struct CollatzWeyl128_64 {
    x: u128,
    a: u64,
    weyl: u64,
    s: u64,
}

impl CollatzWeyl128_64 {
    pub fn next(&mut self) -> u128 {
        self.a = self.a.wrapping_add(self.x as u64);
        self.weyl = self.weyl.wrapping_add(self.s);
        self.x = self.x.wrapping_shr(1).wrapping_mul(self.a as u128 | 1) ^ self.weyl as u128;
        self.a.wrapping_shr(48) as u128 ^ self.x
    }

    pub fn new(seed: u64) -> Self {
        assert_eq!(seed & 1, 1, "seed must be odd");
        let mut generator = Self {
            x: 0,
            a: 0,
            weyl: 0,
            s: seed,
        };

        for _ in 0..48 {
            // warm up
            generator.next();
        }
        generator
    }
}

#[cfg(feature = "rand-core")]
impl TryRng for CollatzWeyl128_64 {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        utils::fill_bytes_via_next_word(dest, || Ok::<u32, Self::Error>(self.next_u32())).expect("Will never panic");
        Ok(())
    }
}

#[cfg(feature = "rand-core")]
impl SeedableRng for CollatzWeyl128_64 {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(u64::from_le_bytes(seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        Self::new(state)
    }

    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new(rng.next_u64())
    }

    fn try_from_rng<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        Ok(Self::new(rng.try_next_u64()?))
    }
}


pub struct CollatzWeyl128 {
    c: [u128; 4] // c[0] must be odd
}

impl CollatzWeyl128 {
    pub fn next(&mut self) -> u128 {
        self.c[2] = self.c[2].wrapping_add(self.c[1]);
        self.c[3] = self.c[3].wrapping_add(self.c[0]);
        self.c[1] = self.c[1].wrapping_shr(1).wrapping_mul(self.c[2] | 1) ^ self.c[3];
        self.c[2].wrapping_shr(96) ^ self.c[1]
    }

    pub fn new(seed: u128) -> Self {
        assert_eq!(seed & 1, 1, "seed must be odd");
        let mut generator = Self {
            c: [0; 4],
        };
        generator.c[0] = seed;

        for _ in 0..48 {
            // warm up
            generator.next();
        }
        generator
    }
}

#[cfg(feature = "rand-core")]
impl TryRng for CollatzWeyl128 {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        utils::fill_bytes_via_next_word(dest, || Ok::<u32, Self::Error>(self.next_u32())).expect("Will never panic");
        Ok(())
    }
}

#[cfg(feature = "rand-core")]
impl SeedableRng for CollatzWeyl128 {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(u128::from_le_bytes(seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        Self::new((state as u128) << 64 | state as u128)
    }

    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new((rng.next_u64() as u128) << 64 | rng.next_u64() as u128)
    }

    fn try_from_rng<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        Ok(Self::new((rng.try_next_u64()? as u128) << 64 | rng.try_next_u64()? as u128))
    }
}

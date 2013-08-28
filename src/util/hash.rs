use std::util;
use std::hash::Hash;

/// Hash function.
pub trait HashFun<K> {
    /// Hash function.
    fn hash(&self, &K) -> uint;
}

/// Hash function for everything implementing the `Hash` trait.
pub struct StdHash { priv unused: uint } // FIXME: ICE if the struct is zero-sized

impl StdHash {
    /// Creates a new StdHash.
    pub fn new() -> StdHash {
        StdHash { unused: 0 }
    }
}

impl<T: Hash> HashFun<T> for StdHash {
    #[inline]
    fn hash(&self, t: &T) -> uint {
        t.hash() as uint
    }
}

/// Hash function for pairs of uint, using the Tomas Wang hash.
pub struct UintPairTWHash { priv unused: uint }

impl UintPairTWHash {
    /// Creates a new UintPairTWHash.
    pub fn new() -> UintPairTWHash {
        UintPairTWHash { unused: 0 }
    }
}

impl HashFun<(uint, uint)> for UintPairTWHash {
    #[inline]
    fn hash(&self, &(a, b): &(uint, uint)) -> uint {
        let mut ia = a;
        let mut ib = b;

        if ia > ib {
            util::swap(&mut ia, &mut ib)
        }

        tomas_wang_hash(key_from_pair(ia, ib)) as uint
    }
}

/// Hash function for uint.
pub struct UintTWHash { priv unused: uint } // FIXME: ICE if the struct is zero-sized

impl UintTWHash {
    /// Creates a new UintTWHash.
    pub fn new() -> UintTWHash {
        UintTWHash { unused: 0 }
    }
}

impl HashFun<uint> for UintTWHash {
    #[inline]
    fn hash(&self, a: &uint) -> uint {
        tomas_wang_hash(*a)
    }
}

/// Combines two uint on a single one.
#[cfg(target_word_size = "32")] #[inline]
pub fn key_from_pair(a: uint, b: uint) -> uint {
    (a & 0xffff) | (b << 16)
}

/// Combines two uint on a sigle one.
#[cfg(target_word_size = "64")] #[inline]
pub fn key_from_pair(a: uint, b: uint) -> uint {
    (a & 0xffffffff) | (b << 32)
}

// http://www.concentric.net/~Ttwang/tech/inthash.htm -- dead link!
// (this one works: http://naml.us/blog/tag/thomas-wang)
/// Tomas Wang integer hash function.
#[cfg(target_word_size = "64")] #[inline]
pub fn tomas_wang_hash(k: uint) -> uint {
    let mut res = k;

    res = res + !(res << 32);
    res = res ^ (res >> 22);
    res = res + !(res << 13);
    res = res ^ (res >> 8);
    res = res + (res << 3);
    res = res ^ (res >> 15);
    res = res + !(res << 27);
    res = res ^ (res >> 31);

    res
}

/// Tomas Wang integer hash function.
#[cfg(target_word_size = "32")] #[inline]
pub fn tomas_wang_hash(k: uint) -> uint {
    let mut res = k;

    res = res + !(res << 15);
    res = res ^ (res >> 10);
    res = res + (res << 3);
    res = res ^ (res >> 6);
    res = res + !(res << 11);
    res = res ^ (res >> 16);

    res
}

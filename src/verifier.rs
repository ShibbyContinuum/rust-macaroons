use std::sync::Arc;
use std::rc::Rc;

pub trait Verifier {
    fn verify(&self, caveat: &[u8]) -> bool;
}

// Pointer primitives

impl<'a, V: Verifier> Verifier for &'a V {
    fn verify(&self, caveat: &[u8]) -> bool {
        (**self).verify(caveat)
    }
}

impl<V: Verifier> Verifier for Box<V> {
    fn verify(&self, caveat: &[u8]) -> bool {
        (**self).verify(caveat)
    }
}

impl<V: Verifier> Verifier for Rc<V> {
    fn verify(&self, caveat: &[u8]) -> bool {
        (**self).verify(caveat)
    }
}

impl<V: Verifier> Verifier for Arc<V> {
    fn verify(&self, caveat: &[u8]) -> bool {
        (**self).verify(caveat)
    }
}

// Func

pub struct Func<F: Fn(&str) -> bool>(pub F);

impl<F> Verifier for Func<F> where
    F: Fn(&str) -> bool
{
    fn verify(&self, caveat: &[u8]) -> bool {
        ::std::str::from_utf8(&caveat)
        .map(&self.0)
        .unwrap_or(false)
    }
}

// ByteFunc

pub struct ByteFunc<F: Fn(&[u8]) -> bool>(F);

impl<F> Verifier for ByteFunc<F> where
    F: Fn(&[u8]) -> bool
{
    fn verify(&self, caveat: &[u8]) -> bool {
        (self.0)(caveat)
    }
}

// LinkedVerifier

pub struct LinkedVerifier<V1: Verifier, V2: Verifier> {
    verifier1: V1,
    verifier2: V2,
}

impl<V1: Verifier, V2: Verifier> LinkedVerifier<V1, V2> {
    pub fn from(verifier1: V1, verifier2: V2) -> Self {
        LinkedVerifier {
            verifier1: verifier1,
            verifier2: verifier2,
        }
    }
}

impl<V1: Verifier, V2: Verifier> Verifier for LinkedVerifier<V1, V2> {
    fn verify(&self, caveat: &[u8]) -> bool {
           self.verifier1.verify(caveat)
        || self.verifier2.verify(caveat)
    }
}

// Eq

pub struct Eq<Tag: AsRef<[u8]>, Value: AsRef<[u8]>>(pub Tag, pub Value);

impl<Tag: AsRef<[u8]>, Value: AsRef<[u8]>> Verifier for Eq<Tag, Value> {
    fn verify(&self, caveat: &[u8]) -> bool {
        let tag = self.0.as_ref();
        let op = b" = ";
        let value = self.1.as_ref();
        let len = tag.len() + op.len() + value.len();

        len == caveat.len()
        && &caveat[0                    .. tag.len()           ] == tag
        && &caveat[tag.len()            .. tag.len() + op.len()] == op
        && &caveat[tag.len() + op.len() ..                     ] == value
    }
}

// LinkVerifier

pub trait LinkVerifier: Verifier + Sized {
    fn link<V: Verifier>(self, verifier: V) -> LinkedVerifier<V, Self>;
}

impl<T: Verifier> LinkVerifier for T {
    fn link<V: Verifier>(self, verifier: V) -> LinkedVerifier<V, Self> {
        LinkedVerifier::from(verifier, self)
    }
}

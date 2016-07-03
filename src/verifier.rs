pub use token::Token;
pub use caveat::Caveat;

pub type CaveatVerifier = Fn(&[u8]) -> bool;

pub struct Verifier {
    pub matchers: Vec<Box<CaveatVerifier>>,
}

impl Verifier {
    pub fn new() -> Verifier {
        Self::with_matchers(Vec::new())
    }

    pub fn with_matchers(matchers: Vec<Box<CaveatVerifier>>) -> Verifier {
        Verifier { matchers: matchers }
    }

    pub fn add_matcher<M>(self, matcher: M) -> Verifier where
        M: Fn(&str) -> bool + 'static
    {
        self.add_byte_matcher(move |caveat|
            ::std::str::from_utf8(&caveat)
            .map(&matcher)
            .unwrap_or(false)
        )
    }

    pub fn add_byte_matcher<M>(mut self, matcher: M) -> Verifier where
        M: Fn(&[u8]) -> bool + 'static
    {
        self.matchers.push(Box::new(matcher));
        self
    }
        
    pub fn verify(&self, key: &[u8], token: &Token) -> bool {
        if !token.verify(&key) {
            return false;
        }

        for c in &token.caveats {
            let verified = match c.verification_id {
                None => self.verify_first_party(c),
                _ => self.verify_third_party()
            };
            if verified == false {
                return false;
            }
        }
        true
    }

    fn verify_first_party(&self, c: &Caveat) -> bool {
        let matchers = &self.matchers;
        for m in matchers {
            if m(&c.caveat_id) {
                return true;
            }
        }
        false
    }

    fn verify_third_party(&self) -> bool {
        unimplemented!();
    }
}

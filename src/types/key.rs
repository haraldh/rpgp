use crate::crypto::public_key::PublicKeyAlgorithm;
use crate::types::KeyId;

pub trait KeyTrait: ::std::fmt::Debug {
    fn fingerprint(&self) -> Vec<u8>;

    /// Returns the Key ID of the associated primary key.
    fn key_id(&self) -> KeyId;

    fn algorithm(&self) -> PublicKeyAlgorithm;

    fn is_signing_key(&self) -> bool {
        use crate::crypto::PublicKeyAlgorithm::*;

        match self.algorithm() {
            RSA | RSASign | ElgamalSign | DSA | EdDSA => true,
            _ => false,
        }
    }
}

impl<'a, T: KeyTrait> KeyTrait for &'a T {
    fn fingerprint(&self) -> Vec<u8> {
        (*self).fingerprint()
    }

    /// Returns the Key ID of the associated primary key.
    fn key_id(&self) -> KeyId {
        (*self).key_id()
    }

    fn algorithm(&self) -> PublicKeyAlgorithm {
        (*self).algorithm()
    }
}

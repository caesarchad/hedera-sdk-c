use crate::claim::CClaim;
use hedera::{query::QueryCryptoGetClaim, AccountId};

def_query!(QueryCryptoGetClaim: crypto_get_claim(AccountId, Vec<u8>) -> CClaim);

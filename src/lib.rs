//! Deep Space
//!
//! A Library for constructing Cosmos SDK JSOn format transactions and signable bytes
//!
//!

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, trivial_casts, unused_qualifications)]

pub mod address;
pub mod canonical_json;
pub mod coin;
pub mod msg;
pub mod stdfee;
pub mod stdsigndoc;
pub mod stdsignmsg;
pub mod stdtx;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

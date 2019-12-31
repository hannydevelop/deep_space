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

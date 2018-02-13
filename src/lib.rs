#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LorentzVector {
    pub e: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

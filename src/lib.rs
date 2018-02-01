#[derive(Clone, Debug, PartialEq)]
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

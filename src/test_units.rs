use crate::units;

mod test {

    use super::*;

    #[test]
    fn test_calculate_size() {
        let k = units::MemoryUnit::Kilobyte;

        let four_k = k.size() * 4;
        assert_eq!(four_k, 4096);
        
        let four_k = k.decimal_size() * 4;
        assert_eq!(four_k, 4000);
    }
}

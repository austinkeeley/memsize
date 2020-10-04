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


    #[test]
    fn test_parse() {
        let b: units::MemoryUnit = "b".parse().expect("Could not parse");
        assert_eq!(b, units::MemoryUnit::Byte);
        let b: units::MemoryUnit = "byte".parse().expect("Could not parse");
        assert_eq!(b, units::MemoryUnit::Byte);

        let kb: units::MemoryUnit = "kb".parse().expect("Could not parse");
        assert_eq!(kb, units::MemoryUnit::Kilobyte);
        let kb: units::MemoryUnit = "kilobyte".parse().expect("Could not parse");
        assert_eq!(kb, units::MemoryUnit::Kilobyte);

        let mb: units::MemoryUnit = "mb".parse().expect("Could not parse");
        assert_eq!(mb, units::MemoryUnit::Megabyte);
        let mb: units::MemoryUnit = "megabyte".parse().expect("Could not parse");
        assert_eq!(mb, units::MemoryUnit::Megabyte);

        let gb: units::MemoryUnit = "gb".parse().expect("Could not parse");
        assert_eq!(gb, units::MemoryUnit::Gigabyte);
        let gb: units::MemoryUnit = "gigabyte".parse().expect("Could not parse");
        assert_eq!(gb, units::MemoryUnit::Gigabyte);

        let tb: units::MemoryUnit = "tb".parse().expect("Could not parse");
        assert_eq!(tb, units::MemoryUnit::Terabyte);
        let tb: units::MemoryUnit = "terabyte".parse().expect("Could not parse");
        assert_eq!(tb, units::MemoryUnit::Terabyte);

        let pb: units::MemoryUnit = "pb".parse().expect("Could not parse");
        assert_eq!(pb, units::MemoryUnit::Petabyte);
        let pb: units::MemoryUnit = "petabyte".parse().expect("Could not parse");
        assert_eq!(pb, units::MemoryUnit::Petabyte);

        let eb: units::MemoryUnit = "eb".parse().expect("Could not parse");
        assert_eq!(eb, units::MemoryUnit::Exabyte);
        let eb: units::MemoryUnit = "exabyte".parse().expect("Could not parse");
        assert_eq!(eb, units::MemoryUnit::Exabyte);

        let zb: units::MemoryUnit = "zb".parse().expect("Could not parse");
        assert_eq!(zb, units::MemoryUnit::Zettabyte);
        let zb: units::MemoryUnit = "zettabyte".parse().expect("Could not parse");
        assert_eq!(zb, units::MemoryUnit::Zettabyte);

        let yb: units::MemoryUnit = "yb".parse().expect("Could not parse");
        assert_eq!(yb, units::MemoryUnit::Yottabyte);
        let yb: units::MemoryUnit = "yottabyte".parse().expect("Could not parse");
        assert_eq!(yb, units::MemoryUnit::Yottabyte);

        let unknown: Result<units::MemoryUnit, String> = "unknown".parse();
        match unknown {
            Ok(_) => panic!("Incorrectly parsed"),
            _ => {},
        }
        
    }
}

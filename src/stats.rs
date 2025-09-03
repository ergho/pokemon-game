use std::fmt;

/// Strongly typed stat: must be > 0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stat(u16);

impl Stat {
    pub fn new(value: u16) -> Result<Self, String> {
        if value == 0 {
            Err("Stat must be greater than 0".into())
        } else {
            Ok(Self(value))
        }
    }

    pub fn get(self) -> u16 {
        self.0
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Collection of base stats for a species
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseStats {
    pub attack: Stat,
    pub defense: Stat,
    pub max_hp: Stat,
    pub speed: Stat,
}

impl BaseStats {
    pub fn new(attack: Stat, defense: Stat, max_hp: Stat, speed: Stat) -> Self {
        Self {
            attack,
            defense,
            max_hp,
            speed,
        }
    }

    pub fn attack(&self) -> u16 {
        self.attack.get()
    }
    pub fn defense(&self) -> u16 {
        self.defense.get()
    }
    pub fn max_hp(&self) -> u16 {
        self.max_hp.get()
    }
    pub fn speed(&self) -> u16 {
        self.speed.get()
    }
}

/// Individual stats for a creature (currently just a copy of base stats)
#[derive(Debug, Clone)]
pub struct IndividualStats {
    pub attack: Stat,
    pub defense: Stat,
    pub max_hp: Stat,
    pub speed: Stat,
}

impl IndividualStats {
    /// Create individual stats from base stats (future: apply level/IV formulas)
    pub fn from_base(base: &BaseStats) -> Self {
        Self {
            attack: Stat::new(base.attack()).unwrap(),
            defense: Stat::new(base.defense()).unwrap(),
            max_hp: Stat::new(base.max_hp()).unwrap(),
            speed: Stat::new(base.speed()).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stat_creation() {
        let s = Stat::new(10).unwrap();
        assert_eq!(s.get(), 10);
        assert!(Stat::new(0).is_err());
    }

    #[test]
    fn basestats_creation() {
        let bs = BaseStats::new(
            Stat::new(10).unwrap(),
            Stat::new(8).unwrap(),
            Stat::new(30).unwrap(),
            Stat::new(12).unwrap(),
        );
        assert_eq!(bs.attack(), 10);
        assert_eq!(bs.defense(), 8);
        assert_eq!(bs.max_hp(), 30);
        assert_eq!(bs.speed(), 12);
    }

    #[test]
    fn individualstats_creation() {
        let bs = BaseStats::new(
            Stat::new(10).unwrap(),
            Stat::new(8).unwrap(),
            Stat::new(30).unwrap(),
            Stat::new(12).unwrap(),
        );
        let ind = IndividualStats::from_base(&bs);
        assert_eq!(ind.attack.get(), 10);
        assert_eq!(ind.defense.get(), 8);
        assert_eq!(ind.max_hp.get(), 30);
        assert_eq!(ind.speed.get(), 12);
    }
}

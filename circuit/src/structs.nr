pub struct Preferences {
    pub age_min: u8,
    pub age_max: u8,
    pub gender: u8, // 0: zk researcher, 1: security researcher, 2: doesn't matter
}

pub struct User {
    pub id: Field,
    pub age: u8,
    pub gender: u8, // 0: zk researcher, 1: security researcher
    pub region: u8,
    pub interests: [u8; 3],
    pub preferences: Preferences,
}

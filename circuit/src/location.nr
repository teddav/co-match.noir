use crate::structs::User;

pub fn check_region(user1: User, user2: User) -> bool {
    assert(user1.region < regions.len() as u8);
    let mut is_match: bool = true;

    if user1.region != user2.region {
        is_match = false;
    }

    is_match
}

global regions: [u8; 4] = [001, 002, 003, 004];

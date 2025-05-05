mod structs;
mod location;

use structs::User;

fn check_age(user1: User, user2: User) -> bool {
    let mut is_match: bool = true;

    // user1 preferences
    if (user1.age < user2.preferences.age_min) | (user1.age > user2.preferences.age_max) {
        is_match = false;
    }

    // user2 preferences
    if (user2.age < user1.preferences.age_min) | (user2.age > user1.preferences.age_max) {
        is_match = false;
    }

    is_match
}

fn check_gender(user1: User, user2: User) -> bool {
    let mut is_match: bool = true;

    // user1 preferences
    if (user1.preferences.gender != user2.gender) & (user1.preferences.gender != 2) {
        is_match = false;
    }

    // user2 preferences
    if (user2.preferences.gender != user1.gender) & (user2.preferences.gender != 2) {
        is_match = false;
    }

    is_match
}

fn check_interests(user1: User, user2: User) -> bool {
    let mut is_match: bool = false;

    for interest in user1.interests {
        if user2.interests.any(|i| i == interest) {
            is_match = true;
        }
    }
    is_match
}

fn main(user1: User, user2: User) -> pub bool {
    // removed hash to make circuit smaller
    // let id_nullifier_hash1 = std::hash::poseidon2::Poseidon2::hash([user1.id_nullifier], 1);
    // let id_nullifier_hash2 = std::hash::poseidon2::Poseidon2::hash([user2.id_nullifier], 1);
    // assert(user1.id == id_nullifier_hash1);
    // assert(user2.id == id_nullifier_hash2);
    assert(user1.id != user2.id);

    let mut is_match: bool = true;

    is_match &= check_age(user1, user2);
    is_match &= check_gender(user1, user2);
    is_match &= location::check_region(user1, user2);
    is_match &= check_interests(user1, user2);

    assert(is_match);
    is_match
}

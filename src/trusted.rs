use std::cell::OnceCell;

const TRUSTED_USERS: OnceCell<Vec<String>> = OnceCell::new();

pub fn user(name: &str) -> bool {
    let list = TRUSTED_USERS;
    list.get_or_init(|| {
        std::fs::read_to_string("trusted_users.txt")
            .unwrap_or_default()
            .lines()
            .map(ToOwned::to_owned)
            .collect()
    })
    .iter()
    .any(|line| line == name)
}

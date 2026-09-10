pub mod sequence_extensions;

#[cfg(test)]
#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
}

#[cfg(test)]
fn sample_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
        },
        User {
            id: 3,
            name: "Bob".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::sample_users;
    use super::sequence_extensions::{
        All, Any, ElementAtOrDefault, FirstOrDefault, GroupBy, LastOrDefault, SkipTakeOwned,
        TakeOwned, TakeSlice,
    };
    use crate::sequence_extensions::{Range, RangeGenerator};

    #[test]
    fn sample_users_returns_expected_ids() {
        let users = sample_users();

        let ids = users.iter().map(|user| user.id).collect::<Vec<_>>();

        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn all_returns_false_when_one_item_does_not_match() {
        let values = vec![2, 4, 6, 7];

        assert!(!values.all(|value| *value % 2 == 0));
    }

    #[test]
    fn any_returns_true_when_one_item_matches() {
        let values = vec![1, 3, 5, 8];

        assert!(values.any(|value| *value > 7));
    }

    #[test]
    fn elementat_or_default_returns_fourth_item() {
        let values = vec![3, 5, 11, -3, 45];

        let fourth_number = values.elementat_or_default(3);

        assert_eq!(fourth_number, -3);
    }

    #[test]
    fn first_or_default_returns_first_item() {
        let values = vec![10, 20, 30];

        assert_eq!(values.first_or_default(), 10);
    }

    #[test]
    fn last_or_default_returns_last_item() {
        let values = vec![10, 20, 30];

        assert_eq!(values.last_or_default(), 30);
    }

    #[test]
    fn skip_take_owned_returns_requested_range() {
        let values = vec![1, 2, 3, 4, 5];

        assert_eq!(values.skip_take_owned(1, 2), vec![2, 3]);
    }

    #[test]
    fn take_n_returns_borrowed_prefix() {
        let values = vec![1, 2, 3, 4, 5];

        assert_eq!(values.take_n(3), &[1, 2, 3]);
    }

    #[test]
    fn take_owned_returns_owned_prefix() {
        let values = vec![1, 2, 3, 4, 5];

        assert_eq!(values.take_owned(4), vec![1, 2, 3, 4]);
    }

    #[test]
    fn groupby_returns_expected_count() {
        let users = sample_users();
        let grouped_users = users.group_by(|user| user.name.clone());

        assert_eq!(grouped_users.len(), 2);
        assert_eq!(grouped_users.get("Alice").map(Vec::len), Some(1));
        assert_eq!(grouped_users.get("Bob").map(Vec::len), Some(2));
    }

    #[test]
    fn range_returns_expected_count() {
        let range = RangeGenerator::range(-5, 10);
        let expected_nums = [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4];

        assert_eq!(range, expected_nums);
    }

}

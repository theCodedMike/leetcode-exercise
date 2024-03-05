use leetcode_exercise::leetcode::editor::cn::_93_restore_i_p_addresses::Solution;
use std::collections::HashSet;

#[test]
fn restore_ip_addresses_1() {
    let s = "25525511135".to_string();
    let res = Solution::restore_ip_addresses(s);

    assert_eq!(res.len(), 2);
    let set = HashSet::from(["255.255.11.135".to_string(), "255.255.111.35".to_string()]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn restore_ip_addresses_2() {
    let s = "0000".to_string();
    let res = Solution::restore_ip_addresses(s);

    assert_eq!(res.len(), 1);
    let set = HashSet::from(["0.0.0.0".to_string()]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn restore_ip_addresses_3() {
    let s = "101023".to_string();
    let res = Solution::restore_ip_addresses(s);

    assert_eq!(res.len(), 5);
    let set = HashSet::from([
        "1.0.10.23".to_string(),
        "1.0.102.3".to_string(),
        "10.1.0.23".to_string(),
        "10.10.2.3".to_string(),
        "101.0.2.3".to_string(),
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn restore_ip_addresses_4() {
    let s = "0279245587303".to_string();
    let res = Solution::restore_ip_addresses(s);

    assert_eq!(res.len(), 0);
}

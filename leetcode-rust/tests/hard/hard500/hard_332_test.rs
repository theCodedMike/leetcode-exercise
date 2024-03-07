use leetcode_rust::leetcode::editor::cn::_332_reconstruct_itinerary::Solution;

#[test]
fn reconstruct_itinerary_1() {
    // MUC -> LHR -> SFO
    //  ↑             ↓
    // JFK           SJC
    let tickets = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    let res = Solution::find_itinerary(tickets);

    assert_eq!(res, ["JFK", "MUC", "LHR", "SFO", "SJC"]);
}

#[test]
fn reconstruct_itinerary_2() {
    // SFO
    //  ↑  ↖↘
    // JFK ⇄ ATL
    let tickets = vec![
        vec!["JFK".to_string(), "SFO".to_string()],
        vec!["JFK".to_string(), "ATL".to_string()],
        vec!["SFO".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "JFK".to_string()],
        vec!["ATL".to_string(), "SFO".to_string()],
    ];
    let res = Solution::find_itinerary(tickets);

    assert_eq!(res, ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]);
}

#[test]
fn reconstruct_itinerary_3() {
    let tickets = vec![
        vec!["JFK".to_string(), "SFO".to_string()],
        vec!["JFK".to_string(), "ATL".to_string()],
        vec!["SFO".to_string(), "JFK".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "AAA".to_string()],
        vec!["AAA".to_string(), "BBB".to_string()],
        vec!["BBB".to_string(), "ATL".to_string()],
    ];
    let res = Solution::find_itinerary(tickets);

    assert_eq!(
        res,
        [
            "JFK", "SFO", "JFK", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB",
            "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB",
            "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB",
            "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB",
            "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB",
            "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB",
            "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL", "AAA", "BBB", "ATL"
        ]
    );
}

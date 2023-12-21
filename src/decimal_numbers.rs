pub(crate) fn extract_decimal_number(rest: &[char]) -> u32 {
    let first = rest
        .first()
        .and_then(|c| -> Option<u32> {c.to_digit(10)});

    let second = rest
        .get(1)
        .and_then(|c| -> Option<u32> {c.to_digit(10)});

    let third = rest
        .get(2)
        .and_then(|c| -> Option<u32> {c.to_digit(10)});

    match (first, second, third) {
        (Some(hundreds),  Some(tens), Some(ones)) => 100*hundreds+10*tens+ones,
        (Some(tens), Some(ones), None) => 10*tens+ones,
        (None,  Some(tens), Some(ones)) => 10*tens+ones,
        (_, None, Some(ones)) => ones,
        (Some(ones),None, None) => ones, 
        (None, Some(ones), None) => ones,
        (None, None, None) => panic!(),
    }
}

pub(crate) fn boat_race() {
    let data = 
        "Time:        35696887
        Distance:   213116810861248";

    let (time, distance) = data.split_once('\n').unwrap();

    let time: Vec<u64>  = time.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    let distance: Vec<u64> = distance.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    let races:Vec<(&u64,&u64)> = time.iter().zip(distance.iter()).collect();

    println!("{races:?}");
  
    let sum: u64 = races.iter()
        .map(|&race| number_of_ways_to_win(race))
        .product();

    println!("Sum: {} ",sum)
}

fn number_of_ways_to_win(race: (&u64,&u64)) -> u64 {
    let total_time = *race.0 as f64;
    let distance_record = *race.1 as f64;
    let d_1 = 0.5*(total_time + f64::sqrt(total_time.powi(2) - 4.0*(distance_record + 1.0)));
    let d_2 = 0.5*(total_time - f64::sqrt(total_time.powi(2) - 4.0*(distance_record + 1.0)));

    let start = d_2.ceil() as u32;
    let finish = d_1.floor() as u32;

    Vec::from_iter(start..=finish).len() as u64 
}

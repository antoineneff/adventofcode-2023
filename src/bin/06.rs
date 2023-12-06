fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let times: [u32; 4] = [44, 82, 69, 81];
    let distances: [u32; 4] = [202, 1076, 1138, 1458];

    let mut races_wins = Vec::new();
    for i in 0..times.len() {
        let race_time = times[i];
        let race_distance = distances[i];
        let mut race_wins = 0;
        for speed in 0..=race_time {
            let result = speed * (race_time - speed);
            if result > race_distance {
                race_wins += 1;
            }
        }
        races_wins.push(race_wins);
    }

    let total: usize = races_wins.iter().product();

    println!("Part 1: {total}")
}

fn part_two() {
    const TIME: u64 = 44826981;
    const DISTANCE: u64 = 202107611381458;

    let mut race_wins = 0;
    for speed in 0..=TIME {
        let result = speed * (TIME - speed);
        if result > DISTANCE {
            race_wins += 1;
        }
    }

    println!("Part 2: {race_wins}")
}

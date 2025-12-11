use std::collections::HashMap;

use crate::utils::get_input;

const USE_TEST_DATA: bool = false;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

#[allow(dead_code)]
const TEST_DATA_2: &str = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

fn get_data(part: usize) -> HashMap<String, Vec<String>> {
    let mut data_str = get_input(11).unwrap();
    if USE_TEST_DATA {
        if part == 1 {
            data_str = TEST_DATA.to_string();
        } else {
            data_str = TEST_DATA_2.to_string();
        }
    }

    data_str
        .trim()
        .split("\n")
        .map(|r| {
            let (k, v) = r.split_once(": ").unwrap();
            (k.to_string(), v.split(' ').map(|c| c.to_string()).collect())
        })
        .collect()
}

// use recursion with caching
// Calculate the number of possible paths between two devices
fn walk(
    data: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, i64>,
    device: &str,
    target: &str,
) -> i64 {
    // On cache hit return cached value
    if let Some(c) = cache.get(device) {
        return *c;
    }
    // When target is reached return 1
    // Recursion base case
    if device == target {
        return 1;
    }
    // get connected devices and calculate the sum of possible path recursively
    // zero when no device is connected
    let sum = match data.get(device) {
        Some(connections) => connections
            .iter()
            .map(|device| walk(data, cache, device, target))
            .sum::<i64>(),
        None => 0,
    };
    // cache calculated sum
    cache.insert(device.to_string(), sum);
    sum
}

pub fn part_1() {
    let data = get_data(1);

    let sum = walk(&data, &mut HashMap::new(), "you", "out");

    assert_eq!(sum, 636);
    println!("Part 1: {sum}");
}

pub fn part_2() {
    let data = get_data(2);

    // calculate the possible paths from one station to another
    // svr -> fft -> dac -> out
    let srv_fft = walk(&data, &mut HashMap::new(), "svr", "fft");
    let fft_dac = walk(&data, &mut HashMap::new(), "fft", "dac");
    let dac_out = walk(&data, &mut HashMap::new(), "dac", "out");
    let svr_fft_dac_out = srv_fft * fft_dac * dac_out;

    // svr -> dac -> fft -> out
    let srv_dac = walk(&data, &mut HashMap::new(), "svr", "dac");
    let dac_fft = walk(&data, &mut HashMap::new(), "dac", "fft");
    let fft_out = walk(&data, &mut HashMap::new(), "fft", "out");
    let svr_dac_fft_out = srv_dac * dac_fft * fft_out;

    // Multiplying the possible paths between the stations gives the number of possible paths
    // Adding both possible combinations gives the absolute number
    let sum = svr_fft_dac_out + svr_dac_fft_out;

    assert_eq!(sum, 509312913844956);
    println!("Part 2: {sum}");
}

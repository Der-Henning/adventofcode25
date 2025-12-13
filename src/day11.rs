use std::collections::HashMap;

use crate::utils::get_input;

const USE_TEST_DATA: bool = false;

const TEST_DATA_PART_1: &str = r#"
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

const TEST_DATA_PART_2: &str = r#"
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

// the device names are always made of 3 characters.
// so I store them in 3 bytes fixed length arrays
type Device = [u8; 3];

fn device(name: &str) -> Device {
    name.as_bytes().try_into().unwrap()
}

fn get_data(part: usize) -> HashMap<Device, Vec<Device>> {
    let data_str = if USE_TEST_DATA {
        if part == 1 {
            TEST_DATA_PART_1.to_string()
        } else {
            TEST_DATA_PART_2.to_string()
        }
    } else {
        get_input(11).unwrap()
    };

    data_str
        .trim()
        .split("\n")
        .map(|r| {
            let (k, v) = r.split_once(": ").unwrap();
            (device(k), v.split_whitespace().map(device).collect())
        })
        .collect()
}

// use recursion with caching
// Calculate the number of possible paths between two devices
// walking backwards from target to start
fn walk(
    data: &HashMap<Device, Vec<Device>>,
    cache: &mut HashMap<Device, u64>,
    start: &Device,
    target: &Device,
) -> u64 {
    // When target is reached return 1
    // Recursion base case
    if start == target {
        return 1;
    }
    // On cache hit return cached value
    if let Some(cached_sum) = cache.get(start) {
        return *cached_sum;
    }
    // get connected devices and calculate the sum of possible path recursively
    // zero when no device is connected
    let sum = match data.get(start) {
        Some(connections) => connections
            .iter()
            .map(|next| walk(data, cache, next, target))
            .sum(),
        None => 0,
    };
    // cache calculated sum
    cache.insert(*start, sum);
    sum
}

pub fn part_1() {
    let data = get_data(1);

    // simply walk all paths from "you" to "out"
    let sum = walk(&data, &mut HashMap::new(), &device("you"), &device("out"));

    assert_eq!(sum, 636);
    println!("Part 1: {sum}");
}

pub fn part_2() {
    let data = get_data(2);

    // calculate the possible paths from one station to another
    // multiplying the options for each part gives the full number of possible paths
    // svr -> fft -> dac -> out
    let srv_fft = walk(&data, &mut HashMap::new(), &device("svr"), &device("fft"));
    let fft_dac = walk(&data, &mut HashMap::new(), &device("fft"), &device("dac"));
    let dac_out = walk(&data, &mut HashMap::new(), &device("dac"), &device("out"));
    let svr_fft_dac_out = srv_fft * fft_dac * dac_out;

    // svr -> dac -> fft -> out
    // dac -> fft is actually 0 in the given data
    // so this part could be skipped in this case
    let srv_dac = walk(&data, &mut HashMap::new(), &device("svr"), &device("dac"));
    let dac_fft = walk(&data, &mut HashMap::new(), &device("dac"), &device("fft"));
    let fft_out = walk(&data, &mut HashMap::new(), &device("fft"), &device("out"));
    let svr_dac_fft_out = srv_dac * dac_fft * fft_out;

    // Adding both possible combinations gives the absolute number
    let sum = svr_fft_dac_out + svr_dac_fft_out;

    assert_eq!(sum, 509_312_913_844_956);
    println!("Part 2: {sum}");
}

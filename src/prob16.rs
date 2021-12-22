use std::slice::Chunks;
use std::fmt;
use std::fmt::Formatter;
use crate::read_lines;
use crate::util::{char_at, parse_binary, parse_binary_substring};


struct Packet {
    version: usize,
    type_id: usize, // anything != 4
    // indicator bit
    value: usize,
    subpackets: Vec<Packet>
}

pub(crate) fn prob16() {
    let lines: Vec<String> = read_lines("./input/prob16.txt");
    let line_hex: String = lines.clone().into_iter().nth(0).unwrap();
    println!("{}", part1(line_hex.clone()));
    println!("{}", part2(line_hex.clone()));
}


//  what do you get if you add up the version numbers in all packets?
fn part1(s: String) -> usize {
    let transmission: String = s.chars().map(|c| expand_hex(c)).collect();
    // println!("{}", bin);
    println!("{}", decode_literal("110100101111111000101000".to_string(), 6).0);
    let (packet, cursor) = decode(transmission, 0);
    // Right now, the specific operations aren't important; focus on parsing the hierarchy of sub-packets.
    return sum_versions(packet);
}


fn part2(s: String) -> usize {
    let transmission: String = s.chars().map(|c| expand_hex(c)).collect();
    let (packet, cursor) = decode(transmission, 0);
    return value_of(packet);
}


fn value_of(packet: Packet) -> usize {
    let subvalues: Vec<usize> = packet.subpackets.into_iter().map(|subpacket| value_of(subpacket)).collect();
    match packet.type_id {
        // sum
        0 => subvalues.into_iter().sum(),
        // prod
        1 => subvalues.into_iter().product(),
        // min
        2 => subvalues.into_iter().min().unwrap(),
        // max
        3 => subvalues.into_iter().max().unwrap(),
        // literal
        4 => packet.value,
        // greater than
        5 => if subvalues[0] > subvalues[1] {1} else {0},
        // less than
        6 => if subvalues[0] < subvalues[1] {1} else {0},
        // equal
        7 => if subvalues[0] == subvalues[1] {1} else {0},
        other => panic!("unknown type_id {}", other)
    }
}

fn cursor_test(mut cursor: usize) -> usize {
    println!("cursor {}", cursor);
    cursor += 1;
    println!("cursor {}", cursor);
    return cursor
}

fn sum_versions(packet: Packet) -> usize {
    let subpackets_sum: usize = packet.subpackets.into_iter().map(|p| sum_versions(p)).sum::<usize>();
    return packet.version + subpackets_sum
}


fn decode(transmission: String, mut cursor: usize) -> (Packet, usize) {
    let version: usize = parse_binary_substring(transmission.clone(), cursor, cursor + 3);
    cursor += 3;
    let type_id: usize = parse_binary_substring(transmission.clone(), cursor, cursor + 3);
    cursor += 3;

    // literal value
    if (type_id == 4) {
        let (lit, cursor) = decode_literal(transmission.clone(), cursor);
        return (Packet { version: version, type_id: type_id, value: lit, subpackets: Vec::new()} , cursor);
    }
    else {
        // check length type_id
        let length_type_id: char = char_at(transmission.to_owned(), cursor);
        cursor += 1;
        if length_type_id == '0' {
            let (subpackets, cursor) = decode_subpackets_fixed_bits(transmission.to_owned(), cursor);
            return (Packet { version: version, type_id: type_id, value: 0, subpackets: subpackets}, cursor);
        }
        else {
            let (subpackets, cursor) = decode_subpackets_fixed_num(transmission.to_owned(), cursor);
            return (Packet { version: version, type_id: type_id, value: 0, subpackets: subpackets}, cursor);
        }
    }
}


fn decode_literal(transmission: String, mut cursor: usize) -> (usize, usize) {
    let mut res: String = "".to_string();

    while true {
        let is_last: bool = char_at(transmission.to_owned(), cursor) == '0';
        res.push_str(&transmission[cursor + 1..cursor + 5]);
        cursor += 5;
        if is_last {
            break;
        }
    }

    return (parse_binary(res), cursor);
}

fn decode_subpackets_fixed_bits(transmission: String, mut cursor: usize) -> (Vec<Packet>, usize) {
    let mut subpackets: Vec<Packet> = Vec::new();
    let num_bits: usize = parse_binary_substring(transmission.to_owned(), cursor, cursor + 15);
    cursor += 15;
    let mut bits_consumed: usize = 0;
    while bits_consumed < num_bits {
        let (subpacket, new_cursor) = decode(transmission.to_owned(), cursor);
        subpackets.push(subpacket);
        bits_consumed += (new_cursor - cursor);
        cursor = new_cursor;
    }

    return (subpackets, cursor);
}

fn decode_subpackets_fixed_num(transmission: String, mut cursor: usize) -> (Vec<Packet>, usize) {
    let mut subpackets: Vec<Packet> = Vec::new();
    let num_sub_packets: usize = parse_binary_substring(transmission.to_owned(), cursor, cursor + 11);
    cursor += 11;

    while subpackets.len() < num_sub_packets {
        let (subpacket, new_cursor) = decode(transmission.to_owned(), cursor);
        cursor = new_cursor;
        subpackets.push(subpacket);
    }

    return (subpackets, cursor);
}


fn print_packet(packet: Packet) {
    println!("packet[version: {}, type_id: {}, value: {}]", packet.version, packet.type_id, packet.value);
    println!("subpackets");
    for subpacket in packet.subpackets {
        print_packet(subpacket);
    }
}


// If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
// If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
fn expand_hex(ch: char) -> String {
    match ch {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),

        other => panic!("invalid hex char {}", other)
    }
}



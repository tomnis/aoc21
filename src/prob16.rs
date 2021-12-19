use std::slice::Chunks;
use crate::prob16::Packet::{FixedBitLengthOperator, FixedChildrenOperator, Literal};
use crate::prob16::Tree::Node;
use crate::read_lines;
use crate::util::{char_at, parse_binary_substring};


struct LiteralPacket {
    version: usize,
    // type id should always be 4
    type_id: usize,
    value: usize
}

struct OperatorPacket {
    version: usize,
    type_id: usize // anything != 4
    // indicator bit
}

enum Packet {
    // version, type_id (alwyays 4)
    Literal { version: usize, type_id: usize, value: usize },
    FixedBitLengthOperator { version: usize, type_id: usize, length_type_id: usize, subpacket_length: usize },
    FixedChildrenOperator { version: usize, type_id: usize, length_type_id: usize, num_subpackets: usize }
}

#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node(T, Box<Vec<Tree<T>>>)
}

pub(crate) fn prob16() {
    let lines: Vec<String> = read_lines("./input/prob16.txt");
    println!("{}", part1(lines.clone().into_iter().nth(0).unwrap()));
    // println!("{}", part2(input.clone()));
}


//  what do you get if you add up the version numbers in all packets?
fn part1(s: String) -> usize {
    let bin: String = s.chars().map(|c| expand_hex(c)).collect();
    // println!("{}", bin);
    // println!("{}", parse_literal("110100101111111000101000".to_string()).0);


    let t: Tree<Packet> = parse_packets(bin.clone());
    // Right now, the specific operations aren't important; focus on parsing the hierarchy of sub-packets.
    return sum_versions(t);
}

// sum versions for a tree
fn sum_versions(tree: Tree<Packet>) -> usize {
    match tree {
        Tree::Empty => 0,
        Tree::Node(packet, children) => {
            let v: usize = match packet {
                Literal { version, type_id, value} => version,
                FixedBitLengthOperator { version, type_id, length_type_id, subpacket_length} => version,
                FixedChildrenOperator { version, type_id, length_type_id, num_subpackets } => version,
            };
            let ch: usize = children.into_iter().map(|t| sum_versions(t)).sum();
            v + ch
        }

    }
}

fn parse_packets(s: String) -> Vec<Tree<Packet>> {
    // assume this is some remainder at the end??
    if (s.len() < 11) {
        return vec![Tree::Empty];
    }
    else {
        let v: usize = version(s.clone());
        let tpe: usize = type_id(s.clone());
        // literal
        if (tpe == 4) {
            // TODO need to handle rem, we may have been called recursively from an operator node
            let (lit, mut rem): (usize, String) = parse_literal(s.clone());
            let trees: Vec<Tree<Packet>> = Vec::new();
            return Tree::Node(Literal {version: v, type_id: tpe, value: lit }, Box::new(Vec::new()));
        }
        else {
            // parse an operator packet
            let l_type_id: usize = length_type_id(s.clone());
            if l_type_id == 0 {
                // next 15 bits is total length in bits of subpackets
                let total_subpacket_length: usize = parse_binary_substring(s.clone(), 7, 7 + 15);
                let subpackets_str: String = s.clone()[7+15+1..7+15+1+total_subpacket_length].to_string();

                let packet = FixedBitLengthOperator {version: v, type_id: tpe, length_type_id: l_type_id, subpacket_length: total_subpacket_length };



            }
            else {
                // next 11 bits is number of subpackets
                let num_subpackets: usize = parse_binary_substring(s.clone(), 7, 7 + 11);
                // ???

                let packet = FixedChildrenOperator { version: v, type_id: tpe, length_type_id: l_type_id, num_subpackets: num_subpackets };

            }
        }

        return Tree::Empty;
    }
}


// first three bits encode the packet version, and the next three bits encode the packet type ID
fn version(packet: String) -> usize {
    let first_3: String = packet[0..3].to_string();
    return isize::from_str_radix(&*first_3, 2).unwrap() as usize;
}

fn type_id(packet: String) -> usize {
    let next_3: String = packet[3..6].to_string();
    return isize::from_str_radix(&*next_3, 2).unwrap() as usize;
}

// Every other type of packet (any packet with a type ID other than 4)
// represent an operator that performs some calculation on one or more sub-packets contained within.
fn is_operator_packet(packet: String) -> bool {
    return type_id(packet) != 4;
}

// To indicate which subsequent binary data represents its sub-packets,
// an operator packet can use one of two modes indicated by the bit immediately after the packet header;
// this is called the length type ID:
fn length_type_id(operator_packet: String) -> usize {
    let ch: String =  char_at(operator_packet, 6).to_string();
    return isize::from_str_radix(&*ch, 2).unwrap() as usize;
}

// the binary number is padded with leading zeroes until its length is a multiple of four bits, and then it is broken into groups of four bits.
// Each group is prefixed by a 1 bit except the last group, which is prefixed by a 0 bit. These groups of five bits immediately follow the packet header
// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC
// return (lit, unconsumed packet data?)
fn parse_literal(packet: String) -> (usize, String) {
    println!("packet {}", packet.clone());
    let mut num: String = "".to_string();
    let mut unconsumed: String = "".to_string();

    //drop packet header
    let packet_data: String = packet[6..packet.len()].to_string();
    println!("packet_data {}", packet_data.clone());

    let d: Vec<char> = packet_data.chars().collect();
    let mut chunks: Chunks<char> = d.chunks(5);
    println!("got {} chunks", chunks.len());


    // while &chunks.len() > &0 && chunks.nth(0).unwrap()[0] == '1' {
    //     let chunk: &[char] = chunks.nth(0).unwrap();
    //     let c: String = chunk.clone().to_vec().into_iter().collect();
    //     println!("chunk {}", c);
    //     let num_portion: String = chunk[1..chunk.len()].to_vec().into_iter().collect();
    //     num += &*num_portion;
    //     chunks.next();
    // }
    let mut still_parsing: bool = true;

    for chunk in chunks {
        let c: String = chunk.clone().to_vec().into_iter().collect();
        println!("chunk {}", c);

        let num_portion: String = chunk[1..chunk.len()].to_vec().into_iter().collect();
        if char_at(c.clone(), 0) == '1' && still_parsing {
            num += &*num_portion;
        }
        else if char_at(c.clone(), 0) == '0' && still_parsing {
            num += &*num_portion;
            still_parsing = false;
        }
        else {
            unconsumed += &*c;
        }

    }

    return (isize::from_str_radix(&*num, 2).unwrap() as usize, unconsumed);
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



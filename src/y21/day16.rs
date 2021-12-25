use itertools::Itertools;
use std::fs;

use crate::solution::Solution;
use crate::y21::Day16;

enum Payload {
    Number(u64),
    SubPackets(Vec<Packet>),
}

struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

fn hex_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!(),
    }
}

const HEADER_LEN_LITERAL: usize = 6;
const HEADER_LEN_OPERATOR_0: usize = 22;
const HEADER_LEN_OPERATOR_1: usize = 18;

impl Packet {
    fn parse(binary_repr: &str) -> (Self, usize) {
        let version = u8::from_str_radix(&binary_repr[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&binary_repr[3..6], 2).unwrap();
        if type_id == 4 {
            // literal value packet
            let mut i = 0;
            let mut combined = String::new();
            loop {
                let current_group =
                    &binary_repr[HEADER_LEN_LITERAL + i * 5..HEADER_LEN_LITERAL + (i + 1) * 5];
                i += 1;
                combined.push_str(&current_group[1..]);
                if current_group.starts_with('0') {
                    break;
                }
            }
            let number = u64::from_str_radix(&combined, 2).unwrap();
            let packet = Packet {
                version,
                type_id,
                payload: Payload::Number(number),
            };
            let packet_length = HEADER_LEN_LITERAL + i * 5;
            (packet, packet_length)
        } else {
            // operator packet
            if binary_repr.chars().nth(6).unwrap() == '0' {
                let sub_packets_length =
                    usize::from_str_radix(&binary_repr[7..HEADER_LEN_OPERATOR_0], 2).unwrap();
                let mut parsed_sub_packets_length = 0;
                let mut sub_packets = Vec::new();
                while parsed_sub_packets_length < sub_packets_length {
                    let (packet, packet_length) = Packet::parse(
                        &binary_repr[HEADER_LEN_OPERATOR_0 + parsed_sub_packets_length..],
                    );
                    sub_packets.push(packet);
                    parsed_sub_packets_length += packet_length;
                }
                let packet = Packet {
                    version,
                    type_id,
                    payload: Payload::SubPackets(sub_packets),
                };
                (packet, HEADER_LEN_OPERATOR_0 + parsed_sub_packets_length)
            } else {
                let sub_packets_num =
                    u32::from_str_radix(&binary_repr[7..HEADER_LEN_OPERATOR_1], 2).unwrap();
                let mut parsed_sub_packets_num = 0;
                let mut parsed_sub_packets_length = 0;
                let mut sub_packets = Vec::new();
                while parsed_sub_packets_num < sub_packets_num {
                    let (packet, packet_length) = Packet::parse(
                        &binary_repr[HEADER_LEN_OPERATOR_1 + parsed_sub_packets_length..],
                    );
                    sub_packets.push(packet);
                    parsed_sub_packets_num += 1;
                    parsed_sub_packets_length += packet_length;
                }
                let packet = Packet {
                    version,
                    type_id,
                    payload: Payload::SubPackets(sub_packets),
                };
                (packet, HEADER_LEN_OPERATOR_1 + parsed_sub_packets_length)
            }
        }
    }
}

fn get_version_number_sum(p: &Packet) -> u32 {
    let mut sum = p.version as u32;
    sum += match &p.payload {
        Payload::Number(_) => 0,
        Payload::SubPackets(sub_packets) => sub_packets.iter().map(get_version_number_sum).sum(),
    };
    sum
}

fn evaluate(p: &Packet) -> u64 {
    match &p.payload {
        Payload::Number(number) => {
            assert_eq!(p.type_id, 4);
            *number
        }
        Payload::SubPackets(sub_packets) => match &p.type_id {
            0 => sub_packets.iter().map(evaluate).sum(),
            1 => sub_packets.iter().map(evaluate).product(),
            2 => sub_packets.iter().map(evaluate).min().unwrap(),
            3 => sub_packets.iter().map(evaluate).max().unwrap(),
            5 => {
                assert_eq!(sub_packets.len(), 2);
                if evaluate(&sub_packets[0]) > evaluate(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                assert_eq!(sub_packets.len(), 2);
                if evaluate(&sub_packets[0]) < evaluate(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                assert_eq!(sub_packets.len(), 2);
                if evaluate(&sub_packets[0]) == evaluate(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        },
    }
}

impl Solution for Day16 {
    fn part_1(&self, input_file: &str) -> String {
        let contents = fs::read_to_string(input_file).unwrap();
        let binary_repr = contents.trim().chars().map(hex_to_binary).join("");
        let (packet, _) = Packet::parse(&binary_repr);
        get_version_number_sum(&packet).to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let contents = fs::read_to_string(input_file).unwrap();
        let binary_repr = contents.trim().chars().map(hex_to_binary).join("");
        let (packet, _) = Packet::parse(&binary_repr);
        evaluate(&packet).to_string()
    }
}

use std::iter::once;

pub fn run(input: &str) -> (u64, u64) {
    let mut stream = hexadecimal_to_bit_stream(input);
    let packet = read_packet(&mut stream);
    let part_1 = get_version_number_sum(&packet);
    let part_2 = evaluate_packet(&packet);
    (part_1, part_2)
}

/// Represents a single packet as decoded from the bit stream. A packet has a version
/// number but also has a packet type which contains further information.
#[derive(Debug, PartialEq)]
struct Packet {
    version: u64,
    packet_type: PacketType,
}

/// A packet can either be a literal binary number, or an operator that operates on one or
/// more sub-packets.
#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

/// Determines if we're using the total length in bits method, or packet count method of reading
/// in sub-packets from the stream for a given operator.
#[derive(Debug, PartialEq)]
enum LengthType {
    BitCount,
    PacketCount,
}

/// Struct which identifies the type of length for reading sub-packets (bit count or packet count)
/// and the number.
#[derive(Debug, PartialEq)]
struct SubPacketLength {
    length_type: LengthType,
    count: u64,
}

/// Wraps the bit stream and allows us to count the number of bits read as it's being read.
/// Need to store the bits read as a stack since packets can be nested. When we iterate the next
/// bits we'll update all values in the stack and they can read theirs while they're in their context
/// from the back of the stack.
struct BitStream<T> {
    iter: T,
    bits_read_stack: Vec<u64>,
}

impl<T> BitStream<T> {
    fn new(iter: T) -> Self {
        Self {
            iter,
            bits_read_stack: vec![],
        }
    }
}

impl<T: Iterator<Item = bool>> Iterator for BitStream<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.iter.next();
        if next_value.is_some() {
            self.bits_read_stack.iter_mut().for_each(|elem| {
                *elem += 1;
            });
        }
        next_value
    }
}

/// Evaluates a packet into a u64 answer
fn evaluate_packet(packet: &Packet) -> u64 {
    match &packet.packet_type {
        PacketType::Literal(x) => *x,
        PacketType::Sum(sub) => sub.iter().map(|elem| evaluate_packet(elem)).sum(),
        PacketType::Product(sub) => sub.iter().map(|elem| evaluate_packet(elem)).product(),
        PacketType::Minimum(sub) => sub.iter().map(|elem| evaluate_packet(elem)).min().unwrap(),
        PacketType::Maximum(sub) => sub.iter().map(|elem| evaluate_packet(elem)).max().unwrap(),
        PacketType::GreaterThan(sub) => {
            (evaluate_packet(&sub[0]) > evaluate_packet(&sub[1])) as u64
        }
        PacketType::LessThan(sub) => (evaluate_packet(&sub[0]) < evaluate_packet(&sub[1])) as u64,
        PacketType::EqualTo(sub) => (evaluate_packet(&sub[0]) == evaluate_packet(&sub[1])) as u64,
    }
}

/// Gets the sum of all version numbers in the given packet and sub-packets.
fn get_version_number_sum(packet: &Packet) -> u64 {
    packet.version
        + match &packet.packet_type {
            PacketType::Literal(..) => 0,
            PacketType::Sum(sub)
            | PacketType::Product(sub)
            | PacketType::Minimum(sub)
            | PacketType::Maximum(sub)
            | PacketType::GreaterThan(sub)
            | PacketType::LessThan(sub)
            | PacketType::EqualTo(sub) => sub.iter().map(|sub| get_version_number_sum(sub)).sum(),
        }
}

/// Read a single packet from the stream. For this function we do need to take it as a BitStream (rather than
/// impl Iterator<Item = bool>) because we will access the bits_read when we're reading the sub-packets for operator
/// types.
fn read_packet(stream: &mut BitStream<impl Iterator<Item = bool>>) -> Packet {
    let version = read_binary_number(stream, 3);
    let packet_type = read_packet_type(stream);
    Packet {
        version,
        packet_type,
    }
}

/// Reads the packet type along with the data associated with it. Will either be a literal or some kind of
/// operation.
fn read_packet_type(stream: &mut BitStream<impl Iterator<Item = bool>>) -> PacketType {
    let packet_type_id = read_binary_number(stream, 3);
    match packet_type_id {
        4 => PacketType::Literal(read_literal_packet_value(stream)),
        0 => PacketType::Sum(read_sub_packets(stream)),
        1 => PacketType::Product(read_sub_packets(stream)),
        2 => PacketType::Minimum(read_sub_packets(stream)),
        3 => PacketType::Maximum(read_sub_packets(stream)),
        5 => PacketType::GreaterThan(read_sub_packets(stream)),
        6 => PacketType::LessThan(read_sub_packets(stream)),
        7 => PacketType::EqualTo(read_sub_packets(stream)),
        _ => panic!("Expected valid packet type ID but found {}", packet_type_id),
    }
}

/// Reads a list of sub packets for the Operator packet type to work on.
fn read_sub_packets(stream: &mut BitStream<impl Iterator<Item = bool>>) -> Vec<Packet> {
    let SubPacketLength { length_type, count } = read_sub_packet_length(stream);
    let mut sub_packets = vec![];
    stream.bits_read_stack.push(0);
    while (length_type == LengthType::BitCount && *stream.bits_read_stack.last().unwrap() < count)
        || (length_type == LengthType::PacketCount && sub_packets.len() < count as usize)
    {
        sub_packets.push(read_packet(stream));
    }
    stream.bits_read_stack.pop();
    sub_packets
}

/// Reads the sub packet length structure from the bit stream which can either be a number of bits
/// or a number of packets.
fn read_sub_packet_length(stream: &mut impl Iterator<Item = bool>) -> SubPacketLength {
    let length_type = read_length_type(stream);
    let count = read_binary_number(
        stream,
        if length_type == LengthType::BitCount {
            15
        } else {
            11
        },
    );
    SubPacketLength { length_type, count }
}

/// Reads a single bit from the bit stream and interprets it as a length type for looking up
/// sub packets.
fn read_length_type(stream: &mut impl Iterator<Item = bool>) -> LengthType {
    if stream.next().unwrap() {
        LengthType::PacketCount
    } else {
        LengthType::BitCount
    }
}

/// Reads from the bit stream and interprets as the value of the literal packet type. This
/// requires reading blocks of 4 bits at a time until it's been terminated and concatenating
/// the resulting chunks together
fn read_literal_packet_value(stream: &mut impl Iterator<Item = bool>) -> u64 {
    let mut total = 0;
    loop {
        let group_bit = stream.next().unwrap();
        let group = read_binary_number(stream, 4);
        total = (total << 4) + group;
        if !group_bit {
            break;
        }
    }
    total
}

/// Takes a hexidecimal input string and returns an iterator over booleans (bits).
/// This can then be parsed into packets as needed.
fn hexadecimal_to_bit_stream(input: &str) -> BitStream<impl Iterator<Item = bool> + '_> {
    BitStream::new(input.trim().chars().flat_map(|c| {
        let value = match c {
            '0' => 0b0000,
            '1' => 0b0001,
            '2' => 0b0010,
            '3' => 0b0011,
            '4' => 0b0100,
            '5' => 0b0101,
            '6' => 0b0110,
            '7' => 0b0111,
            '8' => 0b1000,
            '9' => 0b1001,
            'A' => 0b1010,
            'B' => 0b1011,
            'C' => 0b1100,
            'D' => 0b1101,
            'E' => 0b1110,
            'F' => 0b1111,
            _ => panic!(
                "Invalid input detected, expected hexadecimal digit, found {}",
                c
            ),
        };
        once(value & 0b1000 == 0b1000)
            .chain(once(value & 0b0100 == 0b0100))
            .chain(once(value & 0b0010 == 0b0010))
            .chain(once(value & 0b0001 == 0b0001))
    }))
}

/// Takes the number of bits to read from a given iterator of booleans, reads that
/// many and converts to a decimal number.
fn read_binary_number(stream: &mut impl Iterator<Item = bool>, bit_count: usize) -> u64 {
    (0..bit_count)
        .into_iter()
        .zip(stream)
        .fold(0, |total, (_, bit)| (total << 1) + bit as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal_to_bit_stream_success() {
        const INPUT: &str = "A1F9";
        const EXPECTED: &[bool] = &[
            true, false, true, false, false, false, false, true, true, true, true, true, true,
            false, false, true,
        ];
        assert_eq!(
            &hexadecimal_to_bit_stream(INPUT).collect::<Vec<_>>(),
            EXPECTED
        );
    }

    #[test]
    #[should_panic]
    fn test_hexadecimal_to_bit_stream_invalid() {
        let _ = hexadecimal_to_bit_stream("AIF9").collect::<Vec<_>>();
    }

    #[test]
    fn test_read_binary_number() {
        const EXPECTED: u64 = 27;
        let mut stream = BitStream::new([true, true, false, true, true].into_iter());
        let calculated = read_binary_number(&mut stream, 5);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_read_multiple_binary_numbers() {
        const EXPECTED_1: u64 = 27;
        const EXPECTED_2: u64 = 19;
        let mut stream = BitStream::new(
            [
                true, true, false, true, true, true, false, false, true, true,
            ]
            .into_iter(),
        );
        assert_eq!(read_binary_number(&mut stream, 5), EXPECTED_1);
        assert_eq!(read_binary_number(&mut stream, 5), EXPECTED_2);
        assert!(stream.next().is_none());
    }

    #[test]
    fn test_read_literal_packet_value() {
        const EXPECTED: u64 = 2021;
        let mut stream = BitStream::new(
            [
                true, false, true, true, true, true, true, true, true, false, false, false, true,
                false, true, false, false, false,
            ]
            .into_iter(),
        );
        let calculated = read_literal_packet_value(&mut stream);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_read_length_type() {
        const EXPECTED: &[LengthType] = &[
            LengthType::BitCount,
            LengthType::PacketCount,
            LengthType::PacketCount,
            LengthType::BitCount,
            LengthType::BitCount,
        ];
        let mut stream = BitStream::new([false, true, true, false, false].into_iter());
        let calculated = &(0..=4)
            .map(|_| read_length_type(&mut stream))
            .collect::<Vec<_>>();
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_read_sub_packet_length() {
        let mut stream = BitStream::new(
            [
                false, false, false, false, false, false, false, false, false, false, false, true,
                true, false, true, true, true, false, false, false, false, false, false, false,
                false, false, true, true,
            ]
            .into_iter(),
        );
        assert_eq!(
            read_sub_packet_length(&mut stream),
            SubPacketLength {
                length_type: LengthType::BitCount,
                count: 27
            }
        );
        assert_eq!(
            read_sub_packet_length(&mut stream),
            SubPacketLength {
                length_type: LengthType::PacketCount,
                count: 3
            }
        );
    }

    #[test]
    fn test_bit_stream_bit_count() {
        let mut stream = BitStream::new([true, false, true, false, false, true, true].into_iter());
        stream.bits_read_stack.push(0);
        read_binary_number(&mut stream, 4);
        assert_eq!(*stream.bits_read_stack.last().unwrap(), 4);
        read_binary_number(&mut stream, 2);
        assert_eq!(*stream.bits_read_stack.last().unwrap(), 6);
    }

    #[test]
    fn test_read_packet_literal() {
        let mut stream = hexadecimal_to_bit_stream("D2FE28");
        const EXPECTED: Packet = Packet {
            version: 6,
            packet_type: PacketType::Literal(2021),
        };
        let calculated = read_packet(&mut stream);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_read_packet_operator() {
        let mut stream = hexadecimal_to_bit_stream("38006F45291200");
        let expected = Packet {
            version: 1,
            packet_type: PacketType::LessThan(vec![
                Packet {
                    version: 6,
                    packet_type: PacketType::Literal(10),
                },
                Packet {
                    version: 2,
                    packet_type: PacketType::Literal(20),
                },
            ]),
        };
        let calculated = read_packet(&mut stream);
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_read_packet_another_operator() {
        let mut stream = hexadecimal_to_bit_stream("EE00D40C823060");
        let expected = Packet {
            version: 7,
            packet_type: PacketType::Maximum(vec![
                Packet {
                    version: 2,
                    packet_type: PacketType::Literal(1),
                },
                Packet {
                    version: 4,
                    packet_type: PacketType::Literal(2),
                },
                Packet {
                    version: 1,
                    packet_type: PacketType::Literal(3),
                },
            ]),
        };
        let calculated = read_packet(&mut stream);
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_get_version_number_sum() {
        let mut stream = hexadecimal_to_bit_stream("EE00D40C823060");
        const EXPECTED: u64 = 14;
        let packet = read_packet(&mut stream);
        let calculated = get_version_number_sum(&packet);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_get_another_version_number_sum() {
        let mut stream = hexadecimal_to_bit_stream("8A004A801A8002F478");
        const EXPECTED: u64 = 16;
        let packet = read_packet(&mut stream);
        let calculated = get_version_number_sum(&packet);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_get_yet_another_version_number_sum() {
        let mut stream = hexadecimal_to_bit_stream("A0016C880162017C3686B18A3D4780");
        const EXPECTED: u64 = 31;
        let packet = read_packet(&mut stream);
        let calculated = get_version_number_sum(&packet);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_evaluate_packet() {
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream("C200B40A82"))),
            3
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream("04005AC33890"))),
            54
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream(
                "880086C3E88112"
            ))),
            7
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream(
                "CE00C43D881120"
            ))),
            9
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream("D8005AC2A8F0"))),
            1
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream("F600BC2D8F"))),
            0
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream("9C005AC2F8F0"))),
            0
        );
        assert_eq!(
            evaluate_packet(&read_packet(&mut hexadecimal_to_bit_stream(
                "9C0141080250320F1802104A08"
            ))),
            1
        );
    }
}

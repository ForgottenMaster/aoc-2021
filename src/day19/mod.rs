mod rotations;
mod sensor;
mod sensor_link;
mod sensor_object;

use crate::common::{iter::FilterGroupMapExt, math::Matrix};
use sensor::Sensor;
use sensor_link::SensorLink;
use sensor_object::SensorObject;
use std::collections::{HashMap, HashSet};
use std::iter::once;

pub fn run(input: &str) -> (usize, i64) {
    let sensors = parse_into_sensors(input);
    let processed = vec![&sensors[0]];
    let unprocessed = sensors.iter().skip(1).collect();
    let links = get_sensor_links(processed, unprocessed);

    // Start off with a HashMap keyed by sensor and their unrotated
    // positions for each sensor. Also add in the sensor position itself
    // which is at (0, 0, 0).
    let mut hm = sensors
        .iter()
        .map(|sensor| {
            (
                sensor,
                sensor.points[0]
                    .1
                    .iter()
                    .map(|position| SensorObject::Beacon(*position))
                    .chain(once(SensorObject::Sensor((0, 0, 0))))
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    // Go through the links in REVERSE order and take the positions from the
    // source sensor, apply the given rotation and translation to place them in
    // the space of the destination sensor. Add these to the destination sensors entry.
    // We go back in reverse order because we will end up eventually having everything in
    // the space of Sensor 0.
    links.into_iter().rev().for_each(|link| {
        let transformed = hm[link.source]
            .iter()
            .map(|obj| transform_sensor_object(obj, link.rotation, &link.translation))
            .collect::<Vec<_>>();
        hm.get_mut(link.destination)
            .unwrap()
            .extend(transformed.into_iter());
    });

    // Entire map should have consolidated into sensor 0's list.
    let map = &hm[&sensors[0]];

    // Part 1 is the number of unique beacons. Since we're using HashSet though these beacons will be unique.
    let part_1 = map
        .into_iter()
        .filter(|obj| matches!(obj, SensorObject::Beacon(_)))
        .count();

    // Part 2 will need to test each sensor against each other to find the largest manhattan distance.
    let part_2 = map
        .into_iter()
        .filter(|obj| matches!(obj, SensorObject::Sensor(..)))
        .enumerate()
        .flat_map(|(idx, obj_1)| {
            map.into_iter()
                .filter(|obj| matches!(obj, SensorObject::Sensor(..)))
                .enumerate()
                .skip(idx + 1)
                .map(move |(_, obj_2)| match (obj_1, obj_2) {
                    (SensorObject::Sensor(pos_1), SensorObject::Sensor(pos_2)) => {
                        manhattan_distance(pos_1, pos_2)
                    }
                    _ => unreachable!(),
                })
        })
        .max()
        .unwrap();

    (part_1, part_2)
}

/// Calculates the manhattan distance between two points.
fn manhattan_distance(p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

/// Takes a SensorObject, and a reference to a rotation and translation that can be applied
/// to take it to another sensor's space, and produces a SensorObject that's transformed.
fn transform_sensor_object(
    obj: &SensorObject,
    rotation: &Matrix<i64, 3, 3>,
    translation: &(i64, i64, i64),
) -> SensorObject {
    let pos = match obj {
        SensorObject::Sensor(pos) | SensorObject::Beacon(pos) => pos,
    };
    let pos = rotation * pos;
    let pos = (
        pos.0 + translation.0,
        pos.1 + translation.1,
        pos.2 + translation.2,
    );
    match obj {
        SensorObject::Sensor(_) => SensorObject::Sensor(pos),
        SensorObject::Beacon(_) => SensorObject::Beacon(pos),
    }
}

/// Takes the given &str representing the whole input and parses it into
/// the list of sensors, containing the visible points in all the bases.
fn parse_into_sensors(input: &str) -> Vec<Sensor> {
    input
        .trim()
        .lines()
        .filter_group_map(
            |elem| !elem.trim().is_empty(),
            |lines| {
                lines
                    .into_iter()
                    .map(|elem| *elem)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .parse::<Sensor>()
                    .unwrap()
            },
        )
        .collect::<Vec<_>>()
}

/// Gets links between sensors, enough to be able to trace each one back to
/// sensor #0.
fn get_sensor_links<'a>(
    mut processed: Vec<&'a Sensor<'a>>,
    mut unprocessed: Vec<&'a Sensor<'a>>,
) -> Vec<SensorLink<'a>> {
    let mut links = Vec::with_capacity(unprocessed.len());
    while !unprocessed.is_empty() {
        let (link, idx) = unprocessed
            .iter()
            .enumerate()
            .filter_map(|(idx, sensor)| Some((get_single_sensor_link(&processed, sensor)?, idx)))
            .next()
            .unwrap();
        links.push(link);
        let removed = unprocessed.remove(idx);
        processed.push(removed);
    }
    links
}

/// Gets a single link between the given sensor and one of the sensors in the previously processed array.
fn get_single_sensor_link<'a>(
    processed: &[&'a Sensor],
    source: &'a Sensor,
) -> Option<SensorLink<'a>> {
    processed
        .into_iter()
        .filter_map(|destination| {
            if do_distances_overlap_enough(source, destination) {
                find_correct_rotation_and_translation(source, destination)
            } else {
                None
            }
        })
        .next()
}

/// Finds the correct rotation and translation if there exists one to map between source and destination
/// Sensors. Requires an additional translation after rotation to try to align the points such that there
/// are a minimum of 12 shared after translation.
fn find_correct_rotation_and_translation<'a>(
    source: &'a Sensor,
    destination: &'a Sensor,
) -> Option<SensorLink<'a>> {
    (&source.points)
        .into_iter()
        .filter_map(|(rotation, points)| {
            if let Some(translation) = calculate_translation(&points, &destination.points[0].1) {
                Some(SensorLink {
                    source,
                    destination,
                    rotation,
                    translation,
                })
            } else {
                None
            }
        })
        .next()
}

/// Takes two sets of points and sees if there's a translation which would take at least 12 points from
/// source to destination.
fn calculate_translation(
    source: &[(i64, i64, i64)],
    destination: &[(i64, i64, i64)],
) -> Option<(i64, i64, i64)> {
    source
        .into_iter()
        .flat_map(|source_position| {
            destination.into_iter().filter_map(|destination_position| {
                let translation = (
                    destination_position.0 - source_position.0,
                    destination_position.1 - source_position.1,
                    destination_position.2 - source_position.2,
                );
                if test_apply_translation(source, destination, &translation) {
                    Some(translation)
                } else {
                    None
                }
            })
        })
        .next()
}

/// Applies the given translation to points in source and tests if enough are present in
/// destination (12).
fn test_apply_translation(
    source: &[(i64, i64, i64)],
    destination: &[(i64, i64, i64)],
    translation: &(i64, i64, i64),
) -> bool {
    source
        .into_iter()
        .filter(|position| {
            let position = (
                position.0 + translation.0,
                position.1 + translation.1,
                position.2 + translation.2,
            );
            destination.into_iter().any(|point| position == *point)
        })
        .take(12)
        .count()
        == 12
}

/// Checks whether the two Sensor references actually potentially overlap based on their distances
/// between any two vertices. Since distances remain the same with rotation, and since we're looking for
/// 12 intersecting points, the number of distances in common will be at least 66 (12 choose 2).
fn do_distances_overlap_enough(source: &Sensor, destination: &Sensor) -> bool {
    source
        .distances
        .iter()
        .filter(|(_, _, source_dist)| {
            destination
                .distances
                .iter()
                .any(|(_, _, destination_dist)| source_dist == destination_dist)
        })
        .take(66)
        .count()
        == 66
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_distances_overlap_source() {
        let sensor_0 = r#"
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401
        "#
        .parse::<Sensor>()
        .unwrap();
        let sensor_1 = r#"
        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390
        "#
        .parse::<Sensor>()
        .unwrap();
        let sensor_4 = r#"
        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
        "#
        .parse::<Sensor>()
        .unwrap();

        assert!(do_distances_overlap_enough(&sensor_0, &sensor_1));
        assert!(!do_distances_overlap_enough(&sensor_0, &sensor_4));
        assert!(do_distances_overlap_enough(&sensor_1, &sensor_4));
    }

    #[test]
    fn test_run() {
        const INPUT: &str = r#"
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
        "#;
        const EXPECTED: (usize, i64) = (79, 3621);
        assert_eq!(run(INPUT), EXPECTED);
    }
}

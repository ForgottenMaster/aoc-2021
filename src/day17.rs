pub fn run(input: &str) -> (u32, usize) {
    let aabb = extract_aabb(input);
    let y_velocity = calculate_starting_y_velocity_required_for_highest_peak(aabb.bottom_left.1);
    let part_1 = calculate_triangular_number(y_velocity as u32);
    let velocity_space = calculate_velocity_space(&aabb);
    let part_2 = generate_velocities(velocity_space.clone())
        .filter(|velocity| {
            generate_positions_from_velocity(*velocity)
                .take_while(|position| {
                    position.0 <= velocity_space.top_right.0
                        && position.1 >= velocity_space.bottom_left.1
                })
                .filter(|position| is_position_inside_aabb(*position, &aabb))
                .next()
                .is_some()
        })
        .count();
    (part_1, part_2)
}

/// Defines an axis-aligned bounding box which is the area
/// we're trying to land the probe inside of.
#[derive(Clone, Debug, PartialEq)]
struct AABB {
    bottom_left: (i32, i32),
    top_right: (i32, i32),
}

/// Generates all positions that a given velocity will have.
/// Note that this generates an infinite sequence so it's upto calling code to
/// clamp this.
fn generate_positions_from_velocity(velocity: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    (0..).scan((velocity, (0, 0)), |velocity_and_position, _| {
        let (velocity, position) = (&mut velocity_and_position.0, &mut velocity_and_position.1);
        position.0 += velocity.0;
        position.1 += velocity.1;
        if velocity.0 > 0 {
            velocity.0 -= 1;
        } else if velocity.0 < 0 {
            velocity.0 += 1;
        }
        velocity.1 -= 1;
        Some(*position)
    })
}

/// Tests whether a position is inside the given AABB or not.
fn is_position_inside_aabb(position: (i32, i32), aabb: &AABB) -> bool {
    position.0 >= aabb.bottom_left.0
        && position.0 <= aabb.top_right.0
        && position.1 >= aabb.bottom_left.1
        && position.1 <= aabb.top_right.1
}

/// Generates all velocities in the given velocity space.
fn generate_velocities(velocity_space: AABB) -> impl Iterator<Item = (i32, i32)> {
    (velocity_space.bottom_left.1..=velocity_space.top_right.1).flat_map(move |y| {
        (velocity_space.bottom_left.0..=velocity_space.top_right.0).map(move |x| (x, y))
    })
}

/// Gets the AABB that contains the velocities that we'll check.
/// We'll just brute force it by testing every velocity in the velocity space.
/// The velocity space being all velocities that won't overshoot the thing in a step.
/// For x, we'll just take all velocities (0..=trench_x_max)
/// For y, we'll take all velocities in the range (trench_y_min..=-(trench_y_min+1))
fn calculate_velocity_space(trench: &AABB) -> AABB {
    // we don't start x at 0, but instead constrain to x >= n where n is the
    // highest triangular number that's less than the trench edge.
    let x_min = (1..)
        .take_while(|i| calculate_triangular_number(*i) as i32 <= trench.bottom_left.0)
        .last()
        .unwrap() as i32;

    AABB {
        bottom_left: (x_min, trench.bottom_left.1),
        top_right: (trench.top_right.0, -(trench.bottom_left.1 + 1)),
    }
}

/// Extracts the min and max coordinates of the AABB we're landing in, given the input string.
fn extract_aabb(input: &str) -> AABB {
    let mut input = input
        .trim()
        .split("x=")
        .skip(1)
        .next()
        .unwrap()
        .split(", y=");
    let (x_min, x_max) = parse_range(input.next().unwrap());
    let (y_min, y_max) = parse_range(input.next().unwrap());
    AABB {
        bottom_left: (x_min, y_min),
        top_right: (x_max, y_max),
    }
}

/// Takes a string in the form x..y and parses it as a range, returning the minimum and
/// maximum value.
fn parse_range(range_str: &str) -> (i32, i32) {
    let mut iter = range_str
        .split("..")
        .map(|value| value.parse::<i32>().unwrap());
    let (v_0, v_1) = (iter.next().unwrap(), iter.next().unwrap());
    if v_0 < v_1 {
        (v_0, v_1)
    } else {
        (v_1, v_0)
    }
}

/// We know that the probe will go up and back down, reaching 0 again when velocity is at
/// -(initial+1). The next step it will make this movement, so we need to make sure that this
/// will get it to the lowest point of the trench (from y=0) in one step and not overshoot.
/// This velocity is equal to the lowest_y value. However because of the effect of gravity
/// and the order that the steps are computed in, the initial velocity will want to be 1 closer
/// to 0. Therefore the starting velocity will be -(lowest_y + 1). lowest_y is always negative as it's
/// a trench.
fn calculate_starting_y_velocity_required_for_highest_peak(lowest_y: i32) -> i32 {
    -(lowest_y + 1)
}

/// Calculates the nth triangular number which is given by the formal (n*(n+1))/2.
/// The maximum y value reached by the starting velocity is given as the nth triangle number
/// since to get to max y we do, for initial velocity 5 for example, 5 + 4 + 3 + 2 + 1 = 15.
/// 15 is the 5th triangular number.
fn calculate_triangular_number(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_starting_y_velocity_required_for_highest_peak() {
        const LOWEST_Y: i32 = -10;
        let calculated = calculate_starting_y_velocity_required_for_highest_peak(LOWEST_Y);
        const EXPECTED: i32 = 9;
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_calculate_triangular_number() {
        assert_eq!(calculate_triangular_number(1), 1);
        assert_eq!(calculate_triangular_number(2), 3);
        assert_eq!(calculate_triangular_number(3), 6);
        assert_eq!(calculate_triangular_number(4), 10);
    }

    #[test]
    fn test_extract_aabb() {
        assert_eq!(
            extract_aabb("target area: x=20..30, y=-10..-5"),
            AABB {
                bottom_left: (20, -10),
                top_right: (30, -5)
            }
        );
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("20..42"), (20, 42));
        assert_eq!(parse_range("42..20"), (20, 42));
        assert_eq!(parse_range("-12..-56"), (-56, -12));
        assert_eq!(parse_range("-56..-12"), (-56, -12));
        assert_eq!(parse_range("42..-42"), (-42, 42));
        assert_eq!(parse_range("-42..42"), (-42, 42));
    }

    #[test]
    fn test_calculate_velocity_space() {
        let trench = AABB {
            bottom_left: (22, -30),
            top_right: (55, -10),
        };
        let velocity_space = calculate_velocity_space(&trench);
        const EXPECTED: AABB = AABB {
            bottom_left: (6, -30),
            top_right: (55, 29),
        };
        assert_eq!(velocity_space, EXPECTED);
    }

    #[test]
    fn test_generate_velocities() {
        const VELOCITY_SPACE: AABB = AABB {
            bottom_left: (0, -3),
            top_right: (3, 3),
        };
        let expected = vec![
            (0, -3),
            (1, -3),
            (2, -3),
            (3, -3),
            (0, -2),
            (1, -2),
            (2, -2),
            (3, -2),
            (0, -1),
            (1, -1),
            (2, -1),
            (3, -1),
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
        ];
        assert_eq!(
            generate_velocities(VELOCITY_SPACE).collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_generate_positions_from_velocity() {
        let velocity = (3, 6);
        let calculated = generate_positions_from_velocity(velocity)
            .take(10)
            .collect::<Vec<_>>();
        let expected = vec![
            (3, 6),
            (5, 11),
            (6, 15),
            (6, 18),
            (6, 20),
            (6, 21),
            (6, 21),
            (6, 20),
            (6, 18),
            (6, 15),
        ];
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_is_position_inside_aabb() {
        const BOUNDS: AABB = AABB {
            bottom_left: (10, -20),
            top_right: (30, -5),
        };
        assert!(is_position_inside_aabb((13, -16), &BOUNDS));
        assert!(!is_position_inside_aabb((7, -16), &BOUNDS));
        assert!(!is_position_inside_aabb((13, -3), &BOUNDS));
    }

    #[test]
    fn test_run_example() {
        const INPUT: &str = "target area: x=20..30, y=-10..-5";
        const EXPECTED: (u32, usize) = (45, 112);
        assert_eq!(run(INPUT), EXPECTED);
    }
}

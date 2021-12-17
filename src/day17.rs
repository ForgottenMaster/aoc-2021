pub fn run(input: &str) -> (u32, u32) {
    let aabb = extract_aabb(input);
    let y_velocity = calculate_starting_y_velocity_required_for_highest_peak(aabb.bottom_left.1);
    let part_1 = calculate_triangular_number(y_velocity as u32);
    (part_1, 0)
}

/// Defines an axis-aligned bounding box which is the area
/// we're trying to land the probe inside of.
#[derive(Debug, PartialEq)]
struct AABB {
    bottom_left: (i32, i32),
    top_right: (i32, i32),
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
}

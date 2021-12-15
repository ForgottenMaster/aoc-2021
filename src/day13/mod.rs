mod paper;

use paper::PaperBuilder;

pub fn run(input: &str) -> (usize, String) {
    // Build the transparent sheet of paper and place the dots.
    let mut builder = PaperBuilder::default();
    let mut lines = input.trim().lines();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            break; // all coordinates of dots provided.
        }

        let mut splits = line.split(",");
        let x = splits.next().unwrap().parse::<usize>().unwrap();
        let y = splits.next().unwrap().parse::<usize>().unwrap();
        builder.place_dot(x, y);
    }

    // Prepare the iterator that will do the folding as we iterate over it.
    let mut paper = builder.build().unwrap();
    let mut fold_ops = lines.map(|line| {
        let line = line.trim();
        let fold_component = line.split(" ").skip(2).next().unwrap().trim(); // skips "fold along" text
        let mut splits = fold_component.split("=");
        let axis = splits.next().unwrap();
        let coordinate = splits.next().unwrap().parse::<usize>().unwrap();
        (axis, coordinate)
    });

    // Part 1 is how many dots are present after only a single fold.
    let part_1 = {
        let (axis, coordinate) = fold_ops.next().unwrap();
        if axis == "x" {
            paper.fold_horizontally(coordinate);
        } else if axis == "y" {
            paper.fold_vertically(coordinate);
        }
        paper.count_dots()
    };

    // Finish applying the fold instructions.
    fold_ops.for_each(|(axis, coordinate)| {
        if axis == "x" {
            paper.fold_horizontally(coordinate);
        } else if axis == "y" {
            paper.fold_vertically(coordinate);
        }
    });

    // Part 2 will be the displayed grid, we can use the Display implementation of Paper here.
    let part_2 = format!("\n{}", paper);

    (part_1, part_2)
}

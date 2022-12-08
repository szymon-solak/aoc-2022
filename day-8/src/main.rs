fn get_grid_cells(width: usize, height: usize) -> Vec<(usize, usize)> {
    let xs = 1..width - 1;
    let ys = 1..height - 1;
    let inside_range = xs.flat_map(|x| ys.clone().map(move |y| (x, y)));

    inside_range.collect()
}

fn count_visible_trees(grid: Vec<Vec<usize>>) -> usize {
    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let trees_visible_outside = (width - 1) * 2 + (height - 1) * 2;
    let trees_visible_inside = get_grid_cells(width, height)
        .into_iter()
        .filter(|&(x, y)| {
            let tree_size = grid.get(y).unwrap().get(x).unwrap();
            let left = grid.get(y).unwrap().get(0..x).unwrap().iter().max().unwrap() < tree_size;
            let right = grid.get(y).unwrap().get(x + 1..width).unwrap().iter().max().unwrap() < tree_size;
            let up = grid.get(0..y).unwrap().iter().map(|cy| cy.get(x).unwrap()).max().unwrap() < tree_size;
            let down = grid.get(y + 1..height).unwrap().iter().map(|cy| cy.get(x).unwrap()).max().unwrap() < tree_size;

            left || right || up || down
        })
        .count();

    trees_visible_outside + trees_visible_inside
}

fn take_until<T, P>(vec: Vec<T>, predicate: P) -> Vec<T>
    where P: FnMut(&T) -> bool
{
    let mut iter = vec.into_iter();
    let mut till = iter.by_ref().take_while(predicate).collect::<Vec<T>>();

    if let Some(item) = iter.next() {
        till.push(item);
    }

    till
}

fn count_scenic_scores(grid: Vec<Vec<usize>>) -> Vec<usize> {
    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    get_grid_cells(width, height)
        .into_iter()
        .map(|(x, y)| {
            let tree_size = grid.get(y).unwrap().get(x).unwrap();
            let left = grid.get(y).unwrap().get(0..x).unwrap().iter().rev();
            let right = grid.get(y).unwrap().get(x + 1..width).unwrap();
            let up = grid.get(0..y).unwrap().iter().map(|cy| cy.get(x).unwrap()).rev();
            let down = grid.get(y + 1..height).unwrap().iter().map(|cy| cy.get(x).unwrap());

            let left_score = take_until(left.collect(), |&t| { t < tree_size }).len();
            let right_score = take_until(right.to_vec(), |t| { t < tree_size }).len();
            let up_score = take_until(up.collect(), |&t| { t < tree_size }).len();
            let down_score = take_until(down.collect(), |&t| { t < tree_size }).len();

            left_score * right_score * up_score * down_score
        })
        .collect::<Vec<usize>>()
}

fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    println!("part 1: {}", count_visible_trees(grid.clone()));
    println!("part 2: {:?}", count_scenic_scores(grid.clone()).iter().max());
}

use std::collections::{HashMap, VecDeque};

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let rows = maze.len();
    let cols = maze[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut queue = VecDeque::new();

    visited[start.0][start.1] = true;
    queue.push_back(start);

    let mut found = false;

    while let Some(current) = queue.pop_front() {
        if current == end {
            found = true;
            break;
        }

        let (r, c) = current;
        let neighbors = [
            (r.checked_sub(1), Some(c)),
            (Some(r), c.checked_sub(1)),
            (Some(r), c.checked_add(1)),
            (r.checked_add(1), Some(c)),
        ];

        for (nr, nc) in neighbors {
            let (Some(nr), Some(nc)) = (nr, nc) else {
                continue;
            };
            if nr >= rows || nc >= cols {
                continue;
            }
            if maze[nr][nc] == '#' {
                continue;
            }
            if visited[nr][nc] {
                continue;
            }
            visited[nr][nc] = true;
            prev.insert((nr, nc), current);
            queue.push_back((nr, nc));
        }
    }

    if !found {
        return vec![];
    }

    let mut path = vec![end];
    let mut cursor = end;

    while cursor != start {
        cursor = prev[&cursor];
        path.push(cursor);
    }

    path.reverse();
    path
}

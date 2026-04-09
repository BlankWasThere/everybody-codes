use std::str::FromStr;

macro_rules! split_to_string {
    ($s:expr, $delimeter:expr) => {
        $s.split_once($delimeter)
            .map(|(left, right)| (left.to_string(), right.to_string()))
    };
}

#[derive(Debug, Clone)]
struct Node {
    id: u32,
    plug: (String, String),
    left_socket: (String, String),
    right_socket: (String, String),
    _data: String,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

impl Node {
    fn insert(&mut self, node: &Self) -> bool {
        fn tuple_shortcircuit_check(a: &(String, String), b: &(String, String)) -> bool {
            a.0 == b.0 || a.1 == b.1
        }

        if let Some(left) = &mut self.left_node {
            if left.insert(node) {
                return true;
            }
        } else if tuple_shortcircuit_check(&self.left_socket, &node.plug) {
            self.left_node = Some(Box::new(node.clone()));
            return true;
        }

        if let Some(right) = &mut self.right_node {
            if right.insert(node) {
                return true;
            }
        } else if tuple_shortcircuit_check(&self.right_socket, &node.plug) {
            self.right_node = Some(Box::new(node.clone()));
            return true;
        }

        false
    }

    fn traverse(&self) -> Vec<&Node> {
        let left_iter = self.left_node.iter().flat_map(|node| node.traverse());
        let right_iter = self.right_node.iter().flat_map(|node| node.traverse());

        left_iter.chain([self]).chain(right_iter).collect()
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = s
            .split(',')
            .map(|s| s.split_once('=').ok_or("should contain an =").map(|e| e.1))
            .collect::<Result<Vec<_>, _>>()?;

        let [id, plug, left_socket, right_socket, data] = <[&str; 5]>::try_from(node)
            .map_err(|_| "invalid node")?
            .map(|s| s.to_string());

        let id = id.parse().map_err(|_| "should be a number")?;
        let plug = split_to_string!(plug, ' ').ok_or("should be in the format: `COLOR SHAPE`")?;
        let left_socket =
            split_to_string!(left_socket, ' ').ok_or("should be in the format: `COLOR SHAPE`")?;
        let right_socket =
            split_to_string!(right_socket, ' ').ok_or("should be in the format: `COLOR SHAPE`")?;

        Ok(Node {
            id,
            plug,
            left_socket,
            right_socket,
            _data: data,
            left_node: None,
            right_node: None,
        })
    }
}

pub fn solve(input: &str) -> u32 {
    let mut nodes = parse_input(input);
    assert!(!nodes.is_empty());

    let mut root = nodes.remove(0);

    for node in nodes {
        if !root.insert(&node) {
            panic!("unable to insert {node:?}");
        }
    }

    root.traverse()
        .into_iter()
        .enumerate()
        .map(|(idx, node)| (idx + 1) as u32 * node.id)
        .sum()
}

fn parse_input(input: &str) -> Vec<Node> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|node| node.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 50;
        let actual = solve(
            "\
id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?",
        );

        assert_eq!(actual, expected);
    }
}

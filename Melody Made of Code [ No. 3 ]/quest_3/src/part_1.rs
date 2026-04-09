use std::str::FromStr;

#[derive(Debug, Clone)]
struct Node {
    id: u32,
    plug: String,
    left_socket: String,
    right_socket: String,
    _data: String,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

impl Node {
    fn insert(&mut self, node: &Self) -> bool {
        if let Some(left) = &mut self.left_node {
            if left.insert(node) {
                return true;
            }
        } else if self.left_socket == node.plug {
            self.left_node = Some(Box::new(node.clone()));
            return true;
        }

        if let Some(right) = &mut self.right_node {
            if right.insert(node) {
                return true;
            }
        } else if self.right_socket == node.plug {
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

        Ok(Node {
            id: id.parse().expect("should be a number"),
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
        let expected = 43;
        let actual = solve(
            "\
id=1, plug=BLUE HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=BLUE PENTAGON, data=?
id=2, plug=GREEN CIRCLE, leftSocket=BLUE HEXAGON, rightSocket=BLUE CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=BLUE CIRCLE, data=?
id=4, plug=BLUE CIRCLE, leftSocket=RED HEXAGON, rightSocket=BLUE HEXAGON, data=?
id=5, plug=RED HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=RED HEXAGON, data=?",
        );

        assert_eq!(actual, expected);
    }
}

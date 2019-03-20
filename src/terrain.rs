use ::{v2, v3};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Node {
    width: f32,
    height: f32,
    elevation: f32,
}

impl Node {

    pub fn point(elevation: f32) -> Node {
        Node{elevation, width: 0.0, height: 0.0}
    }

    pub fn new(width: f32, height: f32, elevation: f32) -> Node {
        Node{width, height, elevation}
    }
}

struct Terrain {
    nodes: na::DMatrix<Node>,
    grid: na::DMatrix<na::Vector3<f32>>,
}

impl Terrain {

    fn new(nodes: na::DMatrix<Node>) -> Terrain {
        Terrain{
            grid: Terrain::create_grid(&nodes),
            nodes,
        }
    }

    fn create_grid(nodes: &na::DMatrix<Node>) -> na::DMatrix<na::Vector3<f32>> {
        let width = nodes.shape().0;
        let height = nodes.shape().0;
        let mut grid = na::DMatrix::from_element(width * 2, height * 2, v3(0.0, 0.0, 0.0));

        for x in 0..width {
            for y in 0..height {
                let node = nodes[(x, y)];
                let xf = x as f32;
                let yf = y as f32;
                let x2 = x * 2;
                let y2 = y * 2;
                grid[(x2, y2)] = v3(xf - node.width, yf - node.height, node.elevation);
                grid[(x2 + 1, y2)] = v3(xf + node.width, yf - node.height, node.elevation);
                grid[(x2, y2 + 1)] = v3(xf - node.width, yf + node.height, node.elevation);
                grid[(x2 + 1, y2 + 1)] = v3(xf + node.width, yf + node.height, node.elevation);
            }
        }

        grid
    }

    fn get_border(&self, grid_index: na::Vector2<usize>) -> Vec<na::Vector3<f32>> {
        let offsets: [na::Vector2<usize>; 4] = [v2(0, 0), v2(1, 0), v2(1, 1), v2(0, 1)];
        
        let mut out = vec![];

        for o in 0..4 {
            let focus_index = grid_index + offsets[o];
            let next_index = grid_index + offsets[(o + 1) % 4];

            let focus_position = self.grid[(focus_index.x, focus_index.y)];
            let next_position = self.grid[(next_index.x, next_index.y)];

            if focus_position != next_position {
                out.push(focus_position);
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn terrain() -> Terrain {
        let nodes = na::DMatrix::from_row_slice(3, 3, &[
            Node::point(0.0), Node::point(0.0), Node::point(0.0),
            Node::point(0.0), Node::new(0.5, 0.5, 4.0), Node::new(0.4, 0.1, 3.0),
            Node::point(0.0), Node::new(0.1, 0.4, 2.0), Node::new(0.0, 0.0, 1.0),
        ]).transpose();

        Terrain::new(nodes)
    }

    #[test]
    fn test_create_grid() {
        let actual = terrain().grid;

        let mut expected = na::DMatrix::from_element(6, 6, v3(0.0, 0.0, 0.0));

        for x in 0..5 {
            for y in 0..5 {
                expected[(x, y)] = v3((x / 2) as f32, (y / 2) as f32, 0.0);
            }
        }

        expected[(2, 2)] = v3(0.5, 0.5, 4.0);
        expected[(3, 2)] = v3(1.5, 0.5, 4.0);
        expected[(2, 3)] = v3(0.5, 1.5, 4.0);
        expected[(3, 3)] = v3(1.5, 1.5, 4.0);

        expected[(4, 2)] = v3(1.6, 0.9, 3.0);
        expected[(5, 2)] = v3(2.4, 0.9, 3.0);
        expected[(4, 3)] = v3(1.6, 1.1, 3.0);
        expected[(5, 3)] = v3(2.4, 1.1, 3.0);

        expected[(2, 4)] = v3(0.9, 1.6, 2.0);
        expected[(3, 4)] = v3(1.1, 1.6, 2.0);
        expected[(2, 5)] = v3(0.9, 2.4, 2.0);
        expected[(3, 5)] = v3(1.1, 2.4, 2.0);

        expected[(4, 4)] = v3(2.0, 2.0, 1.0);
        expected[(5, 4)] = v3(2.0, 2.0, 1.0);
        expected[(4, 5)] = v3(2.0, 2.0, 1.0);
        expected[(5, 5)] = v3(2.0, 2.0, 1.0);

        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(actual[(x, y)], expected[(x, y)]);
            }
        }
    }

     #[test]
    fn test_get_border_square() {
        let actual = terrain().get_border(v2(2, 2));

        assert_eq!(actual, vec![
            v3(0.5, 0.5, 4.0),
            v3(1.5, 0.5, 4.0),
            v3(0.5, 1.5, 4.0),
            v3(1.5, 1.5, 4.0),
        ]);
    }

    #[test]
    fn test_get_border_triangle() {
        let actual = terrain().get_border(v2(2, 1));

        assert_eq!(actual, vec![
            v3(1.0, 0.0, 0.0),
            v3(1.5, 0.5, 4.0),
            v3(0.5, 0.5, 4.0),
        ]);
    }

    #[test]
    fn test_get_border_line() {
        let actual = terrain().get_border(v2(1, 0));

        assert_eq!(actual, vec![
            v3(0.0, 0.0, 0.0),
            v3(1.0, 0.0, 0.0),
        ]);
    }

    #[test]
    fn test_get_border_empty() {
        let actual = terrain().get_border(v2(0, 0));

        assert_eq!(actual, vec![]);
    }

}
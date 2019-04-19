use {v2, v3, M, V2, V3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node {
    position: V2<usize>,
    width: f32,
    height: f32,
}

impl Node {
    pub fn point(position: V2<usize>) -> Node {
        Node {
            position,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn new(position: V2<usize>, width: f32, height: f32) -> Node {
        Node {
            position,
            width,
            height,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Edge {
    from: V2<usize>,
    to: V2<usize>,
}

impl Edge {
    pub fn new(from: V2<usize>, to: V2<usize>) -> Edge {
        if to.x > from.x && to.y > from.y {
            panic!("Diagonal edge {:?} from {:?}", from, to);
        }
        if to.x > from.x || to.y > from.y {
            Edge { from, to }
        } else {
            Edge { from: to, to: from }
        }
    }

    pub fn from(&self) -> &V2<usize> {
        &self.from
    }

    pub fn to(&self) -> &V2<usize> {
        &self.to
    }

    pub fn horizontal(&self) -> bool {
        self.from.y == self.to.y
    }
}

pub struct Terrain {
    width: usize,
    height: usize,
    slab_width: usize,
    slab_height: usize,
    slabs: Vec<Vec<Slab>>,
}

impl Terrain {
    pub fn new(elevations: M<f32>) -> Terrain {
        let width = elevations.shape().0;
        let height = elevations.shape().1;
        let slab_width = 32;
        let slab_height = 32;
        let mut slabs = vec![];
        for x in 0..width / slab_width {
            let mut slab_column = vec![];
            for y in 0..width / slab_height {
                let this_width = (width - (slab_width * x)).min(slab_width + 1);
                let this_height = (height - (slab_height * y)).min(slab_height + 1);
                let slab = Slab::new(elevations.slice((x * slab_width, y * slab_height), (this_width, this_height)).into(), &vec![], &vec![]);
                slab_column.push(slab);
            }
            slabs.push(slab_column);
        }
        Terrain{
            width,
            height,
            slab_width,
            slab_height,
            slabs
        }

    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_node(&mut self, node: &Node) {
        let position = v2(node.position.x % self.slab_width, node.position.y % self.slab_height);
        let node = Node::new(position, node.width, node.height);
        let slab = self.get_mut_slab_for_index(&Terrain::get_index_for_node(&node));
        slab.set_node(&node);
    }

    pub fn set_edge(&mut self, edge: &Edge) {
        let from = v2(edge.from.x % self.slab_width, edge.from.y % self.slab_height);
        let to = v2(edge.to.x % self.slab_width, edge.to.y % self.slab_height);
        let edge = Edge::new(from, to);
        let slab = self.get_mut_slab_for_index(&Terrain::get_index_for_edge(&edge));
        slab.set_edge(&edge);
    }

    pub fn clear_edge(&mut self, edge: &Edge) {
        let from = v2(edge.from.x % self.slab_width, edge.from.y % self.slab_height);
        let to = v2(edge.to.x % self.slab_width, edge.to.y % self.slab_height);
        let edge = Edge::new(from, to);
        let slab = self.get_mut_slab_for_index(&Terrain::get_index_for_edge(&edge));
        slab.clear_edge(&edge);
    }

    pub fn get_border(&self, grid_index: V2<usize>) -> Vec<V3<f32>> {
        let slab = self.get_slab_for_index(&grid_index);
        slab.get_border(v2(grid_index.x % (self.slab_width * 2), grid_index.y % (self.slab_height * 2)))
    }

    pub fn get_triangles(&self, grid_index: V2<usize>) -> Vec<[V3<f32>; 3]> {
        let slab = self.get_slab_for_index(&grid_index);
        slab.get_triangles(v2(grid_index.x % (self.slab_width * 2), grid_index.y % (self.slab_height * 2)))
    }

    pub fn get_index_for_node(node: &Node) -> V2<usize> {
        V2::new(node.position.x * 2, node.position.y * 2)
    }

    pub fn get_index_for_edge(edge: &Edge) -> V2<usize> {
        if edge.horizontal() {
            V2::new(edge.from.x * 2 + 1, edge.from.y * 2)
        } else {
            V2::new(edge.from.x * 2, edge.from.y * 2 + 1)
        }
    }

    pub fn get_index_for_tile(tile_coordinate: &V2<usize>) -> V2<usize> {
        V2::new((tile_coordinate.x * 2) + 1, (tile_coordinate.y * 2) + 1)
    }

    fn get_slab_for_index(&self, index: &V2<usize>) -> &Slab {
        &self.slabs[index.x / (self.slab_width * 2)][index.y / (self.slab_height * 2)]
    }

    fn get_mut_slab_for_index(&mut self, index: &V2<usize>) -> &mut Slab {
        &mut self.slabs[index.x / (self.slab_width * 2)][index.y / (self.slab_height * 2)]
    }

    pub fn get_triangles_for_tile(&self, tile_coordinate: &V2<usize>) -> Vec<[V3<f32>; 3]> {
        let tile_coordinate = v2(tile_coordinate.x % self.slab_width, tile_coordinate.y % self.slab_height);
        let slab = self.get_slab_for_index(&Terrain::get_index_for_tile(&tile_coordinate));
        slab.get_triangles_for_tile(&tile_coordinate)
    }


}

pub struct Slab {
    //grid: M<V3<f32>>,
    elevations: M<f32>,
    node: M<Node>,
    edge: M<bool>,
}

impl Slab {
    pub fn new(elevations: M<f32>, nodes: &Vec<Node>, edges: &Vec<Edge>) -> Slab {
        let width = elevations.shape().0;
        let height = elevations.shape().1;
        let mut out = Slab {
            elevations,
            //grid: Terrain::create_grid(elevations, nodes),
            node: Slab::init_node_matrix(width, height),
            edge: Slab::init_edge_matrix(width, height),
        };
        for node in nodes {
            out.set_node(&node);
        }
        for edge in edges {
            out.set_edge(&edge);
        }
        out
    }

    fn get_grid(&self, position: V2<usize>) -> V3<f32> {
        let x = position.x / 2;
        let y = position.y / 2;
        let node = self.node[(x, y)];
        let w = node.width;
        let h = node.height;
        let xf = x as f32;
        let yf = y as f32;
        let z = self.elevations[(x, y)];
        match (position.x % 2, position.y % 2) {
            (0, 0) => v3(xf - w, yf - h, z),
            (1, 0) => v3(xf + w, yf - h, z),
            (0, 1) => v3(xf - w, yf + h, z),
            (1, 1) => v3(xf + w, yf + h, z),
            (_, _) => panic!("Not expected to happen - {} % 2 or {} % 2 is not in range 0..1"),
        }
    }

    fn init_node_matrix(width: usize, height: usize) -> M<Node> {
        let mut out = M::from_element(width, height, Node::point(v2(0, 0)));

        for x in 0..width {
            for y in 0..height {
                out[(x, y)].position = v2(x, y);
            }
        }

        out
    }

    pub fn set_node(&mut self, node: &Node) {
        let position = (node.position.x, node.position.y);
        let current_node = self.node[position];
        let new_width = current_node.width.max(node.width);
        let new_height = current_node.height.max(node.height);
        self.node[position] = Node::new(node.position, new_width, new_height);
    }

    fn init_edge_matrix(width: usize, height: usize) -> M<bool> {
        M::from_element(width * 2, height * 2, false)
    }

    pub fn set_edge(&mut self, edge: &Edge) {
        let position = Terrain::get_index_for_edge(&edge);
        self.edge[(position.x, position.y)] = true;
    }

    pub fn clear_edge(&mut self, edge: &Edge) {
        let position = Terrain::get_index_for_edge(&edge);
        self.edge[(position.x, position.y)] = false;
    }

    pub fn get_border(&self, grid_index: V2<usize>) -> Vec<V3<f32>> {
        let offsets: [V2<usize>; 4] = [v2(0, 0), v2(1, 0), v2(1, 1), v2(0, 1)];

        let mut out = vec![];

        for o in 0..4 {
            let focus_index = grid_index + offsets[o];
            let next_index = grid_index + offsets[(o + 1) % 4];

            let focus_position = self.get_grid(v2(focus_index.x, focus_index.y));
            let next_position = self.get_grid(v2(next_index.x, next_index.y));

            if focus_position != next_position {
                out.push(focus_position);
            }
        }

        out
    }

    pub fn get_triangles(&self, grid_index: V2<usize>) -> Vec<[V3<f32>; 3]> {
        let border = self.get_border(grid_index);

        if border.len() == 4 {
            vec![
                [border[0], border[3], border[2]],
                [border[0], border[2], border[1]],
            ]
        } else if border.len() == 3 {
            vec![[border[0], border[2], border[1]]]
        } else {
            vec![]
        }
    }

    fn clip_to_tile(mut point: V3<f32>, tile_coordinate: &V2<usize>) -> V3<f32> {
        let x = tile_coordinate.x as f32;
        let y = tile_coordinate.y as f32;
        point.x = point.x.max(x).min(x + 1.0);
        point.y = point.y.max(y).min(y + 1.0);

        point
    }

    pub fn get_triangles_for_tile(&self, tile_coordinate: &V2<usize>) -> Vec<[V3<f32>; 3]> {
        let grid_index = Terrain::get_index_for_tile(tile_coordinate);
        let mut out = vec![];
        out.append(&mut self.get_triangles(grid_index));

        let adjacents = vec![
            v2(grid_index.x - 1, grid_index.y),
            v2(grid_index.x + 1, grid_index.y),
            v2(grid_index.x, grid_index.y - 1),
            v2(grid_index.x, grid_index.y + 1),
        ];

        for adjacent in adjacents {
            let triangles = self.get_triangles(adjacent);
            let edge = self.edge[(adjacent.x, adjacent.y)];
            if !edge && (triangles.len() == 1 || triangles.len() == 2) {
                for mut triangle in triangles {
                    for p in 0..3 {
                        triangle[p] = Slab::clip_to_tile(triangle[p], &tile_coordinate);
                    }
                    out.push(triangle);
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[rustfmt::skip]
    fn terrain() -> Slab {
        let elevations = M::from_row_slice(3, 3, &[
            0.0, 0.0, 0.0,
            0.0, 4.0, 3.0,
            0.0, 2.0, 1.0,
        ]).transpose();

        let nodes = vec![
            Node::new(v2(1, 1), 0.5, 0.5),
            Node::new(v2(2, 1), 0.4, 0.1),
            Node::new(v2(1, 2), 0.1, 0.4),
            Node::new(v2(2, 2), 0.0, 0.0),
        ];

        let edges = vec![
            Edge::new(v2(1, 1), v2(2, 1)),
            Edge::new(v2(2, 1), v2(2, 2)),
            Edge::new(v2(1, 2), v2(2, 2)),
        ];

        Slab::new(elevations, &nodes, &edges)
    }

    #[test]
    fn edges_should_be_canonical() {
        let edge = Edge::new(v2(1, 10), v2(10, 10));
        assert_eq!(
            edge,
            Edge {
                from: v2(1, 10),
                to: v2(10, 10)
            }
        );

        let edge = Edge::new(v2(10, 10), v2(1, 10));
        assert_eq!(
            edge,
            Edge {
                from: v2(1, 10),
                to: v2(10, 10)
            }
        );

        let edge = Edge::new(v2(10, 1), v2(10, 10));
        assert_eq!(
            edge,
            Edge {
                from: v2(10, 1),
                to: v2(10, 10)
            }
        );

        let edge = Edge::new(v2(10, 10), v2(10, 1));
        assert_eq!(
            edge,
            Edge {
                from: v2(10, 1),
                to: v2(10, 10)
            }
        );
    }

    #[test]
    fn test_horizontal() {
        let edge = Edge::new(v2(1, 10), v2(10, 10));
        assert!(edge.horizontal());

        let edge = Edge::new(v2(10, 1), v2(10, 10));
        assert!(!edge.horizontal());
    }

    #[rustfmt::skip]
    #[test]
    fn test_set_node() {
        let mut terrain = Slab::new(M::zeros(2, 2), &vec![], &vec![]);

        let nodes = vec![
            Node::new(v2(0, 0), 0.4, 0.0),
            Node::new(v2(0, 0), 0.0, 0.1),
            Node::new(v2(0, 0), 0.5, 0.4),
            Node::new(v2(1, 1), 0.1, 0.1),
        ];

        for node in nodes {
            terrain.set_node(&node);
        }

        assert_eq!(terrain.node[(0, 0)], Node::new(v2(0, 0), 0.5, 0.4));
        assert_eq!(terrain.node[(0, 1)], Node::new(v2(0, 1), 0.0, 0.0));
        assert_eq!(terrain.node[(1, 0)], Node::new(v2(1, 0), 0.0, 0.0));
        assert_eq!(terrain.node[(1, 1)], Node::new(v2(1, 1), 0.1, 0.1));
    }

    #[test]
    fn test_set_edge() {
        let mut terrain = Slab::new(M::zeros(2, 2), &vec![], &vec![]);
        let edge = &Edge::new(v2(0, 1), v2(1, 1));
        terrain.set_edge(&Edge::new(v2(0, 1), v2(1, 1)));

        let index = Terrain::get_index_for_edge(&edge);
        assert_eq!(terrain.edge[(index.x, index.y)], true);
    }

    #[test]
    fn test_clear_edge() {
        let mut terrain = Slab::new(M::zeros(2, 2), &vec![], &vec![]);
        let edge = &Edge::new(v2(0, 1), v2(1, 1));
        terrain.set_edge(&Edge::new(v2(0, 1), v2(1, 1)));
        terrain.clear_edge(&Edge::new(v2(0, 1), v2(1, 1)));

        let index = Terrain::get_index_for_edge(&edge);
        assert_eq!(terrain.edge[(index.x, index.y)], false);
    }

    #[test]
    fn test_get_grid() {
        let mut expected = M::from_element(6, 6, v3(0.0, 0.0, 0.0));

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

        let terrain = terrain();

        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(terrain.get_grid(v2(x, y)), expected[(x, y)]);
            }
        }
    }

    #[test]
    fn test_get_border_square() {
        let actual = terrain().get_border(v2(2, 2));

        assert_eq!(
            actual,
            vec![
                v3(0.5, 0.5, 4.0),
                v3(1.5, 0.5, 4.0),
                v3(1.5, 1.5, 4.0),
                v3(0.5, 1.5, 4.0),
            ]
        );
    }

    #[test]
    fn test_get_border_triangle() {
        let actual = terrain().get_border(v2(2, 1));

        assert_eq!(
            actual,
            vec![v3(1.0, 0.0, 0.0), v3(1.5, 0.5, 4.0), v3(0.5, 0.5, 4.0),]
        );
    }

    #[test]
    fn test_get_border_line() {
        let actual = terrain().get_border(v2(1, 0));

        assert_eq!(actual, vec![v3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0),]);
    }

    #[test]
    fn test_get_border_empty() {
        let actual = terrain().get_border(v2(0, 0));

        assert_eq!(actual, vec![]);
    }

    #[test]
    fn test_get_triangles_square() {
        let actual = terrain().get_triangles(v2(2, 2));

        assert_eq!(
            actual,
            vec![
                [v3(0.5, 0.5, 4.0), v3(0.5, 1.5, 4.0), v3(1.5, 1.5, 4.0)],
                [v3(0.5, 0.5, 4.0), v3(1.5, 1.5, 4.0), v3(1.5, 0.5, 4.0)],
            ]
        );
    }

    #[test]
    fn test_get_triangles_triangle() {
        let actual = terrain().get_triangles(v2(2, 1));

        assert_eq!(
            actual,
            vec![[v3(1.0, 0.0, 0.0), v3(0.5, 0.5, 4.0), v3(1.5, 0.5, 4.0)],]
        );
    }

    #[test]
    fn test_get_triangles_line() {
        let actual = terrain().get_triangles(v2(1, 0));
        let expected: Vec<[V3<f32>; 3]> = vec![];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_triangles_point() {
        let actual = terrain().get_triangles(v2(0, 0));
        let expected: Vec<[V3<f32>; 3]> = vec![];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_index_for_node() {
        let mut actual = vec![];
        for y in 0..3 {
            for x in 0..3 {
                actual.push(Terrain::get_index_for_node(&Node::point(v2(x, y))));
            }
        }
        let expected = vec![
            v2(0, 0),
            v2(2, 0),
            v2(4, 0),
            v2(0, 2),
            v2(2, 2),
            v2(4, 2),
            v2(0, 4),
            v2(2, 4),
            v2(4, 4),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_index_for_tile() {
        let mut actual = vec![];
        for y in 0..2 {
            for x in 0..2 {
                actual.push(Terrain::get_index_for_tile(&v2(x, y)));
            }
        }
        let expected = vec![v2(1, 1), v2(3, 1), v2(1, 3), v2(3, 3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_clip_to_tile() {
        let actual = Slab::clip_to_tile(v3(9.5, 10.5, 12.0), &v2(10, 10));
        assert_eq!(actual, v3(10.0, 10.5, 12.0));

        let actual = Slab::clip_to_tile(v3(11.5, 10.5, 12.0), &v2(10, 10));
        assert_eq!(actual, v3(11.0, 10.5, 12.0));

        let actual = Slab::clip_to_tile(v3(10.5, 9.5, 12.0), &v2(10, 10));
        assert_eq!(actual, v3(10.5, 10.0, 12.0));

        let actual = Slab::clip_to_tile(v3(10.5, 11.5, 12.0), &v2(10, 10));
        assert_eq!(actual, v3(10.5, 11.0, 12.0));
    }

    #[test]
    fn test_get_triangles_for_tile_a() {
        let terrain = terrain();

        let actual = terrain.get_triangles_for_tile(&v2(1, 0));

        assert!(actual.contains(&[v3(1.0, 0.0, 0.0), v3(1.5, 0.5, 4.0), v3(1.6, 0.9, 3.0)]));
        assert!(actual.contains(&[v3(1.0, 0.0, 0.0), v3(1.6, 0.9, 3.0), v3(2.0, 0.0, 0.0)]));
        assert!(actual.contains(&[v3(1.0, 0.0, 0.0), v3(1.0, 0.5, 4.0), v3(1.5, 0.5, 4.0)]));
        assert!(actual.contains(&[v3(2.0, 0.0, 0.0), v3(1.6, 0.9, 3.0), v3(2.0, 0.9, 3.0)]));
        assert_eq!(actual.len(), 4);
    }

    #[test]
    fn test_get_triangles_for_tile_b() {
        let terrain = terrain();

        let actual = terrain.get_triangles_for_tile(&v2(1, 1));

        assert!(actual.contains(&[v3(1.5, 1.5, 4.0), v3(1.1, 1.6, 2.0), v3(2.0, 2.0, 1.0)]));
        assert!(actual.contains(&[v3(1.5, 1.5, 4.0), v3(2.0, 2.0, 1.0), v3(1.6, 1.1, 3.0)]));
        assert!(actual.contains(&[v3(1.0, 1.5, 4.0), v3(1.0, 1.6, 2.0), v3(1.1, 1.6, 2.0)]));
        assert!(actual.contains(&[v3(1.0, 1.5, 4.0), v3(1.1, 1.6, 2.0), v3(1.5, 1.5, 4.0)]));
        assert_eq!(actual.len(), 4);
    }

}

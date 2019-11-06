// chip_info.rs
// Frank Lee
// translated from chipinfo.py
use crate::populate;
use std::convert::TryInto;

#[allow(dead_code)]
pub struct ChipInfo {
    qubit_num: i64,
    coupling_list: Vec<Vec<i64>>,
    grid_edge_list: Vec<Vec<i64>>,
    via_edge_list: Vec<Vec<i64>>,
    edge_list: Vec<Vec<i64>>,
    qubitgrid: Vec<Vec<i64>>,
    adj_mat: Vec<Vec<i64>>,
    crossbuslist: Vec<Vec<i64>>,
}

#[allow(dead_code)]
impl ChipInfo {
    pub fn new(a: i64, b: Vec<Vec<i64>>, c: Vec<Vec<i64>>,
        d: Vec<Vec<i64>>, e: Vec<Vec<i64>>, f: Vec<Vec<i64>>,
        g:Vec<Vec<i64>>, h: Vec<Vec<i64>>) -> ChipInfo {
            ChipInfo {
                qubit_num: a,
                coupling_list: b,
                grid_edge_list: c,
                via_edge_list: d,
                edge_list: e,
                qubitgrid: f,
                adj_mat: g,
                crossbuslist: h,
            }
    }

    pub fn print_qubit_grid(&self) {
        println!("{:?}", self.qubitgrid);
    }

    pub fn print_details(&self) {
        println!("{:?}",self.qubit_num);
        println!("{:?}",self.coupling_list);
        println!("{:?}",self.grid_edge_list);
        println!("{:?}",self.via_edge_list);
        println!("{:?}",self.edge_list);
        println!("{:?}",self.qubitgrid);
        println!("{:?}",self.adj_mat);
        println!("{:?}",self.crossbuslist);
    }

    // bus generation
    pub fn generate_bus(&mut self) {
        let dimx = self.qubitgrid.len();
        let dimy = self.qubitgrid[0].len();
        // build the adjacency matrix
        self.adj_mat = vec![vec![0;self.qubit_num.try_into().unwrap()];self.qubit_num.try_into().unwrap()];
        // now populate it
        for x in 0..dimx {
            for y in 0..dimy {
                if self.qubitgrid[x][y] > -1 {
                    if x > 0 && self.qubitgrid[x-1][y] > -1 {
                        let i1 : usize = self.qubitgrid[x][y].try_into().unwrap();
                        let i2 : usize = self.qubitgrid[x-1][y].try_into().unwrap();
                        self.adj_mat[i1][i2] = 1;
                        self.adj_mat[i2][i1] = 1;
                        self.coupling_list.push(vec![self.qubitgrid[x-1][y], self.qubitgrid[x][y]]);
                    }
                    if y > 0 && self.qubitgrid[x][y-1] > -1 {
                        let i1 : usize = self.qubitgrid[x][y].try_into().unwrap();
                        let i2 : usize = self.qubitgrid[x][y-1].try_into().unwrap();
                        self.adj_mat[i1][i2] = 1;
                        self.adj_mat[i2][i1] = 1;
                        self.coupling_list.push(vec![self.qubitgrid[x][y-1], self.qubitgrid[x][y]]);
                    }
                }
            }
        }
        self.grid_edge_list = self.coupling_list.clone();
        for x in 1..(dimx-1) {
            for y in 0..dimy {
                if self.qubitgrid[x][y] > -1 && self.qubitgrid[x-1][y] > -1 && self.qubitgrid[x+1][y] > -1 {
                    self.via_edge_list.push(vec![self.qubitgrid[x-1][y], self.qubitgrid[x][y], self.qubitgrid[x+1][y]]);
                }
            }
        }
        for x in 0..dimx {
            for y in 1..(dimy-1) {
                if self.qubitgrid[x][y] > -1 && self.qubitgrid[x][y-1] > -1 && self.qubitgrid[x][y+1] > -1 {
                    self.via_edge_list.push(vec![self.qubitgrid[x][y-1], self.qubitgrid[x][y], self.qubitgrid[x][y+1]]);
                }
            }
        }
        self.edge_list = vec![vec![];self.qubit_num.try_into().unwrap()];
        for c in &self.coupling_list {
            let qi : usize = c[0].try_into().unwrap();
            let qj : usize = c[1].try_into().unwrap();
            self.edge_list[qi].push(c[1]);
            self.edge_list[qj].push(c[0]);
        }
     }

    pub fn patch_bus4(&mut self) {
        for bus in &self.crossbuslist {
            let x : usize = bus[0].try_into().unwrap();
            let y : usize = bus[1].try_into().unwrap();
            if x > 0 && y > 0 {
                if self.qubitgrid[x][y] > -1 && self.qubitgrid[x-1][y-1] > -1 {
                    let qubit_i : usize = self.qubitgrid[x][y].try_into().unwrap();
                    let qubit_j : usize = self.qubitgrid[x-1][y-1].try_into().unwrap();
                    self.adj_mat[qubit_i][qubit_j] = 1;
                    self.adj_mat[qubit_j][qubit_i] = 1;
                    self.coupling_list.push(vec![self.qubitgrid[x][y], self.qubitgrid[x-1][y-1]]);
                    self.edge_list[qubit_i].push(self.qubitgrid[x-1][y-1]);
                    self.edge_list[qubit_j].push(self.qubitgrid[x][y]);
                }
                if self.qubitgrid[x-1][y] > -1 && self.qubitgrid[x][y-1] > -1 {
                    let qubit_i : usize = self.qubitgrid[x -1][y].try_into().unwrap();
                    let qubit_j : usize = self.qubitgrid[x][y-1].try_into().unwrap();
                    self.adj_mat[qubit_i][qubit_j] = 1;
                    self.adj_mat[qubit_j][qubit_i] = 1;
                    self.coupling_list.push(vec![self.qubitgrid[x -1][y], self.qubitgrid[x][y-1]]);
                    self.edge_list[qubit_i].push(self.qubitgrid[x][y-1]);
                    self.edge_list[qubit_j].push(self.qubitgrid[x -1][y]);
                }
            }
        }
    }

    // populate chip struct
    pub fn populate_from_file(&mut self, filename: &str) {
        // string representation of the file
        let file_contents : String = populate::read_chip_file(filename);
        // keep track of the qubit id
        let mut qubit_id : i64 = 0;

        for lines in file_contents.lines() {
            // parse each line and split the lines into an easy vector
            // store this in line
            let line : Vec<&str> = lines.split_whitespace().collect();
            if line.len() == 0 { continue; }
            if line[0] == "#" {
                // a comment in the chip file
                continue;
            }
            if line[0] == "qubit" {
                // found a qubit declaration
                self.qubit_num = line[1].parse::<i64>().unwrap();
            }
            if line[0] == "griddim" {
                // initalising dimensions
                let dimx : usize = line[1].parse::<usize>().unwrap();
                let dimy : usize = line[2].parse::<usize>().unwrap();
                self.qubitgrid = vec![vec![-1;dimy];dimx];
            }
            if line[0] == "q" {
                // connection found
                let x : usize = line[1].parse::<usize>().unwrap();
                let y : usize = line[2].parse::<usize>().unwrap();
                self.qubitgrid[x][y] = qubit_id;
                qubit_id += 1;
            }
            if line[0] == "bustype" {
                // current code calls the same function
                self.generate_bus();
            }
            if line[0] == "b" {
                // bus list
                let x : i64 = line[1].parse::<i64>().unwrap();
                let y : i64 = line[2].parse::<i64>().unwrap();
                self.crossbuslist.push(vec![x,y]);
            }
        }
        self.patch_bus4();
    }
}
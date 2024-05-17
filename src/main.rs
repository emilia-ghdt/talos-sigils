use std::fs;

pub enum RotSym {
    QuarterTurns,
    HalfTurns,
    FullTurns,
}

pub struct Tile {
    squares: Vec<(i8, i8)>,
    rot_sym: RotSym,
    placed: bool,
}

impl Tile {
    fn get_square_rotated(&self, square_id: usize, rotation: u8) -> (i8, i8) {
        match (&self.rot_sym, rotation) {
            (_, 0) => self.squares[square_id],
            (RotSym::FullTurns, 1) => {
                (-self.squares[square_id].1, self.squares[square_id].0)
            },
            (RotSym::FullTurns, 2) | (RotSym::HalfTurns, 1) => {
                (-self.squares[square_id].0, -self.squares[square_id].1)
            },
            (RotSym::FullTurns, 3) => {
                (self.squares[square_id].1, -self.squares[square_id].0)
            },
            _ => panic!(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SquareAllocation {
    id: usize,
    square_id: usize,
    rotation: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub struct TilePlacementOnField {
    square_id: usize,
    x: usize,
    y: usize,
    rotation: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TilePlacement {
    OnField(TilePlacementOnField),
    OffField,
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Failed to read input file");

    let mut problem_input: Vec<Vec<String>> = contents.split("\n\n").map(|section| section.lines().map(str::to_owned).collect::<Vec<_>>()).collect();

    let tiles = problem_input.split_off(1);
    let field_size: Vec<usize> = problem_input[0][0].split(" ").map(|a| a.parse::<usize>().expect("Invalid input")).collect();
    let field_size = (field_size[0], field_size[1]);

    let mut tiles: Vec<Tile> = tiles.iter().map(|tile| {
        let mut squares = vec![];
        for y in 0..tile.len() {
            for x in 0..tile[y].len() {
                if tile[y].chars().nth(x).unwrap() != ' ' {
                    squares.push((x as i8, y as i8));
                }
            }
        }
        
        // There is a much better way to do this next part, but I don't care
        // Try 90-degree rotation
        let mut quarter_turn = false;
        'outer: for y in 0..10 {
            for x in 0..10 {
                let mut all_found = true;
                for square_id in 0..squares.len() {
                    let cur_sq_rot = (-squares[square_id].1 + x as i8, squares[square_id].0 + y as i8);
                    let mut found = false;
                    for square_id_2 in 0..squares.len() {
                        if squares[square_id_2] == cur_sq_rot {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        all_found = false;
                        break;
                    }
                }
                if all_found {
                    quarter_turn = true;
                    break 'outer;
                }
            }
        }
        if quarter_turn {
            return Tile {
                squares,
                rot_sym: RotSym::QuarterTurns,
                placed: false,
            }
        }

        let mut half_turn = false;
        'outer: for y in 0..10 {
            for x in 0..10 {
                let mut all_found = true;
                for square_id in 0..squares.len() {
                    let cur_sq_rot = (-squares[square_id].0 + x as i8, -squares[square_id].1 + y as i8);
                    let mut found = false;
                    for square_id_2 in 0..squares.len() {
                        if squares[square_id_2] == cur_sq_rot {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        all_found = false;
                        break;
                    }
                }
                if all_found {
                    half_turn = true;
                    break 'outer;
                }
            }
        }
        if half_turn {
            return Tile {
                squares,
                rot_sym: RotSym::HalfTurns,
                placed: false,
            }
        } else {
            return Tile {
                squares,
                rot_sym: RotSym::FullTurns,
                placed: false,
            }
        }
    }).collect();

    // Some(empty vec) represents not computed yet
    // None represents tile not available
    let mut field: Vec<Vec<Option<Vec<SquareAllocation>>>> = vec![vec![Some(vec![]); field_size.0]; field_size.1];
    let mut populated_field: Vec<Vec<Option<usize>>> = vec![vec![None; field_size.0]; field_size.1];
    let mut populated_tiles = 0u32;
    let total_tiles = (field_size.0 * field_size.1) as u32;

    let mut tile_placements_per_tile: Vec<Vec<TilePlacement>> = vec![vec![]; tiles.len()];
    fn check_placement_and_update(field: &mut Vec<Vec<Option<Vec<SquareAllocation>>>>, tiles: &Vec<Tile>, tile_placements_per_tile: &mut Vec<Vec<TilePlacement>>, x: usize, y: usize, placement: SquareAllocation) -> bool {
        if field[y][x].is_none() {
            return false;
        }
        for existing_placement in field[y][x].as_ref().unwrap() {
            if existing_placement == &placement {
                return true;
            }
        }
        let placed_square = tiles[placement.id].get_square_rotated(placement.square_id, placement.rotation);
        
        for cur_square_id in 0..tiles[placement.id].squares.len() {
            let cur_square = tiles[placement.id].get_square_rotated(cur_square_id, placement.rotation);
            let placement = (x as i8 + cur_square.0 - placed_square.0, y as i8 + cur_square.1 - placed_square.1);
            if placement.0 < 0 || placement.1 < 0 || placement.1 >= field.len() as i8 || placement.0 >= field[placement.1 as usize].len() as i8 {
                return false;
            }
            if field[placement.1 as usize][placement.0 as usize].is_none() {
                return false;
            }
        }
        for cur_square_id in 0..tiles[placement.id].squares.len() {
            let cur_square = tiles[placement.id].get_square_rotated(cur_square_id, placement.rotation);
            let placement_pos = (x as i8 + cur_square.0 - placed_square.0, y as i8 + cur_square.1 - placed_square.1);
            field[placement_pos.1 as usize][placement_pos.0 as usize].as_mut().map(|v| v.push(SquareAllocation{ square_id: cur_square_id, ..placement}));
        }
        tile_placements_per_tile[placement.id].push(TilePlacement::OnField(TilePlacementOnField { rotation: placement.rotation, square_id: placement.square_id, x, y }));
        true
    }

    fn recursive_call(field: &mut Vec<Vec<Option<Vec<SquareAllocation>>>>, tile_placements_per_tile: &mut Vec<Vec<TilePlacement>>, tiles: &mut Vec<Tile>, populated_field: &mut Vec<Vec<Option<usize>>>, populated_tiles: &mut u32, total_tiles: u32) -> bool {
        // Compute valid placements
        for tile_id in 0..tiles.len() {
            if tiles[tile_id].placed {
                continue;
            }
            if tiles[tile_id].squares.len() as u32 <= total_tiles - *populated_tiles {
                tile_placements_per_tile[tile_id].push(TilePlacement::OffField);
            }
        }
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x].is_some() {
                    // Reset field
                    field[y][x] = Some(vec![]);
                } else {
                    continue;
                }
                for tile_id in 0..tiles.len() {
                    if tiles[tile_id].placed {
                        continue;
                    }
                    for square_id in 0..tiles[tile_id].squares.len() {
                        let rotations = match tiles[tile_id].rot_sym {
                            RotSym::FullTurns => 4,
                            RotSym::HalfTurns => 2,
                            RotSym::QuarterTurns => 1,
                        };
                        for rotation in 0..rotations {
                            let cur_placement = SquareAllocation {id: tile_id, square_id, rotation};
                            check_placement_and_update(field, &tiles, tile_placements_per_tile, x, y, cur_placement);
                        }
                    }
                }
            }
        }
    
        // Find best placements
        let mut best_square = (0, None);
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x].is_none() {
                    continue;
                }
                if best_square.1.is_none() || field[y][x].as_mut().unwrap().len() < best_square.0 {
                    best_square.0 = field[y][x].as_mut().unwrap().len();
                    best_square.1 = Some((x, y));
                }
            }
        }
        if best_square.1.is_none() {
            // No more valid placements
            // Done?
            if populated_field.iter().map(|v| v.iter()).flatten().any(|t| t.is_none()) {
                // Not done, not all squares have tiles
                return false;
            } else {
                for row in populated_field {
                    for tile in row {
                        print!("{}", "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(tile.unwrap()).unwrap());
                    }
                    println!();
                }
                return true;
            }
        }
        
        let mut best_tile = (0, None);
        for (i, tile) in tile_placements_per_tile.iter().enumerate() {
            if best_tile.1.is_none() || tile.len() < best_tile.0 {
                best_tile.0 = tile.len();
                best_tile.1 = Some(i);
            }
        }
        
        // Iterate over placements
        if best_square.0 < best_tile.0 {
            // Square
            let x = best_square.1.unwrap().0;
            let y = best_square.1.unwrap().1;
            let placements = field[y][x].clone().unwrap();
            for placement in placements {
                let placement: SquareAllocation = placement;

                // Do placement
                let placed_square = tiles[placement.id].get_square_rotated(placement.square_id, placement.rotation);
                for cur_square_id in 0..tiles[placement.id].squares.len() {
                    let cur_square = tiles[placement.id].get_square_rotated(cur_square_id, placement.rotation);
                    let position = (x as i8 + cur_square.0 - placed_square.0, y as i8 + cur_square.1 - placed_square.1);
                    populated_field[position.1 as usize][position.0 as usize] = Some(placement.id);
                    field[position.1 as usize][position.0 as usize] = None;
                }
                *populated_tiles += tiles[placement.id].squares.len() as u32;
                tiles[placement.id].placed = true;
                
                // Recursive call
                if recursive_call(field, tile_placements_per_tile, tiles, populated_field, populated_tiles, total_tiles) {
                    return true;
                }
                // Undo placement
                for cur_square_id in 0..tiles[placement.id].squares.len() {
                    let cur_square = tiles[placement.id].get_square_rotated(cur_square_id, placement.rotation);
                    let position = (x as i8 + cur_square.0 - placed_square.0, y as i8 + cur_square.1 - placed_square.1);
                    populated_field[position.1 as usize][position.0 as usize] = None;
                    field[position.1 as usize][position.0 as usize] = Some(vec![]);
                }
                *populated_tiles -= tiles[placement.id].squares.len() as u32;
                tiles[placement.id].placed = false;
            }
        } else {
            // Tile
            let tile_id = best_tile.1.unwrap();
            let placements = tile_placements_per_tile[tile_id].clone();
            for placement in placements {
                // Do placement
                match placement.clone() {
                    TilePlacement::OffField => {
                        tiles[tile_id].placed = true;
                    },
                    TilePlacement::OnField(tile_placement) => {
                        let placed_square = tiles[tile_id].get_square_rotated(tile_placement.square_id, tile_placement.rotation);
                        for cur_square_id in 0..tiles[tile_id].squares.len() {
                            let cur_square = tiles[tile_id].get_square_rotated(cur_square_id, tile_placement.rotation);
                            let position = (tile_placement.x as i8 + cur_square.0 - placed_square.0, tile_placement.y as i8 + cur_square.1 - placed_square.1);
                            populated_field[position.1 as usize][position.0 as usize] = Some(tile_id);
                            field[position.1 as usize][position.0 as usize] = None;
                        }
                        *populated_tiles += tiles[tile_id].squares.len() as u32;
                        tiles[tile_id].placed = true;
                    }
                }

                // Recursive call
                if recursive_call(field, tile_placements_per_tile, tiles, populated_field, populated_tiles, total_tiles) {
                    return true;
                }
                // Undo placement
                match placement {
                    TilePlacement::OffField => {
                        tiles[tile_id].placed = false;
                    },
                    TilePlacement::OnField(tile_placement) => {
                        let placed_square = tiles[tile_id].get_square_rotated(tile_placement.square_id, tile_placement.rotation);
                        for cur_square_id in 0..tiles[tile_id].squares.len() {
                            let cur_square = tiles[tile_id].get_square_rotated(cur_square_id, tile_placement.rotation);
                            let position = (tile_placement.x as i8 + cur_square.0 - placed_square.0, tile_placement.y as i8 + cur_square.1 - placed_square.1);
                            populated_field[position.1 as usize][position.0 as usize] = None;
                            field[position.1 as usize][position.0 as usize] = Some(vec![]);
                        }
                        *populated_tiles -= tiles[tile_id].squares.len() as u32;
                        tiles[tile_id].placed = false;
                    }
                }

            }
        }

        return false;
    }

    recursive_call(&mut field, &mut tile_placements_per_tile, &mut tiles, &mut populated_field, &mut populated_tiles, total_tiles);
}

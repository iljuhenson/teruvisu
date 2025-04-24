use std::collections::HashSet;

#[derive(Debug)]
pub struct Sudoku<'a> {
    grid: Vec<Vec<Cell<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct CellPos {
    x: usize,
    y: usize,
}

impl CellPos {
    pub fn build(x: usize, y: usize) -> Result<Self, &'static str> {
        Self::validate_sudoku_indexes(x, y)?;

        Ok(Self { x, y })
    }

    fn validate_sudoku_indexes(x: usize, y: usize) -> Result<(), &'static str> {
        if x > 8 || y > 8 {
            return Err("Index out of range");
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct MiniSquareBounds {
    top_left: CellPos,
    bottom_right: CellPos,
}

#[derive(Debug)]
struct Cell<'a> {
    value: Option<u8>,
    note: &'a str,
}

impl<'a> Sudoku<'a> {
    pub fn generate(uncovered_cells_amount: u8) -> Self {
        let mut sudoku = Self::build_empty();

        sudoku.fill_randomly();

        sudoku
    }

    fn fill_randomly(&mut self) {
        let mut is_filled = false;

        while !is_filled {
            is_filled = true;

            for x in 0..9 {
                for y in 0..9 {}
            }
        }
    }

    fn build_empty() -> Self {
        let mut sudoku = Self { grid: Vec::new() };

        for _ in 0..9 {
            let mut y_line: Vec<Cell> = Vec::new();
            for _ in 0..9 {
                y_line.push(Cell {
                    value: None,
                    note: "",
                });
            }
            sudoku.grid.push(y_line);
        }

        sudoku
    }

    pub fn get_value(&self, pos: &CellPos) -> Result<Option<u8>, &'static str> {
        Ok(self.grid[pos.x][pos.y].value)
    }

    pub fn get_note(&self, pos: &CellPos) -> Result<&'a str, &'static str> {
        Ok(self.grid[pos.x][pos.y].note)
    }

    pub fn set_value(&mut self, pos: &CellPos, value: u8) -> Result<(), &'static str> {
        if value < 1 && value > 9 {
            return Err("Provided value is not in range 1..=9");
        }

        self.grid[pos.x][pos.y].value = Some(value);

        Ok(())
    }

    pub fn unset_value(&mut self, pos: &CellPos) -> Result<(), &'static str> {
        self.grid[pos.x][pos.y].value = None;

        Ok(())
    }

    pub fn set_note<'b: 'a>(&mut self, pos: &CellPos, note: &'b str) -> Result<(), &'static str> {
        self.grid[pos.x][pos.y].note = note;

        Ok(())
    }

    fn available_values_for(&self, pos: &CellPos) -> Vec<u8> {
        let mut available_values = HashSet::from([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8]);

        self.column_values_for(pos).iter().for_each(|v| {
            available_values.remove(v);
        });
        self.row_values_for(pos).iter().for_each(|v| {
            available_values.remove(v);
        });
        self.minisquare_values_for(pos).iter().for_each(|v| {
            available_values.remove(v);
        });

        available_values.into_iter().collect()
    }

    fn column_values_for(&self, pos: &CellPos) -> Vec<u8> {
        let mut values: Vec<u8> = Vec::with_capacity(9);

        for x in 0..9 {
            if let Some(value) = self.grid[x][pos.y].value {
                values.push(value);
            }
        }

        values
    }

    fn row_values_for(&self, pos: &CellPos) -> Vec<u8> {
        let mut values: Vec<u8> = Vec::with_capacity(9);

        for y in 0..9 {
            if let Some(value) = self.grid[pos.x][y].value {
                values.push(value);
            }
        }

        values
    }

    fn minisquare_values_for(&self, pos: &CellPos) -> Vec<u8> {
        let bounds = Self::calculate_minisquare_bounds(pos);
        let mut values: Vec<u8> = Vec::with_capacity(9);

        for x in bounds.top_left.x..=bounds.bottom_right.x {
            for y in bounds.top_left.y..=bounds.bottom_right.y {
                if let Some(value) = self.grid[x][y].value {
                    values.push(value);
                }
            }
        }

        values
    }

    fn calculate_minisquare_bounds(pos: &CellPos) -> MiniSquareBounds {
        let top_left_x = pos.x / 3 * 3;
        let top_left_y = pos.y / 3 * 3;

        let bottom_right_x = top_left_x + 2;
        let bottom_right_y = top_left_y + 2;

        MiniSquareBounds {
            top_left: CellPos {
                x: top_left_x,
                y: top_left_y,
            },
            bottom_right: CellPos {
                x: bottom_right_x,
                y: bottom_right_y,
            },
        }
    }

    //fn is_row_valid_for(pos: &CellPos) -> bool {
    //    false
    //}
    //
    //fn is_column_valid_for(pos: &CellPos) -> bool {
    //    false
    //}
    //
    //fn is_minisquare_valid_for(pos: &CellPos) -> bool {
    //    false
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        let sudoku = build_sudoku_from(vec![
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
        ]);
    */
    fn build_sudoku_from(template: Vec<Vec<u8>>) -> Sudoku<'static> {
        let mut sudoku = Sudoku {
            grid: Vec::with_capacity(9),
        };

        for x in 0..9 {
            let mut y_line: Vec<Cell> = Vec::with_capacity(9);
            for y in 0..9 {
                y_line.push(Cell {
                    value: if template[x][y] != 0 {
                        Some(template[x][y])
                    } else {
                        None
                    },
                    note: "",
                });
            }
            sudoku.grid.push(y_line);
        }

        sudoku
    }

    #[test]
    fn returns_error_when_indexes_out_of_sudoku_range() {
        let x = 9;
        let y = 8;
        let result = CellPos::validate_sudoku_indexes(x, y);

        assert!(result.is_err());

        let x = 8;
        let y = 9;
        let result = CellPos::validate_sudoku_indexes(x, y);

        assert!(result.is_err());
    }

    #[test]
    fn returns_ok_when_indexes_in_sudoku_range() {
        let x = 8;
        let y = 8;
        let result = CellPos::validate_sudoku_indexes(x, y);

        assert!(result.is_ok());

        let x = 0;
        let y = 0;
        let result = CellPos::validate_sudoku_indexes(x, y);

        assert!(result.is_ok());
    }

    #[test]
    fn correctly_calculates_minisquare_bounds() {
        let pos = CellPos::build(1, 4).unwrap();
        let result = Sudoku::calculate_minisquare_bounds(&pos);

        assert_eq!(
            MiniSquareBounds {
                top_left: CellPos { x: 0, y: 3 },
                bottom_right: CellPos { x: 2, y: 5 },
            },
            result
        );

        let pos = CellPos::build(8, 8).unwrap();
        let result = Sudoku::calculate_minisquare_bounds(&pos);

        assert_eq!(
            MiniSquareBounds {
                top_left: CellPos { x: 6, y: 6 },
                bottom_right: CellPos { x: 8, y: 8 },
            },
            result
        );
    }

    #[test]
    fn correctly_finds_minisquare_values() {
        let sudoku = build_sudoku_from(vec![
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 2, 5, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 1, 0, 9, /*|*/ 0, 0, 0],
        ]);
        let pos = CellPos { x: 8, y: 4 };

        let mut result = sudoku.minisquare_values_for(&pos);
        result.sort();
        assert_eq!(result, vec![1, 2, 5, 9]);
    }

    #[test]
    fn correctly_finds_column_values() {
        let sudoku = build_sudoku_from(vec![
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 5, 2, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 1, 0, 9, /*|*/ 0, 0, 0],
        ]);
        let pos = CellPos { x: 2, y: 3 };

        let mut result = sudoku.column_values_for(&pos);
        result.sort();
        assert_eq!(result, vec![1, 5]);
    }

    #[test]
    fn correctly_finds_row_values() {
        let sudoku = build_sudoku_from(vec![
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 5, 2, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 1, 0, 9, /*|*/ 0, 0, 0],
        ]);
        let pos = CellPos { x: 7, y: 2 };

        let mut result = sudoku.row_values_for(&pos);
        result.sort();
        assert_eq!(result, vec![2, 5]);
    }

    #[test]
    fn correctly_finds_available_values_for_specified_cell() {
        let sudoku = build_sudoku_from(vec![
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 1, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 3, 0, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 5, 2, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 8, /*|*/ 1, 0, 9, /*|*/ 0, 0, 0],
        ]);
        let pos = CellPos { x: 8, y: 4 };

        let mut result = sudoku.available_values_for(&pos);
        result.sort();
        assert_eq!(result, vec![4u8, 6u8, 7u8]);

        let sudoku = build_sudoku_from(vec![
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 7, 0, 3, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 1, 0, /*|*/ 2, 0, 4, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 2, 0, /*|*/ 3, 0, 9, /*|*/ 0, 0, 0],
            /*----------------------------------------*/
            vec![0, 3, 6, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
            vec![0, 0, 0, /*|*/ 0, 0, 0, /*|*/ 0, 0, 0],
        ]);
        let pos = CellPos { x: 8, y: 4 };

        let mut result = sudoku.available_values_for(&pos);
        result.sort();
        assert_eq!(result, vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8]);
    }
}

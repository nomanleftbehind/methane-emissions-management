pub fn gen_grid_style(col: usize, row: usize) -> String {
    format!("grid-column: {}; grid-row: {};", col, row)
}

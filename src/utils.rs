pub struct TabularData {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<usize>,
}

impl TabularData {
    /// Constructs a new table with empty columns and no rows.
    pub fn new() -> Self {
        Self {
            columns: vec![],
            rows: vec![],
            column_widths: vec![],
        }
    }

    /// Set the columns of a table.
    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns.clone();

        self.column_widths = Vec::new();
        for i in columns {
            self.column_widths.push(i.len() + 2);
        }
    }

    /// Adds a row to the table, each element should correspond to a column.
    pub fn add_row(&mut self, row: Vec<String>) {
        for (i, el) in row.iter().enumerate() {
            if el.len() + 2 > self.column_widths[i] {
                self.column_widths[i] = el.len() + 2;
            }
        }

        self.rows.push(row);
    }

    /// Renders the table with its columns in rST format e.g.
    /// ```
    /// +-------+-----+
    /// | Name  | Age |
    /// +-------+-----+
    /// | Alice | 24  |
    /// |  Bob  | 19  |
    /// +-------+-----+
    /// ```
    pub fn render(&self) -> String {
        let mut table: Vec<String> = Vec::new();

        // table column header
        let mut sep = String::new();
        for w in &self.column_widths {
            sep += &"-".repeat(w.to_owned());
            sep += "+";
        }
        sep = format!("+{}", sep);
        table.push(sep.clone());

        fn get_entry(row: &Vec<String>, widths: &Vec<usize>) -> String {
            let elem = row
                .iter()
                .zip(widths)
                .map(|(d, w)| format!("{:^width$}", d, width = w))
                .collect::<Vec<String>>()
                .join("|");

            format!("|{}|", elem)
        }

        table.push(get_entry(&self.columns, &self.column_widths));
        table.push(sep.clone());

        for row in &self.rows {
            table.push(get_entry(row, &self.column_widths));
        }
        table.push(sep.clone());

        table.join("\n")
    }
}

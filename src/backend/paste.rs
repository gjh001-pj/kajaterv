use crate::backend::keyboard::TableFocusNavigator;


pub trait PasteCell {
    fn paste(&mut self, cell: &str, index: usize);
}

pub fn handle_paste<T>(
    text: &str, row_index: usize, column_index: usize, 
    data_vec: &mut Vec<T>, focus_nav: &mut TableFocusNavigator
)
where 
    T: PasteCell + Default
{
    let mut i = 0;
    let mut rows = text.split("\n");
    loop { match rows.next() {
        None => break,
        Some(row) => {
            if row == "" { break; }
            let data = if row_index + i < data_vec.len() {
                data_vec.get_mut(row_index + i).expect("előbb néztül meg, hogy van")
            } else {
                let new = T::default();
                data_vec.push(new);
                focus_nav.build(focus_nav.rows + 1, 4);
                data_vec.last_mut().expect("az előbb adtunk hozzá egy összetevőt, nem lehet üres")
            };
            let mut j = 0;
            let mut cells = row.split("\t");
            loop {
                if column_index + j >= focus_nav.cols { break; }
                match cells.next() {
                None => break,
                Some(cell) => {
                    data.paste(cell, column_index + j);
                }
            }; j += 1; }
        },
    }; i += 1; }
}
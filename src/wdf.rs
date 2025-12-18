
pub const fn parse_u8(data: &[u8], idx: &mut usize) -> u8 {
    let mut res = 0;
    while data[*idx] >= b'0' && data[*idx] <= b'9' {
        res *= 10;
        res += (data[*idx] - b'0') as u8;
        *idx += 1;
    }
    res
}

pub const fn count_numbers(data: &[u8], offset: usize) -> usize {
    let mut i = offset;
    let mut count = 0;
    let mut still_same:u8 = 0;
    while i < data.len() && data[i] != b'\n' {
        if data[i] != b' ' {
            if still_same == 0 {
                count += 1;
            }
            still_same = 1
        }
        else {
            still_same = 0
        }
        i += 1;
    }
    count
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub height: u8,
    pub ent_id: u8
}

pub fn parse_wdf<const ROWS: usize, const COLS: usize>(wdf_name: &str) -> [[Cell; ROWS]; COLS] {
    let mut cells = [[Cell { height: 0, ent_id: 0 }; ROWS]; COLS];
    let wdf = include_bytes!("../L0.WDF");
    let mut i = 0;
    
    // Parse CH section
    while i < wdf.len() {
        if i + 2 < wdf.len() && wdf[i] == b'C' && wdf[i+1] == b'H' && wdf[i+2] == b'\n' {
            i += 3; // Skip "CH\n"
            
            for col in 0..COLS {
                while i < wdf.len() && wdf[i] == b' ' { i += 1; }
                if i >= wdf.len() || wdf[i] == b'\n' { break; }
                
                for row in 0..ROWS {
                    while i < wdf.len() && wdf[i] == b' ' { i += 1; }
                    if i >= wdf.len() || wdf[i] == b'\n' { break; }
                    cells[col][row].height = parse_u8(wdf, &mut i);
                }
                
                while i < wdf.len() && wdf[i] != b'\n' { i += 1; }
                if i < wdf.len() { i += 1; }
            }
            break; // Done with CH section
        } else {
            i += 1;
        }
    }

    // Reset and parse CE section
    i = 0;
    while i < wdf.len() {
        if i + 2 < wdf.len() && wdf[i] == b'C' && wdf[i+1] == b'E' && wdf[i+2] == b'\n' {
            i += 3; // Skip "CE\n"
            
            for col in 0..COLS {
                while i < wdf.len() && wdf[i] == b' ' { i += 1; }
                if i >= wdf.len() || wdf[i] == b'\n' { break; }
                
                for row in 0..ROWS {
                    while i < wdf.len() && wdf[i] == b' ' { i += 1; }
                    if i >= wdf.len() || wdf[i] == b'\n' { break; }
                    cells[col][row].ent_id = parse_u8(wdf, &mut i);
                }
                
                while i < wdf.len() && wdf[i] != b'\n' { i += 1; }
                if i < wdf.len() { i += 1; }
            }
            break; // Done with CE section
        } else {
            i += 1;
        }
    }
    
    cells
}
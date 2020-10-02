extern crate simple_excel_writer;
use anyhow::Result;
use excel::*;
use simple_excel_writer as excel;

pub struct Excel1 {
    wb: excel::Workbook,
}

pub fn new_excel1(filename: &str) -> Excel1 {
    return Excel1 {
        wb: Workbook::create(filename),
    };
}

impl Excel1 {
    pub fn finish(&mut self) -> Result<()> {
        self.wb.close()?;
        return Ok(());
    }

    pub fn make_dummy_execl(&mut self) -> Result<()> {
        let mut sheet = self.wb.create_sheet("host1");
        self.wb.write_sheet(&mut sheet, |sw| {
            sw.append_row(row!["ID", "English", "Japanese"])?;
            sw.append_row(row!["1", "Apple", "りんご"])?;
            sw.append_row(row!["2", "Banana", "バナナ"])
        })?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_dummy_excel() {
        let mut e1 = new_excel1("./tests/dummy.xlsx");
        e1.make_dummy_execl().expect("ERROR");
        e1.finish().expect("ERROR");
    }
}

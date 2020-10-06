extern crate simple_excel_writer;
use anyhow::Result;
use excel::*;
use simple_excel_writer as excel;

pub struct Excel1 {
    wb: excel::Workbook,
}

impl Excel1 {
    pub fn new(filename: &str) -> Excel1 {
        Excel1 {
            wb: Workbook::create(filename),
        }
    }
    pub fn finish(&mut self) -> Result<()> {
        self.wb.close()?;
        Ok(())
    }

    #[cfg(test)]
    pub fn make_dummy_execl(&mut self) -> Result<()> {
        let mut sheet = self.wb.create_sheet("host1");
        self.wb.write_sheet(&mut sheet, |sw| {
            sw.append_row(row!["ID", "English", "Japanese"])?;
            sw.append_row(row!["1", "Apple", "りんご"])?;
            sw.append_row(row!["2", "Banana", "バナナ"])
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_dummy_excel() {
        let mut e1 = Excel1::new("./tmp/dummy.xlsx");
        e1.make_dummy_execl().expect("ERROR");
        e1.finish().expect("ERROR");
    }
}

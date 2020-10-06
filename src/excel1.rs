extern crate simple_excel_writer;
use crate::pkg;
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
    pub fn add_host(&mut self, host: &str, packages: Vec<pkg::Pkg>) -> Result<()> {
        let mut sheet = self.wb.create_sheet(host);
        self.wb.write_sheet(&mut sheet, |sw| {
            for v in packages.iter().cloned() {
                sw.append_row(row![v.name, v.version, v.arch])?
            }
            Ok(())
        })?;
        Ok(())
    }

    // func (e1 *Excel1) AddHost(host string, packages []Pkg) {
    // 	e1.f.NewSheet(host)
    // 	for i, v := range packages {
    // 		e1.f.SetCellValue(host, fmt.Sprintf("A%d", i+1), v.Name)
    // 		e1.f.SetCellValue(host, fmt.Sprintf("B%d", i+1), v.Version)
    // 		e1.f.SetCellValue(host, fmt.Sprintf("C%d", i+1), v.Arch)
    // 		e1.f.SetCellValue(host, fmt.Sprintf("D%d", i+1), v.ToString())
    // 	}
    // }

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

use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize)]
struct Airport {
    name: String,
    iata_code: String,
    icao_code: String,
    location: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 获取网页内容
    let url = "https://jichang.todaynav.com/jichang_China.html";
    println!("正在获取网页内容...");
    let resp = reqwest::blocking::get(url)?;
    let bytes = resp.bytes()?;

    // 2. 解码GB18030编码的网页内容
    println!("解码网页内容...");
    let html_content = String::from_utf8(bytes.to_vec())?;

    // 3. 解析HTML
    println!("解析HTML内容...");
    let document = Html::parse_document(&html_content);
    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut airports = Vec::new();

    // 4. 提取表格数据
    println!("提取表格数据...");
    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&row_selector).skip(1) { // 跳过表头
            let cells: Vec<_> = row.select(&cell_selector).collect();

            if cells.len() >= 4 {
                let name = cells[0].text().collect::<String>().trim().to_string();
                let iata_code = cells[1].text().collect::<String>().trim().to_string();
                let icao_code = cells[2].text().collect::<String>().trim().to_string();
                let location = cells[3].text().collect::<String>().trim().to_string();

                airports.push(Airport {
                    name,
                    iata_code,
                    icao_code,
                    location,
                });
            }
        }
    }

    // 5. 导出为JSON
    println!("导出数据到JSON文件...");
    let json_data = serde_json::to_string_pretty(&airports)?;
    let mut file = File::create("airports_china.json")?;
    file.write_all(json_data.as_bytes())?;

    println!("成功导出 {} 个机场数据到 airports_china.json", airports.len());
    Ok(())
}
use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
struct Airport {
    name: String,
    city: String,
    iata_code: String,
    icao_code: String,
}

fn main() -> Result<()> {
    let url = "https://www.gjkdwl.com/china/airportcode.html";
    let output_file = "airport_codes.json";

    // 获取网页内容
    let html = fetch_html(url)?;

    // 解析表格数据
    let airports = parse_html_table(&html)?;

    // 导出为JSON
    save_as_json(&airports, output_file)?;

    println!("成功提取 {} 条数据，已保存到 {}", airports.len(), output_file);
    Ok(())
}

fn fetch_html(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let response = client.get(url).send()?;
    response.text().context("Failed to read response text")
}

fn parse_html_table(html: &str) -> Result<Vec<Airport>> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("table").context("Failed to parse table selector")?;
    let row_selector = Selector::parse("tr").context("Failed to parse row selector")?;
    let cell_selector = Selector::parse("td").context("Failed to parse cell selector")?;

    let table = document
        .select(&table_selector)
        .next()
        .context("No table found in HTML")?;

    let mut airports = Vec::new();

    // 跳过表头行（假设第一行是表头）
    for row in table.select(&row_selector).skip(1) {
        let cells: Vec<_> = row.select(&cell_selector).collect();

        if cells.len() >= 4 { // 确保至少有4列数据
            let airport = Airport {
                name: cells[0].text().collect::<String>().trim().to_string(),
                city: cells[1].text().collect::<String>().trim().to_string(),
                iata_code: cells[2].text().collect::<String>().trim().to_string(),
                icao_code: cells[3].text().collect::<String>().trim().to_string(),
            };
            airports.push(airport);
        }
    }

    Ok(airports)
}

fn save_as_json(airports: &[Airport], filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(airports)?;
    fs::write(filename, json)?;
    Ok(())
}

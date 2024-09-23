use scraper::{ElementRef, Html, Selector};
use std::env;

#[derive(Debug)]
struct Driver {
    number: u16,
    forename: String,
    surname: String,
    code: String,
}

#[derive(Debug)]
struct DriverResult {
    position: u16,
    driver: Driver,
}

fn select_result_row_cell_string_value(
    result_row: &ElementRef,
    cell_value_selector: &Selector,
) -> Option<String> {
    let cell_value: String = result_row
        .select(&cell_value_selector)
        .next()?
        .text()
        .collect();
    return Some(cell_value);
}

fn select_result_row_cell_int_value(
    result_row: &ElementRef,
    cell_value_selector: &Selector,
) -> Option<u16> {
    return select_result_row_cell_string_value(result_row, cell_value_selector)?
        .parse::<u16>()
        .ok();
}

fn driver_result_from_result_row(result_row: &ElementRef) -> Option<DriverResult> {
    let driver_position_selector = Selector::parse("td:nth-child(1)").unwrap();
    let driver_number_selector = Selector::parse("td:nth-child(2)").unwrap();
    let driver_forename_selector = Selector::parse("td:nth-child(3) span:nth-child(1)").unwrap();
    let driver_surname_selector = Selector::parse("td:nth-child(3) span:nth-child(2)").unwrap();
    let driver_code_selector = Selector::parse("td:nth-child(3) span:nth-child(3)").unwrap();

    return Some(DriverResult {
        position: select_result_row_cell_int_value(&result_row, &driver_position_selector)?,
        driver: Driver {
            number: select_result_row_cell_int_value(&result_row, &driver_number_selector)?,
            code: select_result_row_cell_string_value(&result_row, &driver_code_selector)?,
            forename: select_result_row_cell_string_value(&result_row, &driver_forename_selector)?,
            surname: select_result_row_cell_string_value(&result_row, &driver_surname_selector)?,
        },
    });
}

fn main() {
    let uri = env::args()
        .nth(1)
        .expect("Usage: result-scraper <race_uri>");
    let body = reqwest::blocking::get(uri)
        .expect("Failed to request URI")
        .text()
        .expect("Failed to get response body");

    let fragment = Html::parse_fragment(&body);
    let result_rows_selector = Selector::parse(".f1-table-with-data tbody tr").unwrap();

    for result_row in fragment.select(&result_rows_selector) {
        let driver_result = driver_result_from_result_row(&result_row)
            .expect("Unable to get driver result from result row: {result_row:?}");

        println!("Found driver result: {driver_result:?}");
    }
}

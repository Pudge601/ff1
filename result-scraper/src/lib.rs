use scraper::{ElementRef, Html, Selector};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, ScrapeError>;

#[derive(Debug)]
pub enum ScrapeError {
    RequestFailure(reqwest::Error),
    ParseError,
}

impl From<reqwest::Error> for ScrapeError {
    fn from(err: reqwest::Error) -> ScrapeError {
        ScrapeError::RequestFailure(err)
    }
}

impl From<ParseIntError> for ScrapeError {
    fn from(_: ParseIntError) -> ScrapeError {
        ScrapeError::ParseError
    }
}

#[derive(Debug)]
pub struct Driver {
    number: u16,
    forename: String,
    surname: String,
    code: String,
}

#[derive(Debug)]
pub struct DriverResult {
    position: u16,
    driver: Driver,
}

fn select_result_row_cell_string_value(
    result_row: &ElementRef,
    cell_value_selector: &Selector,
) -> Result<String> {
    let cell_value: String = result_row
        .select(&cell_value_selector)
        .next()
        .ok_or(ScrapeError::ParseError)?
        .text()
        .collect();
    return Ok(cell_value);
}

fn select_result_row_cell_int_value(
    result_row: &ElementRef,
    cell_value_selector: &Selector,
) -> Result<u16> {
    Ok(select_result_row_cell_string_value(result_row, cell_value_selector)?.parse::<u16>()?)
}

fn driver_result_from_result_row(result_row: &ElementRef) -> Result<DriverResult> {
    let driver_position_selector = Selector::parse("td:nth-child(1)").unwrap();
    let driver_number_selector = Selector::parse("td:nth-child(2)").unwrap();
    let driver_forename_selector = Selector::parse("td:nth-child(3) span:nth-child(1)").unwrap();
    let driver_surname_selector = Selector::parse("td:nth-child(3) span:nth-child(2)").unwrap();
    let driver_code_selector = Selector::parse("td:nth-child(3) span:nth-child(3)").unwrap();

    return Ok(DriverResult {
        position: select_result_row_cell_int_value(&result_row, &driver_position_selector)?,
        driver: Driver {
            number: select_result_row_cell_int_value(&result_row, &driver_number_selector)?,
            code: select_result_row_cell_string_value(&result_row, &driver_code_selector)?,
            forename: select_result_row_cell_string_value(&result_row, &driver_forename_selector)?,
            surname: select_result_row_cell_string_value(&result_row, &driver_surname_selector)?,
        },
    });
}

pub fn scrape_race_results(race_uri: &str) -> Result<Vec<DriverResult>> {
    let body = reqwest::blocking::get(race_uri)?.text()?;

    let fragment = Html::parse_fragment(&body);
    let result_rows_selector = Selector::parse(".f1-table-with-data tbody tr").unwrap();

    let results: Result<Vec<DriverResult>> = fragment
        .select(&result_rows_selector)
        .map(|result_row| driver_result_from_result_row(&result_row))
        .collect();
    return results;
}

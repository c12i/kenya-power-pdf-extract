#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

use anyhow::Context;
use pdf_extract::extract_text;
use regex::Regex;
use serde::Serialize;

lazy_static! {
    static ref DATE_RE: Regex =
        Regex::new(r"DATE:? (Mon|Tues|Wednes|Thurs|Fri|Satur|Sun)day \d{1,2}\.\d{1,2}\.\d{4}",)
            .unwrap();
    static ref TIME_RE: Regex =
        Regex::new(r"TIME:? \d{1,2}\.\d{2} (A|P)\.M\.? [–-] \d{1,2}\.\d{2} (A|P)\.M\.?").unwrap();
}

pub fn extract_text_from_pdf<P>(path: P) -> Result<String, anyhow::Error>
where
    P: AsRef<std::path::Path>,
{
    let junk_regex = Regex::new(
        r"(Interruption of|Electricity Supply|Notice is hereby|That the electricity|\(It  is  necessary|maintenance and upgrade|construction, etc.\)|customers or to replace|For further information|the nearest Kenya Power|Interruption notices|www.kplc.co.ke)"
    ).context("Error building junk regex filter")?;
    let content = extract_text(path).context("Error extracting text from pdf")?;
    let content = content
        .lines()
        .filter(|c| !c.trim().is_empty())
        .filter(|c| !junk_regex.is_match(c))
        .map(|c| c.trim())
        .collect::<String>();
    Ok(content)
}

#[derive(Debug, Serialize)]
pub struct OutagesList {
    data: Vec<OutagesItem>,
}

impl FromStr for OutagesList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split(r"AREA:")
            .map(|v| v.trim())
            .filter(|v| DATE_RE.is_match(v))
            .collect::<Vec<_>>();
        let data = data
            .into_iter()
            .map(|outage_string| {
                outage_string
                    .parse::<OutagesItem>()
                    .expect("Error parsing string to `OutagesItem`")
            })
            .collect::<Vec<OutagesItem>>();
        Ok(OutagesList { data })
    }
}

#[derive(Debug, Serialize)]
pub struct OutagesItem {
    region: String,
    date: String,
    time: String,
    areas: Vec<String>,
}

impl FromStr for OutagesItem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_match = DATE_RE
            .find(s)
            .ok_or_else(|| anyhow::Error::msg("There was an error matching 'DATE_RE'"))?;
        let first_index = date_match.start();
        let outage_string = s.to_string();
        let region = outage_string[..first_index].to_string();
        let date = outage_string[first_index..date_match.end()]
            .replace("DATE: ", "")
            .to_string();
        let time_match = TIME_RE
            .find(s)
            .ok_or_else(|| anyhow::Error::msg("There was an error matching the 'TIME_RE'"))?;
        let time = outage_string[time_match.start()..time_match.end()]
            .replace("TIME", "")
            .replace(":", "")
            .trim()
            .to_string();
        let areas = outage_string[time_match.end()..]
            .split(", ")
            .map(|s| String::from(s.trim()))
            .collect::<Vec<_>>();
        Ok(OutagesItem {
            areas,
            date,
            region,
            time,
        })
    }
}

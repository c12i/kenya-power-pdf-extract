use kenya_power_pdf_extract::{extract_text_from_pdf, OutagesList};

fn main() -> Result<(), anyhow::Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let pdf_text = extract_text_from_pdf(&args[1])?;
    let outages_list = pdf_text.parse::<OutagesList>()?;
    println!("{:#?}", outages_list);
    Ok(())
}

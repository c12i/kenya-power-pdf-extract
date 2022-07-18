use pdf_extract::extract_text;

fn main() {
    let content = extract_text("./files/kenya_power_latest.pdf").expect("Error");
    let content = content
        .lines()
        .filter(|c| !c.trim().is_empty())
        .map(|c| c.trim())
        .collect::<Vec<_>>();
    println!("{:#?}", content);
}

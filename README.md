# Kenya Power Interruptions PDF Extract
Parsing kenya power interruption data from their pdf files into json format

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">🧐 lemme tinker with the pdf file, see if I can parse the data</p>&mdash; collins muriuki (@collinsmuriuki_) <a href="https://twitter.com/collinsmuriuki_/status/1546955159439392768?ref_src=twsrc%5Etfw">July 12, 2022</a></blockquote> 

### Steps

First step is to actually derive the text content from the pdf file into string format. Luckily, rust crate, [`pdf-extract`](https://crates.io/crates/pdf-extract), handles this for us via it's `extract_text` function. PS: storing this data in a `String` type is not the most memory efficient method of going about this I must say, memory usage will be higher the bigger the pdf text size; we can make this compromise for this short demo.

The next bit is where the "fun" begins - make something meaningful from the junky text that we get back. First is to filter out what I consider as `junk` i.e text that doesn't really hold any meaningful data. This functionality is handled by the `extract_text_from_pdf` function

Next step is to break down the massive string into smaller chunks containing isolated outage information for a given area. The approach that was taken to do this was pretty simple, we split the huge string at `"AREA:"`. See the `FromStr` implementation of the `OutagesList`

Now that we have a list of strings, we can figure out how we can handle a single string from the list. The main goal is to establish breakpoints in the remaining string, this was achieved through two regex objects - stored as lazy static variables:
- `DATE_RE` - matches the date of the outage: With this we can derive the date of the outage as well as the string text that comes before the match; at this point we now have the `region` and the `date`
- `TIME_RE` - matches the time range at which the outage will occur as well as the affected areas which is the string patterns that occurs after the date; at this point we now have the `time` and the `areas`.

What is left is to put everything together by creating two structs `OutagesList` and `OutagesItem` with their respective `FromStr` trait implementations. So that we finally have this in our main function:

```rs
use kenya_power_pdf_extract::{extract_text_from_pdf, OutagesList};

fn main() -> Result<(), anyhow::Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let pdf_text = extract_text_from_pdf(&args[1])?;
    let outages_list = pdf_text.parse::<OutagesList>()?;
    println!("{:#?}", outages_list);
    Ok(())
}
```

Output snippet:

```txt
OutagesList {
    data: [
        OutagesItem {
            region: "PART OF KILIMANI, MILIMANI",
            date: "Monday 18.07.2022",
            time: "9.00 A.M. – 5.00 P.M.",
            areas: [
                "Part  of  Jabavu  Rd",
                "Woodlands",
                "DoD  Headquarters",
                "Woodlands  Mosque",
                "Part  ofHurlingum S/Centre",
                "Jabavu Court",
                "Chinese Embassy",
                "Russian Embassy",
                "Sri LankaEmbassy",
                "Jakaya  Kikwete  Rd",
                "Delamere  Flats",
                "Sagret  Hotel",
                "Comfort  Hotel",
                "SwizzHotel",
                "Ralph Bunch Rd",
                "Integrity Centre",
                "Middle East Bank",
                "Heron Portico",
                "PITMAN,Telkom  Plaza",
                "Adak  House  Nairobi  Central  SDA",
                "Nairobi  Area  Police",
                "Medical  &Dentist Board",
                "Lenana Rd & adjacent customers.",
            ],
        },
//...
```

## Local Development

Requires [rust and `cargo` installation](https://www.rust-lang.org/tools/install).

Once that's done run:

```sh
cargo run ./files/kenya_power.pdf
```

Check the [`output`](output) folder for the resulting `stdout` output for both [`kenya_power_latest.pdf`](files/kenya_power_latest.pdf) and [`kenya_power.pdf`](files/kenya_power.pdf) files in the [`files`](files) directory

## Caveats
- Only tested with 4 pdfs files derived from [kplc.co.ke](https://www.kplc.co.ke/category/view/50/planned-power-interruptions) - Some edge cases might not be covered
- Data is only grouped by `AREA` rather than `REGION` - can be fixed, decided to keep things simple for now

Authored by [Collins Muriuki](https://collinsmuriuki.xyz)

*This project is [MIT](LICENSE) licensed*
# Kenya Power Interruptions PDF Extract
Parsing kenya power interruption data from their pdf files into json format

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">🧐 lemme tinker with the pdf file, see if I can parse the data</p>&mdash; collins muriuki (@collinsmuriuki_) <a href="https://twitter.com/collinsmuriuki_/status/1546955159439392768?ref_src=twsrc%5Etfw">July 12, 2022</a></blockquote> 

### Steps

First step is to actually derive the text content from the pdf file into string format. Luckily, rust crate, `pdf-extract`, handles this for us via it's `extract_text` function. PS: storing this data in a `String` type is not the most memory efficient method of going about this I must say, memory usage will be higher the bigger the pdf text size; we can make this compromise for this short demo.

The next bit is where the "fun" begins - make something meaningful from the junky text that we get back. First is to filter out what I consider as `junk` i.e text that doesn't really hold any meaningful data. This functionality is handled by the `extract_text_from_pdf` function

TODO

# August 29, 2022
This is simply a markdown test to see if my function in converting this into HTML correctly.
Here is a second line. There should be a decent amount of space between this, and the filler text below.
Gravida tristique leo. Dis porttitor orci felis ultrices fames sodales tincidunt mollis dignissim tortor lobortis. Pretium morbi Tortor posuere, vel duis pharetra accumsan. Duis placerat sodales dictumst. Erat. Leo tellus hac lobortis. Ante. Bibendum Penatibus. Faucibus turpis dolor viverra, porta ut. Lobortis risus urna facilisi donec fusce tortor curabitur rhoncus feugiat nisl magnis suscipit habitant justo tellus dapibus. Auctor faucibus elit aenean laoreet pharetra bibendum.
## This should be a smaller looking heading
### This should be even smaller lolol
Below is a list of stuff:
- bullet 1
- bullet 2
- bullet 3
- bullet 4
Wow.
Thats a bunch of bullets!!!!
Now heres a numbered list:
1. Item 1
2. Item 2
3. Item 3
4. Item 5
  I'm not sure when I'll add in a table, but I'll do it eventually...
  This is a big long seperation line:

---

Heres some more text...

Heres a useful, `small in line code block` for all your coding needs!

And heres a decently sized code block for more work:

```rs
async fn convert(md_file:&str) -> String {
    // Create a path variable from the filename
    let input_filename = Path::new(md_file);
    // Try to open the file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");
    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();
    // Read the file line-by-line
    let reader = BufReader::new(file);
    let mut ptag: bool = false; // keep track of paragraph enclosures
    let mut htag: bool = false;
    let mut out = String::new();
    for line in reader.lines() {
```

wow, impressive!

Anything else needed?

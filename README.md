# rusty-blog

This was primarily written for me to practice Rust and start prepping for the Rust course I'm taking. I also just wanted a fast, lightweight, safe way to start blogging, without any PHP bloat `\s`. Given the amount of free time you get during summer break, I somehow got this done in like 1.25 days. I'm pretty satisfied with the speed I wrote this. All the `.to_owned()` shows I think I'm getting a better hang of how the safety works in rust... Sort of... I think...😅 
Other than that, Rust is amazing, just love the way it forces you to think, despite the difficulty in the beginning.

I plan to start writing some more higher-level blog posts on things I do for fun... I do a bit too many projects too, so maybe that'll be interesting to read through.

# General usage:
- Have a folder full of random markdown files
- Throw the folder in this webserver structure; it'll automatically convert the MD files to HTML and structure it into an HTML list page, by creation date of the files

# Dev notes:
Actix has a cool logging system... I plan to incorporate useragent/ip fingerprinting counting or something of the like. Maybe some built in filtering if thats possible... Not too sure yet.



# Credits:
- https://dev.to/michaelin007/creating-a-web-page-with-actix-web-rust--2agd
- https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/
- Everyone that helped me with smaller issues in the Rust Discord server!!! Love you all :)

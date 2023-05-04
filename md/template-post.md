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

```rust
fn main() {
    println!("Hello world!");
}
```

Wow, impressive!

Anything else needed?
# My Test File

This is a test file for my parser.

## Heading 2

This is some text under heading 2.

### Heading 3

This is some text under heading 3.

* Item 1
* Item 2
* Item 3

1. First
2. Second
3. Third

[Google](https://www.google.com/)

**Bold text**

*Italic text*

> This is a blockquote.

And here's an image:

![Cat](https://i.imgur.com/P4JLWQO.jpeg)

```js
console.log("Wow! This is really nice....");
```

```cpp
// A cli tool to apply gaussian blur and output an image
// Usage: ./blur <input image> <output image> [blur strength]
#include <iostream>
#include <opencv2/opencv.hpp>

__global__ void applyFilter(const unsigned char *input, unsigned char *output, const unsigned int width, const unsigned int height, const float *kernel, const unsigned int kernelWidth) {
    const unsigned int col = threadIdx.x + blockIdx.x * blockDim.x;
    const unsigned int row = threadIdx.y + blockIdx.y * blockDim.y;
    if(row < height && col < width) {
        const int half = kernelWidth / 2;
        float blur = 0.0;
        float kernelSum = 0.0;
        for(int i = -half; i <= half; i++) {
            for(int j = -half; j <= half; j++) {
                const unsigned int y = max(0, min(height - 1, row + i));
                const unsigned int x = max(0, min(width - 1, col + j));
                const float w = kernel[(j + half) + (i + half) * kernelWidth];
                kernelSum += w;
                blur += w * input[x + y * width];
            }
        }
        output[col + row * width] = static_cast<unsigned char>(blur / kernelSum);
    }
}
```

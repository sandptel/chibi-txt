# chibiTxt
A simple file compressor using Rust.

# 📹 Tutorial:
- https://youtu.be/81om1sAmHGg

# :grey_question: What is it?
Compresses files by using [Huffman encoding](https://en.wikipedia.org/wiki/Huffman_coding).

# :file_folder: What file types?
Right now only txt. However, the code is simple enough to use with pretty much any file type, just requires a little bit of fine-tuning. The idea is to show how to write a simple encoding/decoding program.

# How to use:
cargo run [input_file.txt] [output_key.txt] [output_file.bin]
cargo run [input_file.bin] [input_key.txt] [decoded_file.txt]

# Example:
Encode:
`user@computer:~/ cargo run -- -e "example_txt/war_and_peace.txt" "key.txt" "war_and_peace.bin"`

`File compression percentage: 145.35601197750995`
`Old file size: 3226652 (in bytes)`
`After file size: 2219827 (in bytes)`

Decode:
`user@computer:~/ cargo run -- -d "war_and_peace.bin" "key.txt" "decoded.txt"`

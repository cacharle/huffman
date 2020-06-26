# huffman

Implementation of the [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding?oldformat=true) made for educational purposes.

## Usage

* compress: `cargo run < input_file > output_file.huffman`
* decompress: `cargo run d < input_file.huffman > output_file`

`python3 draft.py [file_name]` to run the python draft.

## File format

Compress to a custom `.huffman` file format which is a header of the huffman coding tree followed by the compressed content.


### Header format

4 byte unsigned int: header size (including this field)
Conversion table where each entry's format is:
    1 byte for the actual byte value
    1 byte for size of representation in bits
    the representation aligned on a 8-bit boundary

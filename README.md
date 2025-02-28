# lineio

Very simple file interface, to parse things a line at a time, skipping blank lines,
and lines with a hash mark in the first column.

Lots of EDA input files are line-by-line formatted, with a fixed structure, and comments
are with the hash mark.  Rather than rebuilding a file reader every time (and having
to include the use descriptions, and whatnot), just a simple wrapper.  Call getline to get
a line, unwrap as needed.  The new function takes a string filename.


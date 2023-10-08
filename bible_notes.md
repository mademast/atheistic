not about like, the content within. the words. but the layout
of this specific collection.

# Heading
it starts with a list of the.. editions? and their books.

You get an edition title on a line, and then the lines
immediately following are the books therein.

EX:
```
The Old Testament *(blah blah)*
The First Book...
The Second Book...

The New Testament *(blah blah)*
The Gospel *(the first one..)*
The Gospel *(the second one...)*
```

To mark the end of the heading you get many newlines *(two or more)*.
It seems that multiple blank lines mark a State Change
*(and we'll call it thus, Genny 1:1)*

NOTE: the state change may always be four blank lines. i will test this.

## Editions
The edition title, state change, book title, state change.

NOTE: that second state change seems to consistently consist of
but just two blank lines.

### Books
As noted above, we get the book title and then two blank lines.

Following the blank lines we get a series of text blocks separated
by a blank line. Most verses start at the beginning of a text block
and are fully contained, but! It seems that this is not consistent.

They might:
- Start the text block and extend the entire block.
- Start the text block, but another verse *(or more!)* start within
the same block.[^1]
- Start the text block, but extend into another text block.

[^1]: I am not well bible-studied enough to know if this is
the same verse containing the other, or if the verse starts when the
next begins.
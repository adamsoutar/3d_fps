# doom-convert

`doom-convert` is a *very* quick and dirty
Python script for converting DOOM maps into
the Rust structures used to define the map
for `3d_fps`.

Please do not take `convert.py` as a sign of
my Python-writing.

It depends on OMGIFOL.

### Note

It actually didn't work. Though:

 - I tested it on an old version of the
 engine, which quickly crashed.
 - `doom-convert` does not 'convert' the
 Player 1 start position, leading you to
 often spawn outside the map.

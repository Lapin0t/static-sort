Some fun with procedural macros. Should implement fast static (short) length
array sorting. Eg `static_sort!(bose_nelson, 20, my_array);`. The code
generated is simply a sequence of compare/swap with fixed indexes. The
compiler should optimize these to conditional move instructions if possible.

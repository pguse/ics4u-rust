# get()

This [game of life](/notes/14-game-of-life/game-of-life.md) project models a **2-dimensional** grid using a _(1-dimensional)_ [vector](notes/05-vectors/vectors.md) in **Rust**. To do this a `get()` function is used to translate between the _(x,y)_ coordinates of the grid and the **index** of the [vector](notes/05-vectors/vectors.md).  This note explains how this function does this translation/calculation.

Imagine we are modelling a 4 x 3 grid _(4 rows and 3 columns)_:

| (0,0) | (1,0) | (2,0) | (3,0) |
| :---: | :---: | :---: | :---: |
| (0,1) | (1,1) | (2,1) | (3,1) |
| (0,2) | (1,2) | (2,2) | (3,2) |

where the x-coordinates increase from left-to-right, and the y-coordinates increase from top-to-bottom, and the origin (0, 0) is in the top-left corner. This can be modelled with a [vector](notes/05-vectors/vectors.md) in Rust with 12 elements ($width * height = 4 * 3 = 12$) as such,

| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:--:|:--:|

## Convert (x,y) to index

The following formula is used to convert from _(x,y)_ coordinates to the index of the vector:

$index = y * width + x$

For example, the position _(3,0)_ should correspond to index 3 in the  [vector](notes/05-vectors/vectors.md).  With $width = 4$,

$index = 0 * 4 + 3$
$index = 3$

while position _(1,2)_ should correspond to index 9. So, 

$index = 2 * 4 + 1$
$index = 9$





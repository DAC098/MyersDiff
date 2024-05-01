# Project: Myers Difference Algorithm

the Myers difference algorithm is an `O(ND)` edit graph that allows for
comparing differences between two given strings and finding the total changes
needed to convert one string to another.

## Use Cases

the most common and widely used use case is comparing differences between files
for version control. `git` uses a form of this algorithm when comparing diffs of
files and storing the changes between commits. this is the main reason for me
choosing this algorithm as it is a proven way of storing versions of a file
without having to store the entire file multiple times (essentially backups of a
file).

## General Overview

when provided with string `ABCABBA`, find the minimum amount of edits (delete or
insert) necessary to create `CBABAC`. to easily visualize the problem we will
create a 2D array with the end result string as the rows and the starting string
as the columns. the numbers along the X and Y axis will indicate coordinates for
moves and our goal is to start from position `(0,0)` and reach `(7,6)`.

```
 ~   A   B   C   A   B   B   A
   +---+---+---+---+---+---+---+ 0
 C |   |   |   |   |   |   |   |
   +---+---+---+---+---+---+---+ 1
 B |   |   |   |   |   |   |   |
   +---+---+---+---+---+---+---+ 2
 A |   |   |   |   |   |   |   |
   +---+---+---+---+---+---+---+ 3
 B |   |   |   |   |   |   |   |
   +---+---+---+---+---+---+---+ 4
 A |   |   |   |   |   |   |   |
   +---+---+---+---+---+---+---+ 5
 C |   |   |   |   |   |   |   |
   +---+---+---+---+---+---+---+ 6
   0   1   2   3   4   5   6   7
```

once we have the array we can start to look for characters that are similar
between the two strings and fill in the cell with a diagonal character.

```
 ~   A   B   C   A   B   B   A
   +---+---+---+---+---+---+---+ 0
 C |   |   | \ |   |   |   |   |
   +---+---+---+---+---+---+---+ 1
 B |   | \ |   |   | \ | \ |   |
   +---+---+---+---+---+---+---+ 2
 A | \ |   |   | \ |   |   | \ |
   +---+---+---+---+---+---+---+ 3
 B |   | \ |   |   | \ | \ |   |
   +---+---+---+---+---+---+---+ 4
 A | \ |   |   | \ |   |   | \ |
   +---+---+---+---+---+---+---+ 5
 C |   |   | \ |   |   |   |   |
   +---+---+---+---+---+---+---+ 6
   0   1   2   3   4   5   6   7
```

the algorithm is based on an idea of moves with movements right being deletions
and movements down being insertions. each move right or down costs 1 while
diagonal moves cost 0. we will want to favor deletes over inserts when
traversing the table.

for example, starting at `(0,0)` -> `(1, 0)` (delete A) -> `(2,0)` (delete B) ->
`(3,1)` (diagonal C) -> `(3,2)` (insert B) to create the string `CB` with a cost
of 3. following this pattern we can continue to create the rest of the path in
order to reach `(7,6)` with the smallest cost possible.

`(0,0)` -> `(1,0)` delete A +1\
`(1,0)` -> `(2,0)` delete B +1\
`(2,0)` -> `(3,1)` keep C   +0\
`(3,1)` -> `(3,2)` insert B +1\
`(3,2)` -> `(4,3)` keep A   +0\
`(4,3)` -> `(5,4)` keep B   +0\
`(5,4)` -> `(6,4)` delete B +1\
`(6,4)` -> `(7,5)` keep A   +0\
`(7,5)` -> `(7,6)` insert C +1

the total cost of taking this particular path is 5. there are other paths that
we can take that have a similar cost.

```
 ~   A   B   C   A   B   B   A
   +---+---+                     0
 C           \
               +                 1
 B             |
               +                 2
 A               \
                   +             3
 B                   \
                       +---+     4
 A                           \
                               + 5
 C                             |
                               + 6
   0   1   2   3   4   5   6   7
```

another way to view this graph is to condense the diagonal moves down. for
example, when we make a move that would be a delete and it leads to a position
that has a diagonal we can change the end point to include the diagonal.
following the snake as it is called.

this is the same as the above steps but with the diagonals condensed down.

`(0,0)` -> `(1,0)` delete A\
`(1,0)` -> `(3,1)` delete B\
`(3,1)` -> `(5,4)` insert B\
`(5,4)` -> `(7,5)` delete B\
`(7,5)` -> `(7,6)` insert C

using this condensed form we can have a trace that looks like this:

```
 0,0 --- 1,0 --- 3,1
                  |
                  |
                 5,4 --- 7,5
                          |
                          |
                         7,6
```

## Myers Algorithm

the previous section states the goal that we want to achieve and walks a
potential path that we can take to achieve that goal with the smallest amount of
edits possible. with that we will go over what Myers algorithm does to achieve
that goal.

we are going to change how we view the problem from the above section to this:

```
                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |
      |
   3  |
      |
   2  |
      |
   1  |
      |
k  0  |
      |
  -1  |
      |
  -2  |
      |
  -3  |
      |
  -4  |
      |
  -5  |
```

the horizontal values indicate the depth (edits) we have reached while the
vertical values are `k` where `k = x - y`. as we increase our depth we will
start `k` from `-depth` and iterate to `depth` by steps of 2. our goal is to
determine the best path we can make from the previous depth to get the highest x
value we can. our `x` values are stored in an array `values` of size
`2 * (len(A) + len(B)) + 1` that is initalized to `0` for every index. in order
to retrieve the previous depths calculated value we will look for 
`values[k + 1]` for inserts and `values[k - 1]` for deletes. when our `k`
value is equal to `-depth` we will look for an insert and when our `k` is equal
to `depth` we will look for a deletion. since we are only storing our `x`
values, we can calculate `y` with `y = x - k`.

```
depth:  0 | k from 0 -> 0
    k:  0 | -depth         | x: 0 y: 0 | empty snake  | setting k[ 0] to x[0]

                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |
      |
   3  |
      |
   2  |
      |
   1  |
      |
k  0  |      0
      |
  -1  |
      |
  -2  |
      |
  -3  |
      |
  -4  |
      |
  -5  |
```

```
depth:  1 | k from -1 -> 1
    k: -1 | -depth         | x: 0 y: 1 | empty snake  | setting k[-1] to x[0]
    k:  1 |  depth         | x: 1 y: 0 | empty snake  | setting k[ 1] to x[1]

                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |
      |
   3  |
      |
   2  |
      |
   1  |            1
      |         /
k  0  |      0
      |         \
  -1  |            0
      |
  -2  |
      |
  -3  |
      |
  -4  |
      |
  -5  |
```

```
depth:  2 | k from -2 -> 2
    k: -2 | -depth         | x: 0 y: 2 | 2 unit snake | setting k[-2] to x[2]
    k:  0 | k - 1 <  k + 1 | x: 1 y: 1 | 1 unit snake | setting k[ 0] to x[2]
    k:  2 |  depth         | x: 2 y: 0 | 1 unit snake | setting k[ 2] to x[3]

                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |
      |
   3  |
      |
   2  |                  3
      |               /
   1  |            1
      |         /     \
k  0  |      0           2
      |         \
  -1  |            0
      |               \
  -2  |                  2
      |
  -3  |
      |
  -4  |
      |
  -5  |
```

```
depth:  3 | k from -3 -> 3
    k: -3 | -depth         | x: 2 y: 5 | 1 unit snake | setting k[-3] to x[3]
    k: -1 | k - 1 >= k + 1 | x: 3 y: 4 | 1 unit snake | setting k[-1] to x[4]
    k:  1 | k - 1 <  k + 1 | x: 3 y: 2 | 2 unit snake | setting k[ 1] to x[5]
    k:  3 |  depth         | x: 4 y: 1 | 1 unit snake | setting k[ 3] to x[5]

                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |
      |
   3  |                        5
      |                     /
   2  |                  3
      |               /     \
   1  |            1           5
      |         /     \
k  0  |      0           2
      |         \
  -1  |            0           4
      |               \     /
  -2  |                  2
      |                     \
  -3  |                        3
      |
  -4  |
      |
  -5  |
```

```
depth:  4 | k from -4 -> 4
    k: -4 | -depth         | x: 3 y: 7 | empty snake  | setting k[-4] to x[3]
    k: -2 | k - 1 <  k + 1 | x: 4 y: 6 | empty snake  | setting k[-2] to x[4]
    k:  0 | k - 1 <  k + 1 | x: 5 y: 5 | empty snake  | setting k[ 0] to x[5]
    k:  2 | k - 1 >= k + 1 | x: 6 y: 4 | 1 unit snake | setting k[ 2] to x[7]
    k:  4 |  depth         | x: 6 y: 2 | 1 unit snake | setting k[ 4] to x[7]

                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |                              7
      |                           /
   3  |                        5
      |                     /
   2  |                  3           7
      |               /     \     /
   1  |            1           5
      |         /     \           \
k  0  |      0           2           5
      |         \
  -1  |            0           4
      |               \     /     \
  -2  |                  2           4
      |                     \
  -3  |                        3
      |                           \
  -4  |                              3
      |
  -5  |
```

```
depth:  5 | k from -5 -> 5
    k: -5 | -depth         | x: 3 y: 8 | empty snake  | setting k[-5] to x[3]
    k: -3 | k - 1 <  k + 1 | x: 4 y: 7 | empty snake  | setting k[-3] to x[4]
    k: -1 | k - 1 <  k + 1 | x: 5 y: 6 | empty snake  | setting k[-1] to x[5]
    k:  1 | k - 1 <  k + 1 | x: 7 y: 6 | empty snake  | setting k[ 1] to x[7]

                       depth
      |      0     1     2     3     4     5
  ----+--------------------------------------
      |
   4  |                              7
      |                           /
   3  |                        5
      |                     /
   2  |                  3           7
      |               /     \     /     \
   1  |            1           5           7
      |         /     \           \
k  0  |      0           2           5
      |         \                       \
  -1  |            0           4           5
      |               \     /     \
  -2  |                  2           4
      |                     \           \
  -3  |                        3           4
      |                           \
  -4  |                              3
      |                                 \
  -5  |                                    3
```

below is the pseudo code for the base algorithm. this will only return the
minimum edit distance found. if a backtrace is desired then a trace will be need
from each depth calculated in order to discern which operations are needed to
edit string `a` to `b`.

```
// the function will take in strings a and b while returning the lowest edit
// distance found for the two strings
function shorest_edit(a: string, b: string) {
    max = a.len + b.len
    values = [0; 2 * max + 1]

    for depth in 0 to max + 1 {
        for k in -depth to (depth + 1) step 2 {
            x = 0

            // arrays are capable of being accessed with a negative index that
            // retrieve the value from the end of the array
            if k == -depth || (k != depth && values[k - 1] < values[k + 1]) {
                x = values[k + 1]
            } else {
                x = values[k - 1] + 1
            }

            y = x - k

            // x and y are guaranteed to be greater or equal to 0 and if they
            // are greater than the length of a and b then we will not check the
            // strings as the index would be out of bounds
            while x < a.len && y < b.len && a[x] == b[y] {
                x += 1
                y += 1
            }

            values[k] = x

            if x >= a.len && y >= b.len {
                return depth
            }
        }
    }

    return max
}
```

the code can be modified to not deal with `-k` index values by offsetting indexs
from the mid point of the `values` array.

### Runtime

from the paper talking about the algorithm the expected runtime is `O(ND)` where
`N` is the sum of the lengths of `A` and `B` and `D` is the size of the minimum
edit script for `A` and `B`. the paper also discusses other improvements to the
algorithm that can improve time and space requirements but are not implemented
here.

## Sources

1. Myers Diff Algoritm - Code & Interactive Visualization\
Published: 2017/06/07 Author: Robert Elder\
url: https://blog.robertelder.org/diff-algorithm/
2. The Myers diff algorithm: part 1 -> 3\
Published: 2017/02/12 Author: James Coglan\
url: https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/\
url: https://blog.jcoglan.com/2017/02/15/the-myers-diff-algorithm-part-2/\
url: https://blog.jcoglan.com/2017/02/17/the-myers-diff-algorithm-part-3/
3. An O(ND) Difference Algorith and Its Variations\
Published: ? Author: Eugene W. Myers\
url: http://www.xmailserver.org/diff2.pdf

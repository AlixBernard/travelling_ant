# Travelling Ant

Short program solving the travelling ant problem. It was designed as a project to learn some basics of Rust.

## Problem definition

An ant is on a grid where each cell of the grid has coordinates (x, y). The ant can only move:
- left (x - 1)
- right (x + 1)
- down (y - 1)
- up (y + 1)
Moreover the ant can only move to cells of which the sum of the digits of the coordinates is less than or equal to `max_sum` (defined by the user), examples:
- (x, y) = (1000, 1207), `max_sum` = 1 + 1 + 2 + 7 = 11
- (x, y) = (901, 1321), `max_sum` = 9 + 1 + 1 + 3 + 2 + 1 = 17

How far can the ant travel, _i.e,_ how many different cells can it reach, considering it starts from `source_cell` (defined by the user).

## Notes

A more time efficient implementation can be achieved using an array instead of a hashmap as the `grid`. Using an array however requires to bound the range of `Cell` than can be reached which may not be space efficient.

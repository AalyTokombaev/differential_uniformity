# Differential Uniformity Checker

This program takes a truth table of a boolean function, and calculates the differential uniformity.

### Usage
You will need to provide a file with two lines:
* First line is the dimension of the function $n$
* Second line with base-10 integers representing the outputs of the function.
* The input for the functions should match the index of the output, i.e if the first number in the line is 0 then the function maps $\vec{0} \mapsto \vec{0}$ with $\vec{0} \in \mathbb{F}_2^n$

See example.tt for an example of how the file should look like.

#### Running the program
`python3 main.py example.tt`

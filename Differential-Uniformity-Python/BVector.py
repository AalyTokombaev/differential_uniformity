class BVector:
    def __init__(self, dim, *args):
        """Creates a new BVector.\n args:
        dim: natural number n > 0, size of the vector.
        args: Elements of the vector. len(args) must be equal to dim."""

        # Check if a vector has a positive size.
        if dim <= 0:
            raise ValueError(f"Can't instantiate a vector of size {dim}")

        # Check if the dimension of the vectors is the same.
        if len(args) != dim:
            raise ValueError(f"Can't instantiate a vector of size {dim} with {len(args)} values!")

        self.elements = list(args)
        self.dim = dim
    

    def __add__(self, other):
        """Returns the sum of two BVectors"""
        # Check if the dimension of the vectors being added is the same 
        if other.dim != self.dim:
            raise ValueError(f"Can't add vectors of different sizes: {self.dim} and {other.dim}")
        
        # Return new vector with the sum of the elements from the added vectors.
        new = [0]*self.dim
        for i in range(len(self.elements)):
            new[i] = (self.elements[i] + other.elements[i]) % 2 # finite field with p = 2  follows mod 2 arithmetic

        # Don't forget to return an actual vector and not a list as this might break...
        return BVector(self.dim, *new) 

    def is_zero_vector(self):
        """Returns True if all elements are 0, False otherwise."""
        return all([i == 0 for i in self.elements]) and not self.elements == []

    def __repr__(self):
        return f"BV:{self.elements}"


    def __eq__(self, other):
        return (self.dim == other.dim) and (self.elements == other.elements)

    def __hash__(self):
        return hash(sum(self.elements + [self.dim]))

from sys import argv
from BVector import BVector as bvec
import sys


# argv[1] is the input fil from the command line
# usage: python3 main.py <filename>
with open(argv[1], 'r') as file:
    dim, outputs = file.readlines()

dim = int(dim)
# convert the outputs to binary, and and pad them so that they all have the correct length
outputs = list(map(lambda i: bin(i)[2:].rjust(dim, '0'),map(int, outputs.strip().split())))
# do the same for the "inputs". Since the file only gives outputs, the index of the output is the input
inputs = list(map(lambda i: bin(i)[2:].rjust(dim, '0'), range(len(outputs))))

# convert inputs and outpits to BVectors to make math easier.
inputs = list(map(lambda e: bvec(dim, *list(map(int, e))), inputs))
outputs = list(map(lambda e: bvec(dim, *list(map(int, e))), outputs))


# Create a lookup table for our function
F = dict(zip(inputs, outputs))

def main():
    # So the max is at least 2 in theory.
    max = 0
    status = 0
    global inputs

    for A in inputs[1:]:
        for B in inputs[1:]:
            count = 0
            for X in inputs:
                if status % 100000 == 0:
                    o = f"status: {100*round(status/((2**dim)**3),3)}%\n"
                    sys.stdout.write("\033[K")
                    sys.stdout.write(o)
                    sys.stdout.write("\033[F")
                    sys.stdout.flush()
                if (F.get(X + A) + F.get(X)) == B:
                    count += 1
                status += 1
            if count > max:
                max = count

    print()
    print(max)

if __name__ == '__main__':
    main()

        

        
        
    


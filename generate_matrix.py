import random
import sys

matrix_size = sys.argv[1]
matrix_max_value = int((2 ** 32) ** 0.5)

for i in range(0, int(matrix_size)):
    for j in range(0, int(matrix_size)):
        print(str(random.randint(0, matrix_max_value)) + ",", end='')
    print("")

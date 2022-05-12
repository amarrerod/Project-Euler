

import numpy as np


def loop_matrix_in_direction(matrix, i_range, j_range, i_factor=1, j_factor=1):
    max_sum = 0
    for i in i_range:
        for j in j_range:
            a = [matrix[i + (inc * i_factor)][j + (inc * j_factor)]
                 for inc in range(4)]
            product = int(np.product(a))
            if product > max_sum:
                max_sum = product

    return max_sum


if __name__ == "__main__":
    matrix = np.loadtxt('data.txt', usecols=range(20))
    diagonal_right = loop_matrix_in_direction(matrix, range(17), range(17))
    diagonal_left = loop_matrix_in_direction(
        matrix, range(3, 17), range(3, 20), j_factor=-1)
    down = loop_matrix_in_direction(matrix, range(17), range(20), j_factor=0)
    up = loop_matrix_in_direction(matrix, range(
        19, 3, -1), range(20), i_factor=-1, j_factor=0)

    right = loop_matrix_in_direction(
        matrix, range(20), range(17), i_factor=0, j_factor=1)
    left = loop_matrix_in_direction(matrix, range(
        20), range(19, 3, -1), i_factor=1, j_factor=-1)

    max_sum = max([diagonal_left, diagonal_right, up, down, left, right])
    # Solution should be 70600674
    # # Diagonal right
    # for i in range(17):
    #     for j in range(17):
    #         a = [matrix[i + inc][j + inc] for inc in range(4)]
    #         print(f'Analysing: {a}')
    #         product = int(np.product(a))
    #         if product > max_sum:
    #             print(f'The greatest product is: {a} = {product}')
    #             max_sum = product

    # # Diagonal left
    # for i in range(3, 17):
    #     for j in range(3, 20):
    #         a = [matrix[i + inc][j - inc] for inc in range(4)]
    #         print(f'Analysing: {a}')
    #         product = int(np.product(a))
    #         if product > max_sum:
    #             print(f'The greatest product is: {a} = {product}')
    #             max_sum = product

    # # Down
    # for i in range(17):
    #     for j in range(20):
    #         a = [matrix[i + inc][j] for inc in range(4)]
    #         print(f'Analysing: {a}')
    #         product = int(np.product(a))
    #         if product > max_sum:
    #             print(f'The greatest product is: {a} = {product}')
    #             max_sum = product

    # # Up
    # for i in range(19, 3, -1):
    #     for j in range(20):
    #         a = [matrix[i - inc][j] for inc in range(4)]
    #         print(f'Analysing: {a}')
    #         product = int(np.product(a))
    #         if product > max_sum:
    #             print(f'The greatest product is: {a} = {product}')
    #             max_sum = product

    # # Right
    # for i in range(20):
    #     for j in range(17):
    #         a = [matrix[i][j + inc] for inc in range(4)]
    #         print(f'Analysing: {a}')
    #         product = int(np.product(a))
    #         if product > max_sum:
    #             print(f'The greatest product is: {a} = {product}')
    #             max_sum = product

    #  # Left
    # for i in range(20):
    #     for j in range(19, 3, -1):
    #         a = [matrix[i][j - inc] for inc in range(4)]
    #         print(f'Analysing: {a}')
    #         product = int(np.product(a))
    #         if product > max_sum:
    #             print(f'The greatest product is: {a} = {product}')
    #             max_sum = product

    print(max_sum)

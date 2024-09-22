import numpy as np


class Sudoku:
    def __init__(self):
        self.field = np.zeros((9, 9), dtype=np.int32)

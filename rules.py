from types import *


class Rules:
    def __init__(self):
        self.rules = {
            "matmul": MatmulRule,
        }

    def apply(self, name, *args):
        rule = self.rules[name]
        return rule.apply(*args)

class MatmulRule:
    @staticmethod
    def apply(a, b):
        if a.cols != b.rows:
            raise TypeError("Invalid dimensions")
        return MatrixType(a.rows, b.cols)

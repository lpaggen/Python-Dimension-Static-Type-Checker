from custom_types import MatrixType


class Rules:
    def __init__(self):
        self.rules = {
            "matmul": Matmul,
        }

    def apply(self, name, *args):
        rule = self.rules[name]
        return rule.apply(*args)

class Matmul:
    @staticmethod
    def apply(a, b):
        if a.cols != b.rows:
            raise TypeError(
                f"Cannot multiply matrices with shapes "
                f"({a.rows}, {a.cols}) and ({b.rows}, {b.cols}): "
                f"inner dimensions must match "
                f"({a.cols} != {b.rows})"
            )
        return MatrixType(a.rows, b.cols)

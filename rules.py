from custom_types import MatrixType


class Rules:
    def __init__(self):
        self.rules = {
            "matmul": MatmulRule,
            "add": AddRule
        }

    def apply(self, name, *args):
        rule = self.rules[name]
        return rule.apply(*args)

class MatmulRule:
    @staticmethod
    def apply(ltype, rtype, out_shape):
        return [
            ltype.cols == rtype.rows,
            out_shape[0] == ltype.rows,
            out_shape[1] == rtype.cols
        ]

class AddRule:
    @staticmethod
    def apply(ltype, rtype, out_shape):
        return [
            ltype.rows == rtype.rows,
            ltype.cols == rtype.cols,
            out_shape[0] == ltype.rows,
            out_shape[1] == rtype.cols
        ]


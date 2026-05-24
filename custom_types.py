class Type:
    pass

class Scalar(Type):
    def __init__(self, value):
        self.value = value

class MatrixType(Type):
    def __init__(self, rows, cols):
        self.rows = rows
        self.cols = cols

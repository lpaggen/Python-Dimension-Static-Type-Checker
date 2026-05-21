class Type:
    pass

class IntType(Type):
    pass

class MatrixType(Type):
    def __init__(self, rows, cols):
        self.rows = rows
        self.cols = cols

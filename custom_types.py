class Type:
    pass

class ScalarType(Type):
    def __init__(self, value):
        self.value = value

class MatrixType(Type):
    def __init__(self, rows, cols):
        self.rows = rows
        self.cols = cols

class VarType():
    def __init__(self, name):
        self.name = name

    def __str__(self):
        return self.name

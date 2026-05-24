class TensorLiteralExpr:
    def __init__(self, shape):
        self.shape: tuple[int, int] = shape

class RandnExpr:
    def __init__(self, shape):
        self.shape = shape

class MatMulExpr:
    def __init__(self, left, right):
        self.left = left
        self.right = right

class AddExpr:
    def __init__(self, left, right):
        self.left = left
        self.right = right

class TensorDecl:
    def __init__(self, name, shape, value):
        self.name = name
        self.shape = shape
        self.value = value

    def __repr__(self):
        return f"TensorDecl({self.name}, shape=[{self.shape[0]}, {self.shape[1]}])"

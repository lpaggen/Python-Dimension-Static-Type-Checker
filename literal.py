from typing import List, Union


class Literal:
    def __init__(self):
        pass

class IntegerLiteral(Literal):
    def __init__(self, value: int):
        self.value=value

class FloatLiteral(Literal):
    def __init__(self, value: float):
        self.value=value

class TensorLiteral(Literal):
    def __init__(self, value: List[Union[IntegerLiteral, FloatLiteral, NameRef]]):
        self.value=value

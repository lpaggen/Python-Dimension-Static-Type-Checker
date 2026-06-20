from typing import List, Union
from literal import IntegerLiteral
from identifier import Identifier


class Type:
    def __init__(self):
        pass

class TensorType(Type):
    def __init__(self, dims: List[Union[IntegerLiteral, Identifier]]):
        self.dims=dims
        
class IntegerType(Type):
    def __init__(self):
        pass
    
class FloatType(Type):
    def __init__(self):
        pass

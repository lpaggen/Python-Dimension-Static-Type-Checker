from typing import List, Union
from literal import IntegerLiteral


class TensorType:
    def __init__(self, dims: List[Union[IntegerLiteral, NameRef]]):
        self.dims=dims

from tensor_type import TensorType


class DeclIR:
    def __init__(self):
        pass
    
class TensorDeclIR(DeclIR):
    def __int__(self, id, name: str, annotation: TensorType, value):
        
class DimDeclIR(DeclIR):
    pass
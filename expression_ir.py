from ir_node import IRNode
from operators import Operator


class IdentifiedIRNode(IRNode):
    """
    Parent class to IRNode objects which need unique identifiers
    """
    def __init__(self, id, span=None):
        super().__init__(span)
        self.span=span
        self.id=id

class ListIR(IRNode):
    def __init__(self, elements: list[IRNode], span=None):
        super().__init__(span=span)
        self.span=span
        self.elements=elements

    def __repr__(self):
        return "ListIR<" + str(self.contents) + ">"

class IntegerIR(IRNode):
    def __init__(self, value: int, span=None):
        super().__init__(span=span)
        self.span=span
        self.value=value

    def __repr__(self):
        return str(self.value)

class FloatIR(IRNode):
    def __init__(self, value: float, span=None):
        super().__init__(span=span)
        self.span=span
        self.value=value

    def __repr__(self):
        return str(self.value)
    
class BinOpIR(IRNode):
    def __init__(self, left: IRNode, right: IRNode, op: Operator, span=None):
        super().__init__(span=span)
        self.op=op
        self.left=left
        self.right=right
        self.span=span

class CallExprIR(IRNode):
    def __init__(self, callee, args: ListIR, span=None):
        super().__init__(span=span)
        self.span=span
        self.callee=callee
        self.args=args

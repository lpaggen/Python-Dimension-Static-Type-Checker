from common.span import SourceSpan
from .ir_node import IRNode


class TupleIR(IRNode):
    def __init__(self, elements: tuple[IRNode], span=None):
        super().__init__(span=span)
        self.span = span
        self.elements = elements

    def __repr__(self):
        return "TupleIR<" + str(self.contents) + ">"

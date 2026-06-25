from common.span import SourceSpan
from .expr_ir import IRNode


class ListIR(IRNode):
    def __init__(self, elements: list[IRNode], span=None):
        super().__init__(span=span)
        self.span = span
        self.elements = elements

    def __repr__(self):
        return "ListIR<" + str(self.contents) + ">"

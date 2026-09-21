from common.span import SourceSpan


class IRNode:
    def __init__(self, span: SourceSpan):
        """
        Parent class of all IRNode objects
        """
        self.span = span

    def to_proto(self):
        pass

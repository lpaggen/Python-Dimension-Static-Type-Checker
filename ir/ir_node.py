class IRNode:
    def __init__(self, span=None):
        """
        Parent class of all IRNode objects
        """
        self.span = span

    def to_proto(self):
        pass

class Symbol:
    def __init__(self, name, node=None):
        self.name = name
        self.node = node
        self.type = None
        self.dependencies = set()
        self.resolved = False

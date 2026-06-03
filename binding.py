class Binding:
    def __init__(self, target: str, dependencies: set):
        self.target = target
        self.dependencies = dependencies

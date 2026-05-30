class Binding:
    def __init__(self, is_resolved: str, dependencies: set, belongs_to):
        self.is_resolved = is_resolved
        self.dependencies = dependencies
        self.belongs_to = belongs_to

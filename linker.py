import ast


class ImportRef:
    def __init__(self, module, name, alias, is_from):
        self.module = module
        self.name = name
        self.alias = alias
        self.is_from = is_from

    def __repr__(self):
        return f"ImportRef(module={self.module}, name={self.name}, alias={self.alias}, is_from={self.is_from})"


class ImportCollector(ast.NodeVisitor):
    def __init__(self, module_name: str):
        self.module_name = module_name
        self.imports = []

    def visit_Import(self, node):
        for alias in node.names:
            self.imports.append(
                ImportRef(
                    module=alias.name,
                    name=None,
                    alias=alias.asname,
                    is_from=False
                )
            )
        return

    def visit_ImportFrom(self, node):
        for alias in node.names:
            self.imports.append(
                ImportRef(
                    module=node.module,
                    name=alias.name,
                    alias=alias.asname,
                    is_from=True
                )
            )
        return


class Linker:
    def __init__(self, project_scopes, project_imports):
        self.project_scopes = project_scopes
        self.project_imports = project_imports
        self.dependencies = {}
        self.project_modules = set(project_scopes.keys())

    def static_link_files(self):
        for module, imports in self.project_imports.items():
            for importref in imports:
                module_name = importref.module + ".py"
                if module_name in self.project_modules:
                    if module not in self.dependencies:
                        self.dependencies[module] = set()
                    self.dependencies[module].add(module_name)

        for module in self.dependencies:
            print(self.dependencies[module])
            print(self.project_scopes["ex3.py"])

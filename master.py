"""
this parses every file, etc.
"""


import os
from dependency_graph import DependencyGraph
from semantic_visitor import SemanticBuilder
import ast
from symbol_table import Env


def parse_project(dir: str):
    project_scopes = {}
    for file in os.listdir(dir):
        if not file.endswith(".py"):
            continue
        with open(os.path.join(dir, file), "r") as f:
            code = f.read()
        tree = ast.parse(code)
        env = Env()  # same context to be used in all visitors
        builder = SemanticBuilder(env)
        scopes = builder.visit(tree)
        project_scopes[file] = scopes

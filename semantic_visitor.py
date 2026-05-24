import ast
from dimension import DimDecl, Dim
from tensor_decl import TensorDecl
from torch_parser import TorchOpParser
from symbol_table import Env
from custom_types import MatrixType


class SemanticBuilder(ast.NodeVisitor):  # for now only support torch.tensor and int, extend to include other things
    def __init__(self, env: Env):
        self.ir = []  # just a list of nodes (declarations for now, can extend later)
        self.env = env  # !! env is passed to EVERY visitor, this, constraint, IR, everyone needs the SAME one
        self.torchParser = TorchOpParser()

    def visit_FunctionDef(self, node: ast.FunctionDef):  # ensure new scope !!!!!!!!
        self.env.push()
        self.generic_visit(node)  # recursive, comes back to parse inside of function params
        self.env.pop()

    def visit_AnnAssign(self, node: ast.AnnAssign) -> None:
        annotation = node.annotation
        identifier = node.target.id
        value = node.value

        # only parse that which is claimed to be integer, ignore the rest, they can't represent dimensions
        if isinstance(annotation, ast.Name) and (annotation.id == "int"):  # we are in a 'x: int = 4' type of statement
            value = Dim.toDim(node.value)
            self.ir.append(DimDecl(identifier, value))
            self.env.declare(identifier, value)  # is this valid? Dim not really a type
            return

        if (  # will break if using some alias, to fix
        isinstance(annotation, ast.Subscript)
        and isinstance(annotation.value, ast.Attribute)
        and annotation.value.attr == "Tensor"
        and isinstance(annotation.value.value, ast.Name)
        and annotation.value.value.id == "torch"
        ):  # to do fix this to rely on Dim rather than int and str
            dims = [Dim.toDim(i) for i in annotation.slice.elts]  # List(elts...) -> [Dim, Dim]
            tensorValue = self.torchParser.parse(value)
            self.ir.append(TensorDecl(identifier, (dims[0], dims[1]), tensorValue))  # tensorValue encodes information relevant for constraints, shapes
            self.env.declare(identifier, MatrixType(dims[0], dims[1]))
            return

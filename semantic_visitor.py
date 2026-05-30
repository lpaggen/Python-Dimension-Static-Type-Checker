import ast
from dimension import DimDecl, Dim, UnknownDim, SymDim
from tensor_decl import TensorDecl
from torch_parser import TorchOpParser
from symbol_table import Env
from custom_types import MatrixType
from binding import Binding


# TODO run this on each separate file, populate a global dependency graph
class SemanticBuilder(ast.NodeVisitor):  # for now only support torch.tensor and int, extend to include other things
    def __init__(self, env: Env):
        self.ir = []  # just a list of nodes (declarations for now, can extend later)
        self.env = env  # !! env is passed to EVERY visitor, this, constraint, IR, everyone needs the SAME one
        self.torchParser = TorchOpParser()

    def visit_FunctionDef(self, node: ast.FunctionDef):  # ensure new scope !!!!!!!!
        self.env.push()
        self.generic_visit(node)  # recursive, parse inside of function params
        self.env.pop()

    def visit_ClassDef(self, node):
        self.env.push()
        self.generic_visit(node)
        self.env.pop()

    def visit_AnnAssign(self, node: ast.AnnAssign) -> None:
        annotation = node.annotation
        identifier = node.target.id
        value = node.value

        # only parse that which is claimed to be integer, ignore the rest, they can't represent dimensions
        if isinstance(annotation, ast.Name) and (annotation.id == "int"):  # we are in a 'x: int = 4' type of statement
            value = Dim.toDim(node.value) if value is not None else SymDim(identifier)
            self.ir.append(DimDecl(identifier, value))
            self.env.declare(identifier, value)
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
            self.ir.append(TensorDecl(identifier, (dims[0], dims[1]), tensorValue))  # encodes information relevant for constraints, shapes
            self.env.declare(identifier, MatrixType(dims[0], dims[1]))
            return

    def visit_Assign(self, node: ast.Assign) -> None:  # no type annotation
        """
        Visits a node declared as follows: A = torch.tensor([[2], [4], [5]])
        """
        identifier = node.targets[0].id
        value = node.value

        if isinstance(value, ast.Call) and (value.func.value.id == "torch"):  # calling torch.some_method
            self._resolve_dependencies(identifier, value)
            tensorValue = self.torchParser.parse(value)
            self.ir.append(TensorDecl(identifier, (UnknownDim(), UnknownDim()), tensorValue))  # shape is empty, infer it in next pass
            self.env.declare_unresolved(identifier, tensorValue)  # ! unresolved declaration, next pass solves it
            return

    def _resolve_dependencies(self, id: str, node) -> Binding:
        is_resolved = True
        if isinstance(node, ast.Call):
            dependencies = set([i.id for i in node.args])
            for name in dependencies:
                if self.env.lookup(name) is None:
                    is_resolved = False
        print(id, dependencies, is_resolved)
        return Binding(is_resolved=is_resolved, dependencies=dependencies, belongs_to=node)

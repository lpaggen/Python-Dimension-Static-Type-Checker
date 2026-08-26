from typing import List
from ir.stmt.decl_ir import DeclIR
from ir.ir_node import IRNode
from ir.metadata.symbol_ir import SymbolIR
from ir.metadata.scope_ir import ScopeIR
from ir.stmt.import_ir import ImportIR
from ir.stmt.stmt_ir import stmt_to_proto
from generated import _pb2
from dataclasses import dataclass


@dataclass
class ProgramIR:
    module_name: str
    file_path: str
    scopes: List[ScopeIR]
    symbols: List[SymbolIR]
    imports: List[ImportIR]
    decls: List[DeclIR]
    body: List[IRNode]

    def to_proto(self):
        proto = _pb2.ProgramIR(
            module_name=self.module_name,
            file_path=self.file_path,
        )

        proto.scopes.extend([s.to_proto() for s in self.scopes])
        proto.symbols.extend([s.to_proto() for s in self.symbols])
        proto.imports.extend([i.to_proto() for i in self.imports])
        proto.decls.extend([stmt.to_proto() for stmt in self.decls])
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])

        return proto

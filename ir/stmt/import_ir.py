from common.span import SourceSpan
from typing import Union
from generated import _pb2
from .stmt_ir import StmtIR
from dataclasses import dataclass


@dataclass
class ImportIR(StmtIR):
    id: int
    local_symbol_id: int
    scope_id: int
    kind: object
    module_name: str
    imported_name: str | None
    alias: str | None
    relative_level: int
    span: SourceSpan | None
    def to_proto(self):
        proto = _pb2.ImportIR(
            id=self.id,
            local_symbol_id=self.local_symbol_id,
            scope_id=self.scope_id,
            kind=int(self.kind),
            module_name=self.module_name,
            relative_level=self.relative_level,
        )

        if self.imported_name is not None:
            proto.imported_name = self.imported_name

        if self.alias is not None:
            proto.alias = self.alias

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto

    def to_stmt_proto(self):
        stmt = _pb2.StmtIR()
        stmt.import_stmt.CopyFrom(self.to_proto())
        return stmt

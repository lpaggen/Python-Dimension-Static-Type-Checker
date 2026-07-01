from common.span import SourceSpan
from typing import Union
from generated import _pb2
from .stmt_ir import StmtIR


class ImportIR(StmtIR):
    def __init__(
        self,
        id: int,
        local_symbol_id: int,
        scope_id: int,
        kind: str,
        module_name: str,
        imported_name: Union[str, None],
        alias: Union[str, None],
        relative_level: int,
        span: Union[SourceSpan, None],
    ):
        super().__init__(span=span)
        self.id = id
        self.local_symbol_id = local_symbol_id
        self.scope_id = scope_id
        self.kind = kind
        self.module_name = module_name
        self.imported_name = imported_name
        self.alias = alias
        self.relative_level = relative_level
        self.span = span

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
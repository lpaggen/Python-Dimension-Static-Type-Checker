import ast
from dataclasses import dataclass
from generated import _pb2


@dataclass
class SourceSpan:
    file: str
    lineno: int
    col_offset: int
    end_lineno: int | None
    end_col_offset: int | None

    @staticmethod
    def span(node: ast.AST, file_path: str) -> "SourceSpan":
        return SourceSpan(
            file=str(file_path),
            lineno=getattr(node, "lineno", 0),
            col_offset=getattr(node, "col_offset", 0),
            end_lineno=getattr(node, "end_lineno", None),
            end_col_offset=getattr(node, "end_col_offset", None),
        )

    def to_proto(self):
        proto = _pb2.SourceSpan(
            file=self.file,
            lineno=self.lineno or 0,
            col_offset=self.col_offset or 0,
        )
        if self.end_lineno is not None:
            proto.end_lineno = self.end_lineno
        if self.end_col_offset is not None:
            proto.end_col_offset = self.end_col_offset
        return proto

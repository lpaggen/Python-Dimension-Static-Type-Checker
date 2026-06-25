from enum import Enum


class IRNodeKind(Enum):
    MODULE = "MODULE"
    ASSIGN = "ASSIGN"
    ANNASSIGN = "ANNASSIGN"
    AUGASSIGN = "AUGASSIGN"
    ...

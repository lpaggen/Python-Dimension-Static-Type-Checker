from enum import Enum


class ScopeKind(Enum):
    MODULE = "MODULE"
    FUNCTION = "FUNCTION"
    CLASS = "CLASS"
    BLOCK = "BLOCK"

class SymbolKind(Enum):
    UNKNOWN = "UNKNOWN"
    MODULE_ALIAS = "MODULE_ALIAS"
    VARIABLE = "VARIABLE"
    FUNCTION = "FUNCTION"
    CLASS = "CLASS"
    PARAM = "PARAM"

class BindingKind(Enum):
    ASSIGN = "ASSIGN"
    ANNASSIGN = "ANNASSIGN"

class ImportKind(Enum):
    MODULE = "MODULE"
    FROM = "FROM"
    MODULE_ALIAS = "MODULE_ALIAS"

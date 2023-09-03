"""
Manages the symbol table
"""


class SymbolTable:
    def __init__(self):
        self.table = {
            "SCREEN": 16384,
            "KBD": 24576,
            "SP": 0,
            "LCL": 1,
            "ARG": 2,
            "THIS": 3,
            "THAT": 4,
        }
        for i in range(16):
            self.table[f"R{i}"] = i

        self.mem_counter = 16

    def add_label(self, name: str, value: int):
        """
        Used in first pass for label symbols: given a label name and
        instruction number, store it in the symbol table.
        """
        self.table[name] = value

    def maybe_add_and_return(self, name: str) -> int:
        """
        Used in second pass for both label and variable symbols: if the
        symbol does not exist in the table, it is a new variable. Add it to
        the table and increment the memory counter. In any case, return the
        value stored in the table for the symbol name.
        """
        if name not in self.table:
            self.table[name] = self.mem_counter
            self.mem_counter += 1
        return self.table[name]

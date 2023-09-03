from typing import TextIO

from code_parser import CodeParser
from instruction_parser import InstructionParser
from symbol_table import SymbolTable


class Assembler:
    def __init__(self, infile: TextIO, outfile: TextIO):
        self.infile = infile
        self.outfile = outfile
        self.code_parser = CodeParser()
        self.instruction_parser = InstructionParser()
        self.symbol_table = SymbolTable()
        self.lines = infile.readlines()

    def assemble(self):
        self.set_label_symbols()
        binary_output = self.parse_instructions()
        self.outfile.writelines(binary_output)

    def set_label_symbols(self):
        """
        First pass: traverse the entire file, and when a label definition is
        found, add the label to the symbol table.
        """
        counter = 0
        for line in self.lines:
            line = self.strip_comment(line)
            line = line.strip()
            if not line:
                continue
            if line.startswith("("):
                label = line.lstrip("(").rstrip(")")
                self.symbol_table.add_label(label, counter)
            else:
                counter += 1

    def parse_instructions(self) -> list[str]:
        """
        Second pass: parse all the A and C instructions in the file. Return
        the binary output.
        """
        binary_output = []
        for line in self.lines:
            line = self.strip_comment(line)
            line = line.strip()
            if not line or line.startswith("("):
                continue
            if line.startswith("@"):
                parsed_instruction = self.parse_a_instruction(line)
            else:
                parsed_instruction = self.parse_c_instruction(line)
            binary_output.append(parsed_instruction + "\n")

        return binary_output

    def parse_a_instruction(self, instruction: str) -> str:
        address = instruction.lstrip("@")
        if address.isnumeric():
            address = int(address)
        else:
            address = self.symbol_table.maybe_add_and_return(address)
        binary = format(address, "b").zfill(16)
        return binary

    def parse_c_instruction(self, instruction: str) -> str:
        parsed_instruction = self.instruction_parser.parse(instruction)
        comp = self.code_parser.comp(parsed_instruction.comp)
        dest = self.code_parser.dest(parsed_instruction.dest)
        jump = self.code_parser.jump(parsed_instruction.jump)
        return "111" + comp + dest + jump

    @staticmethod
    def strip_comment(line: str) -> str:
        return line.split("//")[0]

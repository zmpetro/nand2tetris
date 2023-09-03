"""
Unpacks each C instruction into its underlying fields
"""

from dataclasses import dataclass


@dataclass
class ParsedInstruction:
    comp: str
    dest: str | None = None
    jump: str | None = None


class InstructionParser:
    @staticmethod
    def parse(instruction: str) -> ParsedInstruction:
        if "=" in instruction and ";" in instruction:
            dest, comp_jump = instruction.split("=")
            comp, jump = comp_jump.split(";")
        elif "=" in instruction and ";" not in instruction:
            dest, comp = instruction.split("=")
            jump = None
        elif "=" not in instruction and ";" in instruction:
            comp, jump = instruction.split(";")
            dest = None

        return ParsedInstruction(comp, dest, jump)

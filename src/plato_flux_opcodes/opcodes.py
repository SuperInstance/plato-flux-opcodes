"""FLUX bytecode opcode definitions and registry."""
from dataclasses import dataclass, field
from enum import Enum

class OpcodeCategory(Enum):
    CONTROL = "control"
    TILE = "tile"
    ROOM = "room"
    MEMORY = "memory"
    MATH = "math"
    I2I = "i2i"
    FORGE = "forge"

@dataclass
class Opcode:
    code: int
    name: str
    category: OpcodeCategory
    operands: int = 0
    description: str = ""
    cycles: int = 1

class OpcodeRegistry:
    def __init__(self):
        self._opcodes: dict[int, Opcode] = {}
        self._names: dict[str, int] = {}
        self._load_defaults()

    def _load_defaults(self):
        defaults = [
            (0x00, "HALT", OpcodeCategory.CONTROL, 0, "Stop execution"),
            (0x01, "PUSH", OpcodeCategory.CONTROL, 1, "Push value onto stack"),
            (0x02, "POP", OpcodeCategory.CONTROL, 0, "Pop value from stack"),
            (0x10, "TILE_CREATE", OpcodeCategory.TILE, 3, "Create tile: content, domain, confidence"),
            (0x11, "TILE_STORE", OpcodeCategory.TILE, 1, "Store tile by ID"),
            (0x12, "TILE_SEARCH", OpcodeCategory.TILE, 1, "Search tiles by query"),
            (0x13, "TILE_SCORE", OpcodeCategory.TILE, 1, "Score tile by ID"),
            (0x14, "TILE_GHOST", OpcodeCategory.TILE, 1, "Ghost tile by ID"),
            (0x20, "ROOM_ENTER", OpcodeCategory.ROOM, 1, "Enter room by name"),
            (0x21, "ROOM_LEAVE", OpcodeCategory.ROOM, 0, "Leave current room"),
            (0x22, "ROOM_NAV", OpcodeCategory.ROOM, 1, "Navigate to connected room"),
            (0x30, "MEM_LOAD", OpcodeCategory.MEMORY, 1, "Load from memory address"),
            (0x31, "MEM_STORE", OpcodeCategory.MEMORY, 1, "Store to memory address"),
            (0x40, "I2I_SEND", OpcodeCategory.I2I, 2, "Send message to agent"),
            (0x41, "I2I_RECV", OpcodeCategory.I2I, 0, "Receive next message"),
            (0x50, "FORGE_TRAIN", OpcodeCategory.FORGE, 1, "Train for N steps"),
            (0x51, "FORGE_EMIT", OpcodeCategory.FORGE, 0, "Emit training artifact"),
            (0x60, "SNAP", OpcodeCategory.MATH, 0, "Snap vector to Pythagorean manifold"),
            (0x61, "VERIFY", OpcodeCategory.MATH, 0, "Verify holonomy constraint"),
        ]
        for code, name, cat, ops, desc in defaults:
            self.register(Opcode(code=code, name=name, category=cat, operands=ops, description=desc))

    def register(self, opcode: Opcode):
        self._opcodes[opcode.code] = opcode
        self._names[opcode.name] = opcode.code

    def get(self, code: int) -> Opcode:
        return self._opcodes.get(code)

    def get_by_name(self, name: str) -> Opcode:
        code = self._names.get(name)
        return self._opcodes.get(code) if code is not None else None

    def by_category(self, category: OpcodeCategory) -> list[Opcode]:
        return [o for o in self._opcodes.values() if o.category == category]

    @property
    def stats(self) -> dict:
        cats = {}
        for o in self._opcodes.values():
            cats[o.category.value] = cats.get(o.category.value, 0) + 1
        return {"total": len(self._opcodes), "categories": cats}

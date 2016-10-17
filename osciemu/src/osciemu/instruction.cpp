#include "osciemu/instruction.h"
#include "osciemu/memory.h"

namespace osciemu {
  Instruction::Instruction()
    : Instruction(0, 0, 0, 0) {}

  Instruction::Instruction(uint32_t a, uint32_t b, uint32_t t, uint32_t j)
    : operand_a(a), operand_b(b), target(t), jmp(j) {}

  void Instruction::WriteToMemory(MemoryInterface& m, uint32_t addr) const {
    WriteIntToMemory(m, addr+ 0, operand_a);
    WriteIntToMemory(m, addr+ 4, operand_b);
    WriteIntToMemory(m, addr+ 8, target);
    WriteIntToMemory(m, addr+12, jmp);
  }

  Instruction Instruction::ReadFromMemory(MemoryInterface& m, uint32_t addr) {
    auto operand_a = ReadIntFromMemory(m, addr+ 0);
    auto operand_b = ReadIntFromMemory(m, addr+ 4);
    auto target    = ReadIntFromMemory(m, addr+ 8);
    auto jmp       = ReadIntFromMemory(m, addr+12);

    return Instruction(operand_a, operand_b, target, jmp);
  }

  void Instruction::Execute(MemoryInterface& m, uint32_t& ip) {
    auto inst = Instruction::ReadFromMemory(m, ip);
    auto a = ReadIntFromMemory(m, inst.operand_a);
    auto b = ReadIntFromMemory(m, inst.operand_b);
    WriteIntToMemory(m, inst.target, a-b);
    if (a-b <= 0) {
      // Round to the next biggest multiple of `Instruction::Size` if its
      // not multiple already.
      ip = (inst.jmp + Instruction::Size - 1) & ~(Instruction::Size-1);
    } else {
      ip += Size;
    }
  }

  bool operator==(const Instruction& lhs, const Instruction& rhs) {
    return
      lhs.operand_a == rhs.operand_a &&
      lhs.operand_b == rhs.operand_b &&
      lhs.target == rhs.target &&
      lhs.jmp == rhs.jmp;
  }

  bool operator!=(const Instruction& lhs, const Instruction& rhs) {
    return !(lhs == rhs);
  }
}
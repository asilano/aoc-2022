defmodule Day01 do
  def part_one(inventory) do
    inventory
    |> inventory_per_elf()
    |> Enum.map(&elf_calories/1)
    |> Enum.map(&total_elf_calories/1)
    |> Enum.max()
  end

  def part_two(inventory) do
    inventory
    |> inventory_per_elf()
    |> Enum.map(&elf_calories/1)
    |> Enum.map(&total_elf_calories/1)
    |> Enum.sort()
    |> Enum.reverse()
    |> Enum.take(3)
    |> Enum.sum()
  end

  defp total_elf_calories(elf_calorie_list) do
    Enum.sum(elf_calorie_list)
  end

  defp elf_calories(elf_inventory) do
    elf_inventory |> String.split() |> Enum.map(&String.to_integer/1)
  end

  defp inventory_per_elf(inventory) do
    String.split(inventory, "\n\n")
  end
end

data = to_string(File.read!("data.txt"))
IO.puts(Day01.part_one(data))
IO.puts(Day01.part_two(data))

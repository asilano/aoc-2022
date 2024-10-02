defmodule Day04 do
  def part_one(list) do
    list
    |> list_to_assignment_pairs()
    |> Enum.count(fn {left, right} ->
      (left.first in right && left.last in right) ||
        (right.first in left && right.last in left)
    end)
  end

  def part_two(list) do
    list
    |> list_to_assignment_pairs()
    |> Enum.count(fn {left, right} -> !Range.disjoint?(left, right) end)
  end

  defp list_to_assignment_pairs(list) do
    list
    |> list_to_elf_pair()
    |> Enum.map(&pair_to_assignments/1)
  end

  defp list_to_elf_pair(list) do
    String.split(list)
  end

  defp pair_to_assignments(pair) do
    with [left_str, right_str] <- String.split(pair, ",", parts: 2),
         [left_start, left_end] <- String.split(left_str, "-", parts: 2),
         [right_start, right_end] <- String.split(right_str, "-", parts: 2),
         left = Range.new(String.to_integer(left_start), String.to_integer(left_end)),
         right = Range.new(String.to_integer(right_start), String.to_integer(right_end)) do
      {left, right}
    end
  end
end

data = to_string(File.read!("data.txt"))
IO.inspect(Day04.part_one(data))
IO.inspect(Day04.part_two(data))

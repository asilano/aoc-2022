defmodule Day03 do
  def part_one(inventory) do
    inventory
    |> rucksacks_from_list()
    |> Enum.map(&rucksack_compartments/1)
    |> Enum.map(fn [left, right] -> duplicated_letter(left, right) end)
    |> Enum.map(fn letter_str -> List.first(String.to_charlist(letter_str)) end)
    |> Enum.map(&priority/1)
    |> Enum.sum()
  end

  def part_two(inventory) do
    inventory
    |> rucksacks_from_list()
    |> Enum.map(&String.codepoints/1)
    |> Enum.chunk_every(3)
    |> Enum.map(&threeway_common_letter/1)
    |> Enum.map(fn letter_str -> List.first(String.to_charlist(letter_str)) end)
    |> Enum.map(&priority/1)
    |> Enum.sum()
  end

  defp rucksacks_from_list(inventory) do
    String.split(inventory)
  end

  defp rucksack_compartments(rucksack) do
    items = String.codepoints(rucksack)
    count = round(length(items) / 2)
    Enum.chunk_every(items, count)
  end

  defp duplicated_letter([], _), do: nil

  defp duplicated_letter([left_head | left_rest], right) do
    if contains?(right, left_head) do
      left_head
    else
      duplicated_letter(left_rest, right)
    end
  end

  defp threeway_common_letter([alpha, beta, gamma]) do
    Enum.find(alpha, fn letter -> contains?(beta, letter) && contains?(gamma, letter) end)
  end

  defp contains?([needle | _], needle), do: true
  defp contains?([], _), do: false
  defp contains?([_ | haystack], needle), do: contains?(haystack, needle)

  defp priority(letter) when letter in ?A..?Z, do: letter - ?A + 27
  defp priority(letter) when letter in ?a..?z, do: letter - ?a + 1
end

data = to_string(File.read!("data.txt"))
IO.inspect(Day03.part_one(data))
IO.inspect(Day03.part_two(data))

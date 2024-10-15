defmodule Day08 do
  def part_one(data) do
    parse_forest(data)
    |> count_visible()
  end

  def part_two(data) do
    parse_forest(data)
    |> then(&Enum.map(&1, fn {loc, height} -> scenic_score(&1, loc, height) end))
    |> Enum.max()
  end

  defp parse_forest(data) do
    forest_lists =
      data
      |> String.split("\n")
      |> Enum.map(&String.codepoints/1)

    for {row, row_ix} <- Enum.with_index(forest_lists),
        {tree, col_ix} <- Enum.with_index(row),
        into: %{} do
      {{row_ix, col_ix}, tree}
    end
  end

  defp count_visible(forest) do
    Enum.count(forest, fn {{row, col}, height} ->
      visible_from?(forest, {row - 1, col}, height, fn {row, col} -> {row - 1, col} end) ||
        visible_from?(forest, {row + 1, col}, height, fn {row, col} -> {row + 1, col} end) ||
        visible_from?(forest, {row, col - 1}, height, fn {row, col} -> {row, col - 1} end) ||
        visible_from?(forest, {row, col + 1}, height, fn {row, col} -> {row, col + 1} end)
    end)
  end

  defp visible_from?(forest, location, height, next_loc_fn)

  defp visible_from?(forest, loc, _, _) when not is_map_key(forest, loc), do: true

  defp visible_from?(forest, loc, height, next_loc_fn) do
    forest[loc] < height && visible_from?(forest, next_loc_fn.(loc), height, next_loc_fn)
  end

  defp scenic_score(forest, {row, col}, height) do
    view_distance(forest, {row - 1, col}, height, fn {row, col} -> {row - 1, col} end, 0) *
      view_distance(forest, {row + 1, col}, height, fn {row, col} -> {row + 1, col} end, 0) *
      view_distance(forest, {row, col - 1}, height, fn {row, col} -> {row, col - 1} end, 0) *
      view_distance(forest, {row, col + 1}, height, fn {row, col} -> {row, col + 1} end, 0)
  end

  defp view_distance(forest, loc, _, _, distance) when not is_map_key(forest, loc),
    do: distance

  defp view_distance(forest, loc, height, next_loc_fn, distance) do
    if forest[loc] < height do
      view_distance(forest, next_loc_fn.(loc), height, next_loc_fn, distance + 1)
    else
      distance + 1
    end
  end
end

sample = "30373
25512
65332
33549
35390"

data = to_string(File.read!("data.txt"))

IO.inspect(Day08.part_one(data))
IO.inspect(Day08.part_two(data))

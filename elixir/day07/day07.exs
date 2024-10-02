defmodule Day07 do
  def part_one(data) do
    data
    |> String.splitter("\n")
    |> parse_filesystem()
    |> Map.values()
    |> Enum.filter(&(&1 <= 100_000))
    |> Enum.sum()
  end

  def part_two(data) do
    data
    |> String.splitter("\n")
    |> parse_filesystem()
    |> then(fn object_map ->
      min_size = object_map[["/"]] - 40_000_000

      Enum.filter(Map.values(object_map), &(&1 >= min_size))
      |> Enum.min()
    end)
  end

  defp parse_filesystem(lines) do
    lines
    |> Enum.reduce({%{}, []}, &process_filesystem_line/2)
    |> clean_up_dir_stack()
  end

  defp process_filesystem_line(line, acc)

  defp process_filesystem_line(
         "$ cd ..",
         {object_map, dir_stack = [current_dir, parent_dir | rest]}
       ) do
    {Map.update!(object_map, [parent_dir | rest], &(&1 + object_map[dir_stack])),
     [parent_dir | rest]}
  end

  defp process_filesystem_line("$ cd " <> subdir, {object_map, dir_stack}) do
    {Map.put(object_map, [subdir | dir_stack], 0), [subdir | dir_stack]}
  end

  defp process_filesystem_line("$ ls", acc), do: acc

  defp process_filesystem_line("dir " <> _, acc), do: acc

  defp process_filesystem_line(file_record, {object_map, dir_stack}) do
    filesize = file_record |> String.split() |> List.first() |> String.to_integer()
    {Map.update!(object_map, dir_stack, &(filesize + &1)), dir_stack}
  end

  defp clean_up_dir_stack({object_map, ["/"]}), do: object_map

  defp clean_up_dir_stack(map_stack_tuple),
    do: clean_up_dir_stack(process_filesystem_line("$ cd ..", map_stack_tuple))
end

sample = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
data = to_string(File.read!("data.txt"))

IO.inspect(Day07.part_one(sample))
IO.inspect(Day07.part_one(data))
IO.inspect(Day07.part_two(sample))
IO.inspect(Day07.part_two(data))

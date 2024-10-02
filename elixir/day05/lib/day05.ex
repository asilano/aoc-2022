defmodule Step do
  defstruct [:count, :from, :to]

  def instructions_from(manual) do
    manual
    |> String.split("\n", trim: true)
    |> Enum.map(&from/1)
  end

  def from(line) do
    [count, from, to] =
      Regex.run(~r"move (\d+) from (\d+) to (\d+)", line, capture: :all_but_first)

    %Step{
      count: String.to_integer(count),
      from: String.to_integer(from),
      to: String.to_integer(to)
    }
  end
end

defmodule Dock do
  defstruct stacks: []

  def from(picture) do
    [ground | rows] = Enum.reverse(String.split(picture, "\n"))
    num_stacks = length(String.split(ground, " ", trim: true))

    %Dock{
      stacks:
        for stack_ix <- 0..(num_stacks - 1) do
          Enum.filter(
            for row <- Enum.reverse(rows) do
              case String.at(row, 4 * stack_ix + 1) do
                blank when blank == " " or is_nil(blank) -> nil
                letter -> letter
              end
            end,
            &(!is_nil(&1))
          )
        end
    }
  end

  def tops(%Dock{stacks: stacks}) do
    stacks |> Enum.map(&List.first/1) |> Enum.join()
  end

  def process_phase_1(%Dock{} = dock, steps) do
    for step <- steps, reduce: dock do
      dock -> apply_step_phase_1(dock, step)
    end
  end

  defp apply_step_phase_1(%Dock{stacks: stacks}, %Step{count: count, from: from, to: to}) do
    stacks =
      for _ <- 1..count, reduce: stacks do
        stacks ->
          stacks
          |> List.update_at(to - 1, fn to_stack ->
            [List.first(Enum.at(stacks, from - 1)) | to_stack]
          end)
          |> List.update_at(from - 1, fn [_ | rest] -> rest end)
      end

    %Dock{stacks: stacks}
  end

  def process_phase_2(%Dock{} = dock, steps) do
    for step <- steps, reduce: dock do
      dock -> apply_step_phase_2(dock, step)
    end
  end

  defp apply_step_phase_2(%Dock{stacks: stacks}, %Step{count: count, from: from, to: to}) do
    with from_stack = Enum.at(stacks, from - 1),
         {moved, remain} = Enum.split(from_stack, count),
         to_stack = Enum.at(stacks, to - 1) do
      stacks =
        stacks
        |> List.replace_at(from - 1, remain)
        |> List.replace_at(to - 1, moved ++ to_stack)

      %Dock{stacks: stacks}
    end
  end
end

defmodule Day05 do
  def part_one(input) do
    {dock, steps} = parse_input(input)
    Dock.process_phase_1(dock, steps) |> Dock.tops()
  end

  def part_two(input) do
    {dock, steps} = parse_input(input)
    Dock.process_phase_2(dock, steps) |> Dock.tops()
  end

  defp parse_input(input) do
    [dock_pic, steps] = String.split(input, "\n\n", parts: 2)
    {Dock.from(dock_pic), Step.instructions_from(steps)}
  end
end

sample = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"

data = to_string(File.read!("data.txt"))

IO.inspect(Day05.part_one(data))
IO.inspect(Day05.part_two(data))

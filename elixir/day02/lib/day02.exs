defmodule Day02 do
  def part_one(guide) do
    guide
    |> split_rounds()
    |> Enum.map(&interpret_round_part_1/1)
    |> Enum.map(&Enum.sum/1)
    |> Enum.sum()
  end

  def part_two(guide) do
    guide
    |> split_rounds()
    |> Enum.map(&interpret_round_part_2/1)
    |> tap(&IO.inspect/1)
    |> Enum.map(&Enum.sum/1)
    |> Enum.sum()
  end

  defp split_rounds(guide) do
    String.split(guide, "\n")
  end

  defp interpret_round_part_1(round) do
    [code_opponent, code_self] = String.split(round)

    opponent_move = interpret_opponent(code_opponent)

    self_move =
      case code_self do
        # rock
        "X" -> 1
        # paper
        "Y" -> 2
        # scissors
        "Z" -> 3
      end

    [self_move, compare(self_move, opponent_move)]
  end

  defp interpret_round_part_2(round) do
    [code_opponent, code_win] = String.split(round)

    opponent_move = interpret_opponent(code_opponent)

    win_type =
      case code_win do
        # rock
        "X" -> 0
        # paper
        "Y" -> 3
        # scissors
        "Z" -> 6
      end

    [move_for_win_type(opponent_move, win_type), win_type]
  end

  defp interpret_opponent(opponent_move) do
    case opponent_move do
      # rock
      "A" -> 1
      # paper
      "B" -> 2
      # scissors
      "C" -> 3
    end
  end

  defp compare(self, opponent) do
    case Integer.mod(self - opponent, 3) do
      0 -> 3
      1 -> 6
      2 -> 0
    end
  end

  defp move_for_win_type(opponent, win_type) do
    move =
      Integer.mod(
        opponent +
          case win_type do
            0 -> 2
            3 -> 0
            6 -> 1
          end,
        3
      )

    if move == 0, do: 3, else: move
  end
end

data = to_string(File.read!("data.txt"))
IO.puts(Day02.part_one(data))
IO.puts(Day02.part_two(data))

defmodule Day06 do
  def part_one(data) do
    find_marker(data, 4)
  end

  def part_two(data) do
    find_marker(data, 14)
  end

  defp find_marker(data, window_length) do
    data
    |> String.codepoints()
    |> Stream.chunk_every(window_length, 1)
    |> Enum.find_index(fn window -> length(window) == length(Enum.uniq(window)) end)
    |> then(fn ix -> ix + window_length end)
  end
end

data =
  "rhghwwsmsgmgsmmmlzljllrddvsvhvhhnvvrcccwhhvgghchwhvwvcvrrrgtgrgdggfdgffshsllvvslsppglgvgwgswwcbcwbwrrbjbwjjtgjjdzdfdhfddrmmhqhpqhhqghhzssqzqccbwwffzvzffvtftddrbrtrrcjrrmmbmrrrlrppplmplmppfzpfpvpqvpqqdndpndnmnwmwjmwmwbbrhhmzmffwfqwfqwffppvfpvffnwwwwcbwbhbccvmmplmplmmlmplpprbpbllmhlhzhffngfnntrrsrprrvlllvjvmjvvppsrrfnrfnrfrfwrffrsfrflfltfllpfprrthhhnhqnnfwwcttvzzddqsddwpddnrnttvjttjjdjvdjvjnvnrvvchhschhlffjggtqgqnnvrnrsnrsnsvspsnncppvwvvtmmcqmcmscmsmwmvmpvvlqqmbqqlwwtgtztgttpjjbfjjzhzthhzssffwqqdmdcdlclrclrccbmmqjqmqcmmnnsvnntpphhdpdhhtbtdtffhzfhhpwhppwbppfspphdhggzqgzzffdsdfsszcscpscsqcssffhjhnhhlqqdbqqdmdsmddrfddndldgglvglvlggqlggjngjnnbffthhjbjpjnjmnmgmqqbjqjzjqzzzjffwttcnnzffmllcncmnnthnncttpgpnnjdjfftmmvqmqsqggfrfjjqbjbvvshvhphfhhljlfjjvcclzclctcmmfwfvwfvwwjvwvjjplpbpqqdvqqbhbdbnnsndssgffgsglssmfmjjfrjfrfttljldjdffjjdnjjlbbnrntrtfrttmhhndnsdsffgqqrpplvvfwvfvsfszzfzhfzzlttlctllrzrggvwvhhgnnlmnlmlpppdldqqdpdlplrpppjspspzzhphccgjjpmmwvwbwssnpssrzrwrhhmzhmhsmhmttrqrvqvcclggptpjpzpccltctppflfjffrtrjrhjhffhjfjbfjfqfqzffdtthwwjwzjzggsgzgqqjdqjddvgvsvnnqwqgwgtttfvfrfwrfwfwmwmqwmmmbzmzvzfvfrfvvgvtvptpbpwbwrbwrwffzzfgzfgfcfzcffjdjjvrvnnfmmtztffsggjcjzcjcssscnscnssmmwgwbbljlnjndnrrqwqjjjbnnmbmcmbcmbbvvmtvmmfcmcjcdcnnzfzfzqfzfvvmbvmmndnbnsnnlddqvvsnngvnnrttfbtbqqnwqqzfzfzccfmmgrrsbbhvvgjjhfhssjmmsjjzzbqblqlfqllwhlwwvvstvtfvttnncjjzcjcjrrsvrssmfsfczbrzrvscvmcmrjpzwhcqfrrzbljnmqlzbzqtmhrshlrjjpvhnsvtlhqggqwppsjpszmqwfqmlwbqzwcrggrvfbvztnflwvbrqcrqbcllswvsvhwjzpldgphwptfdnlgdlnbttjfzrdcfvpdhlssfsljvdjmwddbnrpqnnqlfdfdspbnjwqjwrgtnrftsqcfjpmqwgwhttggjwzvgbwlhmtmmjlwhssrgshzpbcnstzlqdshdhjfgqlsmqqhpwbscsjhfbhprvhmftqngjgdbcvfldqgqsjqjfdmcvsflwzflsjfssnjpbwffnsfcnrdsphbjpgghmcthgnzmpgppqjdvbztvhnwqzndntcpjdtwwhvsmgdcthpssszrqcbntgsznpghmbqddpqscntjprlwzhbzhjtwzbwwcldwdgsttzmtnstjnngzrgvhncdbgqnllfzbthldztsdwsngjzprbvrnbzrsghlgssbqfnbvhnhzwmmmtncvsdngdtwcbjlnnzbnlrnmrvnvsjnvzdqnggmsvljlvjznwdszcmblhrsjvczpnlhsmqjsmwhbjbplbtqsgqjdllhncwdgbvzwnmqvndcbfhnvtjnzmvjhwzvdldhgfwbqqzcnbflfnwntlhmqgdhmrgwqcpmsvfbmwhbtsbdlhcnbcbswvdfffgjvddrbpzpcwsrsjnfvmzhlvbdnwttnqrmzbnnntbfvptrlhjhwjcsbnhvtwtwzvgfnzjplthqjbsjjwzdtqqvblnbvgcmvrmnvmwfrhcqgvrcjlfzdlpbfvncbtfgvnsflbjzqqhczcmtbwqmrppmptfgzvfbmcslwlfrfpvvnvvnwfvvmmdzmmtjsgqdfhngtphtlfjqrtljgnthgnbbqfrnpfmpwhpzdvzmtswwdvcnpsdcqwjdwlvbsbmlwdsjbcbgcrfljshlvpngfmsrzlfhtfqgwbcctnzzhnqhdmqzdwthftwtmpbcmqvdcdtgvltbzmszzwwmhzlfvbdqnhjqgdmstsnhftcwzvvbmnhwvgqzscwcdjbdgfmvpjdzctwqwltbwjlgcblnnhpnmggbmvqpqtgqjzspgqzvcvsdbvjgjfzdzhfpbzjqljjcgldzgnlmtjcmfgdbqgglvjqrppwmhccvqzvsrjjvfhjprwdsqsnszfprznljtcsrtqhcrpljfrccflmbpvqtzgmzhjrlbnrmmsmmjbtzwpglqgdvvvjvnfzmplsmvlvcnjshvjwntclwgpznnzwhjssgdcjbzrmsgnfgcgphrhfvrfhzwdcvsplhbmqwhpmjvqlmschznbqblvhtqfgtdggmncndhhplnzjphccjmlmtdqnmnlnpnfqdstljqnsqbrjrtspvrwvdmwzlgdmsfvctzgtmgqhqqrzpbgplzcfdqnzhsqrbcvhsccshnnpvvrpvqzqsgzgmpzfvvrrcvhdtntnsqnjrbzlbzmpgwdqzbhtlrrhbwdqjlsfgdhmvgmgbqhwvljmmqfllqvvrznrlftgzjdcgtstjffqmgvffpvtctzpdqjfnmlcdzscntctmqhrtmhrlhbjzttrcvcnlhsrvtpbmdchhntnnpnzlvqqnsrjcmblcvphqgwshnjmplgvnbsmmdzgqcpqztjhhgjvtlbpdpdwlwmmfrdgcvzfbgvbgpbjnwsssvhszwplcgvpgjwdrwngbcdjwvlsfhqlrqzgrzpfgjstqfdbrpqdvrlgdwqcsrgvhctznjjlzsmzctsqtfnhhlpjgnltssglmlwshfbrgmjqbvsmqwvszdfsvhmtrfjgwjctpsmgzzjbpwsztnnvzrhwvvmhdpgdmwzsjprhlgzcdvhznlfgjqvcwqrplcfvzmthsdsnrtfvnlrmvwplmbdvdggmlvpgdgzhvzmvzwmptzsnfrcrjspccmqjpjmhjqgrjbdcdjbzjmphmcdvjqtmdshhjrqgjgsnpzfbfgpjpczwzvmclgzgztlvzmdbwgncnndjwhhhjnhtjdmcnrmnqbmjdrdcmtvcsmftqcfhsvhsfjmtzjpnwffggpfqlqmzlbhnnhbtgzfgnjvdzmvthqjrhzbwvhcjzcsmsvsctrqbltpcrpjjnsbjdbfjqfcbpcgcwtqsflmlwprjcwlmcjjgsfdpwcqvhjpsgvdgsfnscnbzsrmrbbvdrlltzplbvgqsdnplcvbhddbtmwnfmvqhqdlrtrmrmzmhlccgwgmbdppjqdjtwmvdzfsbsggrfstjwjpjnljffwffmqncfnthnhglwvsgvzmgbzhtdfpfmdwmcldthvsnqnptpmhqctblgfsszhcfbvcrggjdhthqvvvlldshvqwmvdtfslrhzvgdfwztrczdjgcfcgtmwnphqthlgpfnrqwcgpzwnlgdvsnvzftlnlfflfsmjzhrhqjctsbvtccwbfsdrnbhszzjhqndvwcsmffnstnfdfwpbgfztjmjngdczzlgpscjtshpmmmzlnqndsttbdgfjqcvbqlphwhlhgcvjbhjmtrfzlgpwdnvzrllndbhvhlngvhlszzdcrdgvrmjwcvhhtbhnjmdzgctqnpdlrnqjzbchjtcsggsczlgmvtqvzmsqvtrhtvdmzlcdddfnbvbsnrzvgzfqjtbhjqhdznrhbfbqwtnwvrfqsznbqfzfzfgmhvjjsgbbdbdtzswwlnfrq"

IO.inspect(Day06.part_one(data))
IO.inspect(Day06.part_two(data))

# https://leetcode.com/problems/maximum-product-of-three-numbers/


# @param {Integer[]} nums
# @return {Integer}
def maximum_product(nums)
    max = 0

    # 3 (+) from end 
    # 2 (-) from start + 1 (+) from end
    # 3 (-) closest to 0
                # 1 (-), 2 (+)
    # 1 (-), 0 (=),

    results = []
    strategy_1 = []
    strategy_2 = []
    strategy_3 = []
    strategy_4 = []
    strategy_5 = []

    nums.each do |num|
        if num > 0

            if !strategy_1[2] || num > strategy_1[2]
                strategy_1 << num
                strategy_1 = strategy_1.sort { |a, b| b <=> a }[0..2]
            end

            if !strategy_2[0] || num > strategy_2[0]
                strategy_2[0] = num
            end

        elsif num < 0
            if !strategy_2[2] || num < strategy_2[2]
                if !strategy_2[1] || num < strategy_2[1]
                    strategy_2.insert(1, num)
                else
                    strategy_2.insert(2, num)
                end
                strategy_2 = strategy_2[0..2]
            end

            if !strategy_3[2] || num > strategy_3[2]
                strategy_3 << num
                strategy_3 = strategy_3.sort { |a, b| b <=> a }[0..2]
            end

        else
            strategy_5 = [0, 0 ,0]
        end
    end

    results << strategy_1.inject(:*) if strategy_1.compact.size == 3
    results << strategy_2.inject(:*) if strategy_2.compact.size == 3
    results << strategy_3.inject(:*) if strategy_3.compact.size == 3
    results << strategy_5.inject(:*) if strategy_5.compact.size == 3

    results.max
end


# @param {Integer[]} nums
# @return {Integer}
def maximum_product(nums)
    max = 0

    # 3 (+) from end 
    # 2 (-) from start + 1 (+) from end
    # 3 (-) closest to 0
    # 1 (-), 2 (+)
    # 1 (-), 0 (=),

    results = []
    strategy_1 = []
    strategy_2 = []
    strategy_3 = []
    strategy_4 = []
    strategy_5 = []

    nums.each do |num|
        if num > 0

            if !strategy_1[2] || num > strategy_1[2]
                strategy_1 << num
                strategy_1 = strategy_1.sort { |a, b| b <=> a }[0..2]
            end

            if !strategy_2[0] || num > strategy_2[0]
                strategy_2[0] = num
            end

            if !strategy_4[2] || num < strategy_4[2]
                if !strategy_4[1] || num < strategy_4[1]
                    strategy_4.insert(1, num)
                else
                    strategy_4.insert(2, num)
                end
                strategy_4 = strategy_4[0..2]
            end
        elsif num < 0
            if !strategy_2[2] || num < strategy_2[2]
                if !strategy_2[1] || num < strategy_2[1]
                    strategy_2.insert(1, num)
                else
                    strategy_2.insert(2, num)
                end
                strategy_2 = strategy_2[0..2]
            end

            if !strategy_3[2] || num > strategy_3[2]
                strategy_3 << num
                strategy_3 = strategy_3.sort { |a, b| b <=> a }[0..2]
            end

            if !strategy_4[0] || num > strategy_4[0]
                strategy_4[0] = num
            end
        else
            strategy_5 = [0, 0 ,0]
        end
    end

    results << strategy_1.inject(:*) if strategy_1.compact.size == 3
    results << strategy_2.inject(:*) if strategy_2.compact.size == 3
    results << strategy_3.inject(:*) if strategy_3.compact.size == 3
    results << strategy_4.inject(:*) if strategy_4.compact.size == 3
    results << strategy_5.inject(:*) if strategy_5.compact.size == 3

    results.max
end








# @param {Integer[]} nums
# @return {Integer}
def maximum_product(nums)
    max = 0

    # 3 (+) from end 
    # 2 (-) from start + 1 (+) from end
    # 3 (-) closest to 0
    # 1 (-), 2 (+)
    # 1 (-), 0 (=),

    results = []
    strategy_1 = []
    strategy_2 = []
    strategy_3 = []
    strategy_4 = []
    strategy_5 = []

    # nums.each do |num|
    nums.sort.each do |num|
        if num > 0
            if !strategy_1[2] || num > strategy_1[-1] 
                strategy_1.insert(0, num)
                strategy_1 = strategy_1[0..2]
            end

            if !strategy_2[0] || num > strategy_2[0]
                strategy_2[0] = num
            end

            if !strategy_5[2] || num < strategy_5[2] 
                strategy_5.insert(1, num)
                strategy_5 = strategy_5[0..2]
            end
        elsif num < 0
            if !strategy_2[2] || num < strategy_2[2] 
                strategy_2.insert(1, num)
                strategy_2 = strategy_2[0..2]
            end

            if !strategy_3[2] || num > strategy_3[-1]
                strategy_3.insert(0, num)
                strategy_3 = strategy_3[0..2]
            end

            if !strategy_5[0] || num > strategy_5[0]
                strategy_5[0] = num
            end
        else
            strategy_5 = [0, 0 ,0]
        end
    end


    results << strategy_1.inject(:*) if strategy_1.compact.size == 3
    results << strategy_2.inject(:*) if strategy_2.compact.size == 3
    results << strategy_3.inject(:*) if strategy_3.compact.size == 3
    results << strategy_4.inject(:*) if strategy_4.compact.size == 3
    results << strategy_5.inject(:*) if strategy_5.compact.size == 3

    max = results.max

    # p strategy_1
    # p strategy_2
    # p strategy_3
    # p strategy_4
    # p strategy_5
    # p results
    # p max 

    max
end




############################# AI

def maximum_product(nums)
  min1 = min2 = Float::INFINITY
  max1 = max2 = max3 = -Float::INFINITY

  nums.each do |n|
    if n <= min1
      min2 = min1
      min1 = n
    elsif n < min2
      min2 = n
    end

    if n >= max1
      max3 = max2
      max2 = max1
      max1 = n
    elsif n >= max2
      max3 = max2
      max2 = n
    elsif n > max3
      max3 = n
    end
  end

  [min1 * min2 * max1, max1 * max2 * max3].max
end























-1000 1000


1 2 3 4 -> 2 3 4

-1 -2 -3 -4 -> -1 -2 -3

0 -1 -2 -3  -> 0 -1 -2

1  0 -1 -2  -> 1 -1 -2


9 8 7 6 0 -6 -7 -8 -9 -> 9 8 7


3 2 1 0 -6 -7 -8 -9 -> 3 -8 -9















































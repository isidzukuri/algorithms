# https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

# @param {Integer[]} prices
# @return {Integer}
def max_profit(prices)
    profit = 0
    (1...prices.size).each do |idx|
        if prices[idx] > prices[idx - 1]
            profit += prices[idx] - prices[idx - 1]
        end
    end
    profit
end



###################################################### AI

# @param {Integer[]} prices
# @return {Integer}
def max_profit(prices)
  prices.each_cons(2).sum { |prev, curr| curr > prev ? curr - prev : 0 }
end
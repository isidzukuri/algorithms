# https://www.hackerrank.com/challenges/repeated-string/problem
require 'bigdecimal'

def repeatedString(s, n)
  pattern_repeats = BigDecimal(n)/s.length

  count = s.count(s) * pattern_repeats.floor

  if pattern_repeats > pattern_repeats.floor 
    to_index = (BigDecimal(s.length) * (pattern_repeats - pattern_repeats.floor)).ceil - 1

    tail_count = s[0..to_index].count(s)

    count += tail_count
  end

  count
end

repeatedString('a', 1000000000000)
# repeatedString('aba', 10)
# repeatedString('bab', 725261545450)

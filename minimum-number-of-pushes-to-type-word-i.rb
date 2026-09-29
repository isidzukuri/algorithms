# https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-i/


# @param {String} word
# @return {Integer}
BUTTONS_COUNT = 8
def minimum_pushes(word)
    pushes = 0
    len = word.length()
    multiplier = 1

    loop do
        remainder = len - BUTTONS_COUNT
        if remainder >= 0
            pushes += BUTTONS_COUNT * multiplier
            len = remainder
        else
            pushes += len * multiplier
        end

        break if remainder <= 0
        multiplier +=1
    end
    pushes
end




def minimum_pushes(word)
  n = word.length
  ans = 0
  
  (0...n).each do |i|
    ans += (i / 8) + 1
  end
  
  ans
end
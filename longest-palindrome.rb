# https://leetcode.com/problems/longest-palindrome/

# @param {String} s
# @return {Integer}
def longest_palindrome(s)
    hash = Hash.new(0)

    s.each_byte do |chr|
        hash[chr] +=1
    end

    count = 0
    longest_odd = 0

    hash.values.each do |val|
        if val.odd?
            if val > longest_odd
                count += longest_odd == 0 ? 0 : longest_odd-1
                longest_odd = val
            else
                count += val -1
            end
        else
            count += val
        end
    end

    count + longest_odd
end



# @param {String} s
# @return {Integer}
def longest_palindrome(s)
    hash = Hash.new(0)

    s.each_char do |chr|
        hash[chr] +=1
    end

    count = 0
    longest_odd = 0

    hash.values.each do |val|
        if val.odd?
            if val > longest_odd
                count += longest_odd == 0 ? 0 : longest_odd-1
                longest_odd = val
            else
                count += val -1
            end
        else
            count += val
        end
    end

    count + longest_odd
end
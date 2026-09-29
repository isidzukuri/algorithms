# https://leetcode.com/problems/valid-palindrome/

# @param {String} s
# @return {Boolean}
def is_palindrome(s)
    left = 0
    right = s.size - 1

    while left <= right do
        l_char = s[left]
        r_char = s[right]
        
        if !l_char.match?(/[[:alnum:]]/) 
            left +=1
            next
        end

        if !r_char.match?(/[[:alnum:]]/) 
            right -=1
            next
        end

        return false if l_char.downcase != r_char.downcase

        left +=1
        right -=1
    end

    true
end
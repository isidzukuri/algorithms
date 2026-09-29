# https://leetcode.com/problems/is-subsequence/

# @param {String} s
# @param {String} t
# @return {Boolean}
def is_subsequence(s, t)
    return true if s.size == 0
    r_bytes = s.bytes
    idx = 0

    t.bytes.each do |left|
        if r_bytes[idx] == left
            idx +=1
            return true if idx == r_bytes.size
        end
    end

    false
end
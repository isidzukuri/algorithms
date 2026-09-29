# https://leetcode.com/problems/ransom-note/

# @param {String} ransom_note
# @param {String} magazine
# @return {Boolean}
def can_construct(ransom_note, magazine)
    mag_stats = magazine.bytes.tally
    ransom_note.each_byte.each do |byte|
        return false if mag_stats[byte].nil? || mag_stats[byte] == 0

        mag_stats[byte] -= 1
    end
    true
end

############################## AI


def can_construct(ransom_note, magazine)
  return false if ransom_note.length > magazine.length

  mag_counts = magazine.bytes.tally
  ransom_note.bytes.tally.all? { |char, count| (mag_counts[char] || 0) >= count }
end

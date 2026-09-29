# https://leetcode.com/problems/design-add-and-search-words-data-structure

class WordDictionary
    attr_accessor :children, :completed

    DOT = 46
    
    def initialize()
       @children = {}
       @completed = false 
    end


=begin
    :type word: String
    :rtype: Void
=end
    def add_word(word)
        cur = self
        word.bytes do |byte|
            cur = cur.children[byte] ||= WordDictionary.new()
        end
        cur.completed = true
    end

    def search_bytes(bytes, start_idx)
        cur = self
        
        (start_idx...bytes.length).each do |i|
            byte = bytes[i]
            
            if byte == DOT
                next_idx = i + 1
                return cur.children.each_value.any? { |child| child.search_bytes(bytes, next_idx) }
            end

            cur = cur.children[byte]
            return false unless cur
        end
        cur.completed
    end


=begin
    :type word: String
    :rtype: Boolean
=end
    def search(word)
        search_bytes(word.bytes, 0)
    end
end

# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary.new()
# obj.add_word(word)
# param_2 = obj.search(word)

########################################################### V!

class WordDictionary
    attr_accessor :children, :completed

    DOT = 46
    
    def initialize()
       @children = {}
       @completed = false 
    end


=begin
    :type word: String
    :rtype: Void
=end
    def add_word(word)
        cur = self
        word.bytes do |byte|
            cur = cur.children[byte] ||= WordDictionary.new()
        end
        cur.completed = true
    end

    def search_bytes(bytes)
        cur = self
        bytes.each_with_index do |byte, index|
            if byte == DOT
                return cur.children.values.any? { |child| child.search_bytes(bytes[(index+1)..]) }
            else
                cur = cur.children[byte]
            end

            return false unless cur
        end
        cur.completed
    end


=begin
    :type word: String
    :rtype: Boolean
=end
    def search(word)
        search_bytes(word.bytes)
    end
end

# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary.new()
# obj.add_word(word)
# param_2 = obj.search(word)




############################################################## AI

class WordDictionary
  def initialize
    @words_by_len = Hash.new { |h, k| h[k] = [] }
  end

  def add_word(word)
    @words_by_len[word.length] << word
  end

  def search(word)
    candidates = @words_by_len[word.length]
    return false if candidates.empty?

    # If no dots, do a fast O(1) exact match lookup via Set or Hash
    unless word.include?('.')
      return candidates.include?(word)
    end

    # Convert wildcard pattern to standard Regex once
    regex = Regexp.new("^#{word}$")
    candidates.any? { |w| regex.match?(w) }
  end
end



######################################################## AI

class WordDictionary
  attr_accessor :children, :completed

  DOT = 46

  def initialize
    @children = Array.new(26) # Fixed array instead of Hash
    @completed = false
  end

  def add_word(word)
    cur = self
    word.each_byte do |byte|
      idx = byte - 97
      cur = (cur.children[idx] ||= WordDictionary.new)
    end
    cur.completed = true
  end

  def search(word)
    search_bytes(word.bytes, 0)
  end

  private

  def search_bytes(bytes, start_idx)
    cur = self

    (start_idx...bytes.length).each do |i|
      byte = bytes[i]

      if byte == DOT
        next_idx = i + 1
        # Iterate direct array references, filtering out nil entries
        return cur.children.compact.any? { |child| child.search_bytes(bytes, next_idx) }
      end

      cur = cur.children[byte - 97]
      return false unless cur
    end

    cur.completed
  end
end
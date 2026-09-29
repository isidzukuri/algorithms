# https://leetcode.com/problems/implement-magic-dictionary

class MagicDictionary
    
    MAX_DIFF = 1

    attr_accessor :children, :completed
    
    def initialize()
        @children = {}
        @completed = false
    end

    def [](key)
        children[key]
    end

    def []=(key, val)
        children[key] = val
    end


=begin
    :type dictionary: String[]
    :rtype: Void
=end
    def build_dict(dictionary)
        dictionary.each do |word|
            add_word(word.bytes)
        end
    end

    def add_word(bytes)
        cur = self

        bytes.each_with_index do |bt, idx|
            cur[bt] ||= MagicDictionary.new
            cur = cur[bt]
        end
        cur.completed = true
    end


=begin
    :type search_word: String
    :rtype: Boolean
=end
    def search(search_word)
        search_bytes(search_word.bytes, 0, 0)
    end

    def search_bytes(bytes, start, diff)
        children.any? do |bt, child|
            if bt == bytes[start]
                if start == (bytes.size-1) && child.completed && diff == MAX_DIFF
                    return true
                end
                child.search_bytes(bytes, (start + 1), diff)
            else
                if start == (bytes.size-1) && child.completed && diff < MAX_DIFF
                    return true
                end
                if diff == MAX_DIFF
                    false
                else
                    child.search_bytes(bytes, (start + 1), (diff+1))
                end
            end
        end
    end


end

# Your MagicDictionary object will be instantiated and called as such:
# obj = MagicDictionary.new()
# obj.build_dict(dictionary)
# param_2 = obj.search(search_word)

################################################################################### AI

class MagicDictionary
    def initialize()
        @children = {}
        @completed = false
    end

    def build_dict(dictionary)
        dictionary.each do |word|
            curr = self
            word.each_byte do |byte|
                curr.children[byte] ||= MagicDictionary.new
                curr = curr.children[byte]
            end
            curr.completed = true
        end
    end

    def search(search_word)
        search_from(search_word.bytes, 0, 0)
    end

    protected

    attr_accessor :children, :completed

    def search_from(bytes, index, diff)
        return false if diff > 1

        if index == bytes.length
            return completed && diff == 1
        end

        current_byte = bytes[index]

        children.any? do |byte, child_node|
            next_diff = (byte == current_byte) ? diff : diff + 1
            child_node.search_from(bytes, index + 1, next_diff)
        end
    end
end

################################################################################ AI

class MagicDictionary
    def initialize()
        @children = {}
        @completed = false
    end

    def build_dict(dictionary)
        dictionary.each do |word|
            curr = self
            word.each_char do |char|
                curr.children[char] ||= MagicDictionary.new
                curr = curr.children[char]
            end
            curr.completed = true
        end
    end

    def search(search_word)
        search_from(search_word, 0, 0)
    end

    protected

    attr_accessor :children, :completed

    def search_from(word, index, diff)
        # Prune branches that exceed the single-character difference limit
        return false if diff > 1

        # If we've reached the end of the search word, it's a valid match 
        # only if the node is completed and we made exactly 1 substitution.
        if index == word.length
            return completed && diff == 1
        end

        current_char = word[index]

        # Check all possible child paths
        children.any? do |char, child_node|
            next_diff = (char == current_char) ? diff : diff + 1
            child_node.search_from(word, index + 1, next_diff)
        end
    end
end

################################################################################### AI



require 'set'

class MagicDictionary
    def initialize()
        # Maps wildcard variants to a set of words that produce them
        @variants = Hash.new { |h, k| h[k] = Set.new }
    end

    def build_dict(dictionary)
        dictionary.each do |word|
            word.length.times do |i|
                variant = word.dup
                variant[i] = '*'
                @variants[variant].add(word)
            end
        end
    end

    def search(search_word)
        search_word.length.times do |i|
            variant = search_word.dup
            variant[i] = '*'
            
            # If this variant exists in our dictionary
            if @variants.key?(variant)
                matching_words = @variants[variant]
                
                # A match is valid if there is at least one word in the bucket 
                # that is different from our search_word.
                if matching_words.any? { |word| word != search_word }
                    return true
                end
            end
        end
        
        false
    end
end
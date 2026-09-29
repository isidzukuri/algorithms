# https://leetcode.com/problems/design-an-ordered-stream

class OrderedStream

=begin
    :type n: Integer
=end
    def initialize(n)
        @store = Array.new(n)
        @longest_prefix = 0
        @start = 0
    end


=begin
    :type id_key: Integer
    :type value: String
    :rtype: String[]
=end
    def insert(id_key, value)
        @store[id_key-1] = value
        while @store[@longest_prefix]
            @longest_prefix += 1
        end
        return [] if @longest_prefix == 0
        
        result = @store[@start..@longest_prefix-1]
        @start = @longest_prefix
        result
    end


end

# Your OrderedStream object will be instantiated and called as such:
# obj = OrderedStream.new(n)
# param_1 = obj.insert(id_key, value)


##################################################################### AI

class OrderedStream
    def initialize(n)
        @stream = Array.new(n)
        @ptr = 0
    end

    def insert(id_key, value)
        @stream[id_key - 1] = value
        
        result = []
        # Collect values sequentially as long as they exist starting from @ptr
        while @ptr < @stream.length && @stream[@ptr]
            result << @stream[@ptr]
            @ptr += 1
        end
        
        result
    end
end
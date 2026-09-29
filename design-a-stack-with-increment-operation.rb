# https://leetcode.com/problems/design-a-stack-with-increment-operation

class CustomStack

=begin
    :type max_size: Integer
=end
    def initialize(max_size)
        @store = []
        @max_size = max_size
    end


=begin
    :type x: Integer
    :rtype: Void
=end
    def push(x)
      return if @store.size == @max_size

      @store << x
    end


=begin
    :rtype: Integer
=end
    def pop()
        @store.pop || -1
    end


=begin
    :type k: Integer
    :type val: Integer
    :rtype: Void
=end
    def increment(k, val)
        @store.each_with_index do |item, idx|
            break if idx == k

            @store[idx] = item + val
        end
    end


end

# Your CustomStack object will be instantiated and called as such:
# obj = CustomStack.new(max_size)
# obj.push(x)
# param_2 = obj.pop()
# obj.increment(k, val)



################################################################### AI


class CustomStack
=begin
    :type max_size: Integer
=end
    def initialize(max_size)
        @store = []
        @inc = [] # Lazy increment array
        @max_size = max_size
    end

=begin
    :type x: Integer
    :rtype: Void
=end
    def push(x)
        return if @store.size == @max_size

        @store << x
        @inc << 0 # Initialize increment slot for the new element
    end

=begin
    :rtype: Integer
=end
    def pop()
        return -1 if @store.empty?

        idx = @store.size - 1
        
        # Propagate lazy increment to the element below it
        if idx > 0
            @inc[idx - 1] += @inc[idx]
        end

        # Pop the value and add any pending lazy increment
        val = @store.pop + @inc.pop
        val
    end

=begin
    :type k: Integer
    :type val: Integer
    :rtype: Void
=end
    def increment(k, val)
        return if @store.empty?

        # Find the actual index to apply the increment (min of k-1 and last index)
        idx = [k - 1, @store.size - 1].min
        @inc[idx] += val
    end
end
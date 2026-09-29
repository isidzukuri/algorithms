# https://leetcode.com/problems/min-stack

class MinStack
    Node = Struct.new(:val, :min, :prev)

    def initialize()
        @head = Node.new
        @tail = Node.new
        @tail.prev = @head
    end


=begin
    :type value: Integer
    :rtype: Void
=end
    def push(value)
        node = Node.new(value)
        node.prev = @tail.prev
        @tail.prev = node
        if @tail.min
            if @tail.min.val >= node.val
                node.min = @tail.min
                @tail.min = node
            end
        else
            @tail.min = node
            node.min = @tail
        end
    end


=begin
    :rtype: Void
=end
    def pop()
        return if @tail.prev == @head

        remove_prev(@tail)
    end

    def remove_prev(node)
        removable = node.prev
        if removable == node.min
            if removable.min == node
                node.min = nil
            else
                node.min = removable.min
            end
        end

        node.prev = removable.prev
    end


=begin
    :rtype: Integer
=end
    def top()
        return if @tail.prev == @head
        @tail.prev.val
    end


=begin
    :rtype: Integer
=end
    def get_min()
        return if @tail.prev == @head
        return unless @tail.min
        @tail.min.val
    end


end

# Your MinStack object will be instantiated and called as such:
# obj = MinStack.new()
# obj.push(value)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.get_min()


###################################################################### AI


class MinStack
    def initialize()
        @stack = []
        @min_stack = []
    end

=begin
    :type value: Integer
    :rtype: Void
=end
    def push(value)
        @stack.push(value)
        if @min_stack.empty? || value <= @min_stack.last
            @min_stack.push(value)
        end
    end

=begin
    :rtype: Void
=end
    def pop()
        val = @stack.pop
        @min_stack.pop if val == @min_stack.last
    end

=begin
    :rtype: Integer
=end
    def top()
        @stack.last
    end

=begin
    :rtype: Integer
=end
    def get_min()
        @min_stack.last
    end
end
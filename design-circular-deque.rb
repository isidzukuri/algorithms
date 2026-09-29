# https://leetcode.com/problems/design-circular-deque

Node = Struct.new(:val, :prev, :next)

class MyCircularDeque

=begin
    :type k: Integer
=end
    def initialize(k)
        @max_size = k
        @size = 0
        @start = Node.new
        @end = Node.new
        @start.next = @end
        @end.prev = @start
    end


=begin
    :type value: Integer
    :rtype: Boolean
=end
    def insert_front(value)
        return false if is_full

        @size += 1
        node = Node.new(value, @start, @start.next)
        @start.next.prev = node
        @start.next = node
        true
    end


=begin
    :type value: Integer
    :rtype: Boolean
=end
    def insert_last(value)
        return false if is_full

        @size += 1
        node = Node.new(value, @end.prev, @end)
        @end.prev.next = node
        @end.prev = node
        true
    end


=begin
    :rtype: Boolean
=end
    def delete_front()
        return false if is_empty

        @size -= 1
        node = @start.next
        node.next.prev = @start
        @start.next = node.next
        true
    end


=begin
    :rtype: Boolean
=end
    def delete_last()
        return false if is_empty

        @size -= 1
        node = @end.prev
        node.prev.next = @end
        @end.prev = node.prev
        true
    end


=begin
    :rtype: Integer
=end
    def get_front()
        return -1 if is_empty
        
        @start.next.val
    end


=begin
    :rtype: Integer
=end
    def get_rear()
        return -1 if is_empty
        
        @end.prev.val
    end


=begin
    :rtype: Boolean
=end
    def is_empty()
        @size == 0
    end


=begin
    :rtype: Boolean
=end
    def is_full()
        @size == @max_size
    end


end

# Your MyCircularDeque object will be instantiated and called as such:
# obj = MyCircularDeque.new(k)
# param_1 = obj.insert_front(value)
# param_2 = obj.insert_last(value)
# param_3 = obj.delete_front()
# param_4 = obj.delete_last()
# param_5 = obj.get_front()
# param_6 = obj.get_rear()
# param_7 = obj.is_empty()
# param_8 = obj.is_full()


############################################################################## AI

class MyCircularDeque
=begin
    :type k: Integer
=end
    def initialize(k)
        @k = k
        @queue = Array.new(k)
        @head = 0
        @tail = 0
        @size = 0
    end

=begin
    :type value: Integer
    :rtype: Boolean
=end
    def insert_front(value)
        return false if is_full()
        @head = (@head - 1) % @k
        @queue[@head] = value
        @size += 1
        true
    end

=begin
    :type value: Integer
    :rtype: Boolean
=end
    def insert_last(value)
        return false if is_full()
        @queue[@tail] = value
        @tail = (@tail + 1) % @k
        @size += 1
        true
    end

=begin
    :rtype: Boolean
=end
    def delete_front()
        return false if is_empty()
        @head = (@head + 1) % @k
        @size -= 1
        true
    end

=begin
    :rtype: Boolean
=end
    def delete_last()
        return false if is_empty()
        @tail = (@tail - 1) % @k
        @size -= 1
        true
    end

=begin
    :rtype: Integer
=end
    def get_front()
        return -1 if is_empty()
        @queue[@head]
    end

=begin
    :rtype: Integer
=end
    def get_rear()
        return -1 if is_empty()
        @queue[(@tail - 1) % @k]
    end

=begin
    :rtype: Boolean
=end
    def is_empty()
        @size == 0
    end

=begin
    :rtype: Boolean
=end
    def is_full()
        @size == @k
    end
end
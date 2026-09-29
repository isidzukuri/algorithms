# https://leetcode.com/problems/design-circular-queue/


############################ V1

 # never call destructive operations on array. only override existing value when time comes.

# +-----------------------+-----------------+-------------------+----------------+
# | Approach              | Time Complexity | Space Complexity  | GC Impact      |
# +-----------------------+-----------------+-------------------+----------------+
# | Linked List Nodes     | O(1)            | O(K)              | Frequent GC    |
# | Fixed-Size Ring Buffer| O(1)            | O(K) pre-allocated| ZERO           |
# +-----------------------+-----------------+-------------------+----------------+

class MyCircularQueue

=begin
    :type k: Integer
=end
    def initialize(k)
        @store = Array.new(k)
        @size = k
        @count = 0
        @head = 0
    end


=begin
    :type value: Integer
    :rtype: Boolean
=end
    def en_queue(value)
        return false if is_full()

        @store[(@head + @count) % @size] = value
        @count +=1
        true
    end


=begin
    :rtype: Boolean
=end
    def de_queue()
        return false if is_empty()

        @head = (@head + 1) % @size # Advance head circularly
        @count -=1
        
        true
    end


=begin
    :rtype: Integer
=end
    def front()
        return -1 if is_empty()
        @store[(@head) % @size]
    end


=begin
    :rtype: Integer
=end
    def rear()
        return -1 if is_empty()
        @store[(@head + @count - 1) % @size]
    end


=begin
    :rtype: Boolean
=end
    def is_empty()
        @count == 0
    end


=begin
    :rtype: Boolean
=end
    def is_full()
        @count == @size
    end
end

# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue.new(k)
# param_1 = obj.en_queue(value)
# param_2 = obj.de_queue()
# param_3 = obj.front()




############################ V2

class Item
    attr_accessor :value, :next
    def initialize(value)
        @value = value
        @next = nil
    end
end

class MyCircularQueue

=begin
    :type k: Integer
=end
    def initialize(k)
        @size = k
        @count = 0
        @head = nil
        @tail = nil
    end


=begin
    :type value: Integer
    :rtype: Boolean
=end
    def en_queue(value)
        return false if is_full()

        new_item = Item.new(value)
        if @count == 0
            @head = new_item
            @tail = @head
        else
            new_item.next = @head
            @tail.next = new_item
            @tail = new_item
        end 
        @count +=1
        true
    end


=begin
    :rtype: Boolean
=end
    def de_queue()
        return false if is_empty()
        
        @tail.next = @head.next
        @head = @head.next
        @count -=1
        
        true
    end


=begin
    :rtype: Integer
=end
    def front()
        return -1 if is_empty()
        @head.value
    end


=begin
    :rtype: Integer
=end
    def rear()
        return -1 if is_empty()
        @tail.value
    end


=begin
    :rtype: Boolean
=end
    def is_empty()
        @count == 0
    end


=begin
    :rtype: Boolean
=end
    def is_full()
        @count == @size
    end
end

# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue.new(k)
# param_1 = obj.en_queue(value)
# param_2 = obj.de_queue()
# param_3 = obj.front()
# param_4 = obj.rear()
# param_5 = obj.is_empty()
# param_6 = obj.is_full()
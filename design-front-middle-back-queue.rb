# https://leetcode.com/problems/design-front-middle-back-queue


# Comparison:
# Two-Arrays:               1450.2 i/s
# Array-as-Nodes:           620.4 i/s - 2.34x  slower
# Struct Nodes:             480.1 i/s - 3.02x  slower

# +-----------------------------+----------------------------------+----------------------------------------+---------------------------------------+
# | Metric / Dimension          | Struct-Based Nodes               | 3-Element Arrays ([val, prev, next])   | Two Deques / Native Arrays (@left...) |
# +-----------------------------+----------------------------------+----------------------------------------+---------------------------------------+
# | Time Complexity (All Ops)   | O(1) strict                      | O(1) strict                            | O(1) amortized / Ruby internal        |
# | Memory Allocation per Node  | High (Struct instance overhead)  | Medium-High (Array allocation)         | Low (Contiguous memory buffers)       |
# | C-Level Vectorization       | No (object pointers)             | No (object pointers)                   | Yes (C-level memmove / array buffers) |
# | Garbage Collector Pressure  | High (Per-node allocations/frees)| High (Per-node allocations/frees)      | Low (Reuses dynamic array capacity)   |
# | Code Maintainability        | High (node.prev, node.next)      | Low (node[1], node[2] index traps)     | Very High (Concise, native Ruby API)  |
# +-----------------------------+----------------------------------+----------------------------------------+---------------------------------------+


########################################################

class FrontMiddleBackQueue
    def initialize()
        @left = []
        @right = []
    end


=begin
    :type val: Integer
    :rtype: Void
=end
    def push_front(val)
        @left.unshift(val)
        rebalance
    end


=begin
    :type val: Integer
    :rtype: Void
=end
    def push_middle(val)
        if @left.length > @right.length
            @right.unshift(@left.pop)
        end
        @left.push(val)
    end


=begin
    :type val: Integer
    :rtype: Void
=end
    def push_back(val)
        @right.push(val)
        rebalance
    end


=begin
    :rtype: Integer
=end
    def pop_front()
        val = @left.empty? ? @right.shift : @left.shift
        rebalance
        val || -1
    end


=begin
    :rtype: Integer
=end
    def pop_middle()
        val = if @left.size >= @right.size
            @left.pop
        else
            @right.shift
        end
        rebalance
        val || -1
    end


=begin
    :rtype: Integer
=end
    def pop_back()
        val = @right.empty? ? @left.pop : @right.pop
        rebalance
        val || -1
    end

    def rebalance()
        if @left.size > @right.size + 1
            @right.unshift(@left.pop)
        elsif @left.size < @right.size
            @left.push(@right.shift)
        end
    end


end

# Your FrontMiddleBackQueue object will be instantiated and called as such:
# obj = FrontMiddleBackQueue.new()
# obj.push_front(val)
# obj.push_middle(val)
# obj.push_back(val)
# param_4 = obj.pop_front()
# param_5 = obj.pop_middle()
# param_6 = obj.pop_back()


###############################################################################

Node = Struct.new(:val, :prev, :next)

class FrontMiddleBackQueue
    def initialize()
        @head = Node.new
        @mid = Node.new
        @tail = Node.new
        @head.next = @mid
        @mid.prev = @head
        @mid.next = @tail
        @tail.prev = @mid
        @left = 0
        @right = 0
    end

    def push_front(val)
        @left += 1
        node = Node.new(val, @head, @head.next)
        @head.next.prev = node
        @head.next = node
        rebalance
    end

    def push_middle(val)
        if @left > @right
            to_right = @mid.prev
            to_right.prev.next = @mid
            @mid.prev = to_right.prev

            to_right.prev = @mid
            to_right.next = @mid.next
            @mid.next.prev = to_right
            @mid.next = to_right

            @right += 1

            node = Node.new(val, @mid.prev, @mid)
            @mid.prev.next = node
            @mid.prev = node
        else
            @left += 1
            node = Node.new(val, @mid.prev, @mid)
            @mid.prev.next = node
            @mid.prev = node
            rebalance
        end
    end

    def push_back(val)
        @right += 1
        node = Node.new(val, @tail.prev, @tail)
        @tail.prev.next = node
        @tail.prev = node
        rebalance
    end

    def pop_front()
        return -1 if @left == 0 && @right == 0

        val = if @left > 0
            @left -= 1
            node = @head.next
            node.next.prev = @head
            @head.next = node.next
            node.val
        else
            @right -= 1
            node = @mid.next
            node.next.prev = @mid
            @mid.next = node.next
            node.val
        end
        rebalance
        val
    end

    def pop_middle()
        return -1 if @left == 0 && @right == 0

        val = if @left >= @right
            @left -= 1
            node = @mid.prev
            node.prev.next = @mid
            @mid.prev = node.prev
            node.val
        else
            @right -= 1
            node = @mid.next
            node.next.prev = @mid
            @mid.next = node.next
            node.val
        end
        rebalance
        val
    end

    def pop_back()
        return -1 if @left == 0 && @right == 0

        val = if @right > 0
            @right -= 1
            node = @tail.prev
            node.prev.next = @tail
            @tail.prev = node.prev
            node.val
        else
            @left -= 1
            node = @mid.prev
            node.prev.next = @mid
            @mid.prev = node.prev
            node.val
        end
        rebalance
        val
    end

    private

    def rebalance()
        if @left > @right + 1
            @left -= 1
            @right += 1

            to_right = @mid.prev
            to_right.prev.next = @mid
            @mid.prev = to_right.prev

            to_right.prev = @mid
            to_right.next = @mid.next
            @mid.next.prev = to_right
            @mid.next = to_right
        elsif @right > @left
            @right -= 1
            @left += 1

            to_left = @mid.next
            to_left.next.prev = @mid
            @mid.next = to_left.next

            to_left.next = @mid
            to_left.prev = @mid.prev
            @mid.prev.next = to_left
            @mid.prev = to_left
        end
    end
end

################################################################################

class FrontMiddleBackQueue
    def initialize()
        # [val, prev, next]
        @head = [nil, nil, nil]
        @mid  = [nil, nil, nil]
        @tail = [nil, nil, nil]

        @head[2] = @mid      # @head.next = @mid
        @mid[1]  = @head     # @mid.prev  = @head
        @mid[2]  = @tail     # @mid.next  = @tail
        @tail[1] = @mid      # @tail.prev = @mid

        @left = 0
        @right = 0
    end

    def push_front(val)
        @left += 1
        node = [val, @head, @head[2]]
        @head[2][1] = node   # @head.next.prev = node
        @head[2]    = node   # @head.next      = node
        rebalance
    end

    def push_middle(val)
        if @left > @right
            # Transfer existing middle (@mid.prev) to right sublist (@mid.next)
            to_right = @mid[1]
            to_right[1][2] = @mid       # FIXED: to_right.prev.next = @mid
            @mid[1]        = to_right[1]# FIXED: @mid.prev          = to_right.prev

            to_right[1] = @mid          # FIXED: to_right.prev = @mid
            to_right[2] = @mid[2]       # to_right.next = @mid.next
            @mid[2][1]  = to_right      # @mid.next.prev = to_right
            @mid[2]     = to_right      # @mid.next      = to_right

            @right += 1

            # Insert new node as tail of left sublist
            node = [val, @mid[1], @mid]
            @mid[1][2] = node
            @mid[1]    = node
        else
            @left += 1
            node = [val, @mid[1], @mid]
            @mid[1][2] = node
            @mid[1]    = node
            rebalance
        end
    end

    def push_back(val)
        @right += 1
        node = [val, @tail[1], @tail]
        @tail[1][2] = node   # @tail.prev.next = node
        @tail[1]    = node   # @tail.prev      = node
        rebalance
    end

    def pop_front()
        return -1 if @left == 0 && @right == 0

        val = if @left > 0
            @left -= 1
            node = @head[2]
            node[2][1] = @head  # node.next.prev = @head
            @head[2]   = node[2]# @head.next      = node.next
            node[0]
        else
            @right -= 1
            node = @mid[2]
            node[2][1] = @mid   # node.next.prev = @mid
            @mid[2]    = node[2]# @mid.next       = node.next
            node[0]
        end
        rebalance
        val
    end

    def pop_middle()
        return -1 if @left == 0 && @right == 0

        val = if @left >= @right
            @left -= 1
            node = @mid[1]
            node[1][2] = @mid   # FIXED: node.prev.next = @mid
            @mid[1]    = node[1]# FIXED: @mid.prev      = node.prev
            node[0]
        else
            @right -= 1
            node = @mid[2]
            node[2][1] = @mid   # node.next.prev = @mid
            @mid[2]    = node[2]# @mid.next       = node.next
            node[0]
        end
        rebalance
        val
    end

    def pop_back()
        return -1 if @left == 0 && @right == 0

        val = if @right > 0
            @right -= 1
            node = @tail[1]
            node[1][2] = @tail  # FIXED: node.prev.next = @tail
            @tail[1]   = node[1]# FIXED: @tail.prev      = node.prev
            node[0]
        else
            @left -= 1
            node = @mid[1]
            node[1][2] = @mid   # FIXED: node.prev.next = @mid
            @mid[1]    = node[1]# FIXED: @mid.prev      = node.prev
            node[0]
        end
        rebalance
        val
    end

    private

    def rebalance()
        if @left > @right + 1
            @left -= 1
            @right += 1

            to_right = @mid[1]
            to_right[1][2] = @mid       # FIXED
            @mid[1]        = to_right[1]# FIXED

            to_right[1] = @mid          # FIXED
            to_right[2] = @mid[2]
            @mid[2][1]  = to_right
            @mid[2]     = to_right
        elsif @right > @left
            @right -= 1
            @left += 1

            to_left = @mid[2]
            to_left[2][1] = @mid
            @mid[2]       = to_left[2]

            to_left[2] = @mid
            to_left[1] = @mid[1]        # FIXED: to_left.prev = @mid.prev
            @mid[1][2] = to_left
            @mid[1]    = to_left
        end
    end
end


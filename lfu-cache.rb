# https://leetcode.com/problems/lfu-cache/

class LFUCache
    Node = Struct.new(:key, :val, :freq, :prev, :next)
    Lru = Struct.new(:head, :tail)

=begin
    :type capacity: Integer
=end
    def initialize(capacity)
        @capacity = capacity
        @map = {}
        @min_freq = 1
        @lists = { 1 => new_lru }
    end


=begin
    :type key: Integer
    :rtype: Integer
=end
    def get(key)
        if node = @map[key]
            increment(node)
            return node.val
        end
        -1
    end


=begin
    :type key: Integer
    :type value: Integer
    :rtype: Void
=end
    def put(key, value)
        node = @map[key]
        if node
            increment(node)
        else
            if @map.size == @capacity
                delete
            end
            @min_freq = 1
            node = Node.new
            node.key = key
            node.freq = 1
            list_add(node)
            @map[key] = node
        end
        node.val = value
    end

    private

    def delete
        lru = nil
        loop do
            if lru = @lists[@min_freq]
                if lru.head.next == lru.tail
                    @min_freq +=1
                else
                    break
                end
            end
        end

        node = lru.tail.prev
        lru.tail.prev.prev.next = lru.tail
        lru.tail.prev = lru.tail.prev.prev

        @map.delete(node.key)
    end

    def new_lru
        head = Node.new
        tail = Node.new
        tail.prev = head
        head.next = tail
        Lru.new(head, tail)
    end

    def increment(node)
        node.freq +=1

        node.prev.next = node.next
        node.next.prev = node.prev
        list_add(node)
    end

    def list_add(node)
        @lists[node.freq] ||= new_lru

        node.prev = @lists[node.freq].head
        node.next = @lists[node.freq].head.next
        @lists[node.freq].head.next.prev = node
        @lists[node.freq].head.next = node
    end
end

# Your LFUCache object will be instantiated and called as such:
# obj = LFUCache.new(capacity)
# param_1 = obj.get(key)
# obj.put(key, value)

########################################################################### AI


class LFUCache
    Node = Struct.new(:key, :val, :freq, :prev, :next)

    def initialize(capacity)
        @capacity = capacity
        @min_freq = 0
        @key_node = {}
        @freq_lists = Hash.new { |h, k| h[k] = create_doubly_linked_list }
    end

    def get(key)
        return -1 unless @key_node.key?(key)

        node = @key_node[key]
        increment_freq(node)
        node.val
    end

    def put(key, value)
        return if @capacity <= 0

        if @key_node.key?(key)
            node = @key_node[key]
            node.val = value
            increment_freq(node)
        else
            if @key_node.size >= @capacity
                # Evict the least recently used item from the minimum frequency list
                evict_list = @freq_lists[@min_freq]
                node_to_evict = evict_list[:tail].prev
                remove_node(node_to_evict)
                @key_node.delete(node_to_evict.key)
            end

            # Create new node with frequency 1
            node = Node.new(key, value, 1, nil, nil)
            @key_node[key] = node
            @min_freq = 1
            add_to_head(node, @freq_lists[1])
        end
    end

    private

    def create_doubly_linked_list
        head = Node.new(nil, nil, 0, nil, nil)
        tail = Node.new(nil, nil, 0, nil, nil)
        head.next = tail
        tail.prev = head
        { head: head, tail: tail }
    end

    def remove_node(node)
        node.prev.next = node.next
        node.next.prev = node.prev
    end

    def add_to_head(node, list)
        head = list[:head]
        node.prev = head
        node.next = head.next
        head.next.prev = node
        head.next = node
    end

    def increment_freq(node)
        current_freq = node.freq
        current_list = @freq_lists[current_freq]

        remove_node(node)

        # If the minimum frequency list is now empty, increment @min_freq
        if current_freq == @min_freq && current_list[:head].next == current_list[:tail]
            @min_freq += 1
        end

        node.freq += 1
        add_to_head(node, @freq_lists[node.freq])
    end
end
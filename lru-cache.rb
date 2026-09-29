# https://leetcode.com/problems/lru-cache

class LRUCache

    Node = Struct.new(:key, :val, :prev, :next)

=begin
    :type capacity: Integer
=end
    def initialize(capacity)
        @capacity = capacity
        @cache = {}
        @head = Node.new
        @tail = Node.new
        @head.next = @tail
        @tail.prev = @head
    end


=begin
    :type key: Integer
    :rtype: Integer
=end
    def get(key)
        return -1 unless node = @cache[key]
        
        touch(node)

        node[:val]
    end


=begin
    :type key: Integer
    :type value: Integer
    :rtype: Void
=end
    def put(key, value)
        node = @cache[key]
        if node.nil? && @capacity == @cache.size
            last = @tail.prev
            @cache.delete(last.key)
            new_last = last.prev
            new_last.next = @tail
            @tail.prev = new_last
        end

        node ||= Node.new(key)
        node.val = value
        
        touch(node)

        @cache[key] = node
    end

    def touch(node)
        node.prev.next = node.next if node.next
        node.next.prev = node.prev if node.prev
        node.prev = @head
        node.next = @head.next
        node.next.prev = node 
        @head.next = node
    end


end

# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache.new(capacity)
# param_1 = obj.get(key)
# obj.put(key, value)



################################################################# AI


class LRUCache
    Node = Struct.new(:key, :val, :prev, :next)

=begin
    :type capacity: Integer
=end
    def initialize(capacity)
        @capacity = capacity
        @cache = {}
        @head = Node.new
        @tail = Node.new
        @head.next = @tail
        @tail.prev = @head
    end

=begin
    :type key: Integer
    :rtype: Integer
=end
    def get(key)
        node = @cache[key]
        return -1 unless node

        promote(node)
        node.val
    end

=begin
    :type key: Integer
    :type value: Integer
    :rtype: Void
=end
    def put(key, value)
        if node = @cache[key]
            node.val = value
            promote(node)
        else
            evict if @cache.size >= @capacity

            node = Node.new(key, value)
            @cache[key] = node
            add_to_head(node)
        end
    end

    private

    def remove(node)
        node.prev.next = node.next
        node.next.prev = node.prev
    end

    def add_to_head(node)
        node.prev = @head
        node.next = @head.next
        @head.next.prev = node
        @head.next = node
    end

    def promote(node)
        remove(node)
        add_to_head(node)
    end

    def evict
        lru = @tail.prev
        remove(lru)
        @cache.delete(lru.key)
    end
end
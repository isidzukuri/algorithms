# https://leetcode.com/problems/number-of-recent-calls/

####### V1 #####################

class RecentCounter
    def initialize()
        @value = 0
        @store = []
    end


=begin
    :type t: Integer
    :rtype: Integer
=end
    def ping(t)
        @store.push(t)

        index = @store.bsearch_index { |item| item >= (t - 3000) } || 0

        @store.size - index
    end
end




####### V2 #####################

class RecentCounter
    PERIOD = 3000.freeze

    def initialize()
        @head = 0
        @store = Array.new(40000)
    end

=begin
    :type t: Integer
    :rtype: Integer
=end
    def ping(t)
        @store[@head] = t

        start_time = t - PERIOD
        idx = @head.clone

        while idx >= 0 && @store[idx] >= start_time do
            idx -=1
        end
        entries = @head - idx

        @head +=1

        entries
    end
end

# Your RecentCounter object will be instantiated and called as such:
# obj = RecentCounter.new()
# param_1 = obj.ping(t)



####### V3 #####################

class RecentCounter
    PERIOD = 3000.freeze

    def initialize()
        @head = 0
        @tail = 0
        @store = Array.new(40000)
    end

=begin
    :type t: Integer
    :rtype: Integer
=end
    def ping(t)
        return unless t

        @store[@head] = t

        start_time = t > PERIOD ? t - PERIOD : 0

        loop do
            if @store[@tail] < start_time
                @tail +=1
            else
                break
            end
        end

        @head +=1
        @head - @tail
    end
end

# Your RecentCounter object will be instantiated and called as such:
# obj = RecentCounter.new()
# param_1 = obj.ping(t)




####### V4 #####################

class Item
    attr_accessor :value, :next
    def initialize(value = 0, _next = nil)
        @value = value
        @next = _next
    end
end

class Que
    def initialize()
        @head = nil
        @tail = nil
        @count = 0
    end

    def push(value)
        new_item = Item.new(value)
        if @head
            @tail.next = new_item
            @tail = new_item
        else
            @head = new_item
            @tail = @head
        end
        @count +=1
    end

    def pop()
        return if @count == 0

        result = if @count == 1
            result = @head
            @head = nil
            @tail = nil
            result
        else
            result = @head
            @head = @head.next
            result
        end
        @count -=1
        result.value
    end

    def len
        @count
    end

    def front
        @head&.value
    end
end

class RecentCounter
    PERIOD = 3000.freeze

    def initialize()
        @que = Que.new()
    end

=begin
    :type t: Integer
    :rtype: Integer
=end
    def ping(t)
        return unless t
        @que.push(t)

        while (@que.front + PERIOD) < t
            @que.pop
        end
        
        @que.len
    end
end







####### NOT MY SOLUTION #####################


class RecentCounter
  def initialize()
    @calls = []
    @i = 0 
  end

=begin
    :type t: Integer
    :rtype: Integer
=end
    def ping(t)
          @calls << t
          while @calls[@i] < t - 3000
            @i += 1
          end
        return @calls.size - @i
    end
end

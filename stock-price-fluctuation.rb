# https://leetcode.com/problems/stock-price-fluctuation

class StockPrice
    def initialize()
       @ts = 0
       @prices = {}
       @min_max = Hash.new(0) 
    end


=begin
    :type timestamp: Integer
    :type price: Integer
    :rtype: Void
=end
    def update(timestamp, price)
        if @prices[timestamp]
            old_price = @prices[timestamp]
            @min_max[old_price] -= 1
            @min_max.delete(old_price) if @min_max[old_price] == 0
        end
        
        @min_max[price] += 1
        @prices[timestamp] = price
        @ts = timestamp if @ts < timestamp
    end


=begin
    :rtype: Integer
=end
    def current()
        @prices[@ts]
    end


=begin
    :rtype: Integer
=end
    def maximum()
        @min_max.keys.max
    end


=begin
    :rtype: Integer
=end
    def minimum()
        @min_max.keys.min
    end


end

# Your StockPrice object will be instantiated and called as such:
# obj = StockPrice.new()
# obj.update(timestamp, price)
# param_2 = obj.current()
# param_3 = obj.maximum()
# param_4 = obj.minimum()


################################################################### AI


class StockPrice
    def initialize()
        @timestamp_to_price = {}
        @max_timestamp = 0
        @min_heap = MinHeap.new
        @max_heap = MinHeap.new # Stores [-price, timestamp] for max-heap behavior
    end

    def update(timestamp, price)
        @timestamp_to_price[timestamp] = price
        @min_heap.push([price, timestamp])
        @max_heap.push([-price, timestamp])
        @max_timestamp = [@max_timestamp, timestamp].max
    end

    def current()
        @timestamp_to_price[@max_timestamp]
    end

    def maximum()
        while true
            neg_price, timestamp = @max_heap.peek
            price = -neg_price
            return price if @timestamp_to_price[timestamp] == price
            @max_heap.pop
        end
    end

    def minimum()
        while true
            price, timestamp = @min_heap.peek
            return price if @timestamp_to_price[timestamp] == price
            @min_heap.pop
        end
    end
end

class MinHeap
  def initialize
    @heap = []
  end

  def push(element)
    @heap << element
    bubble_up(@heap.size - 1)
  end

  def pop
    return nil if @heap.empty?
    return @heap.pop if @heap.size == 1

    root = @heap[0]
    @heap[0] = @heap.pop
    bubble_down(0)
    root
  end

  def peek
    @heap.first
  end

  def size
    @heap.size
  end

  def empty?
    @heap.empty?
  end

  private

  # Explicit comparison helper to prevent type-mismatch errors
  def less?(a, b)
    if a[0] == b[0]
      a[1] < b[1] # Tie-breaker by timestamp
    else
      a[0] < b[0] # Compare by price
    end
  end

  def bubble_up(index)
    parent_index = (index - 1) / 2
    if index > 0 && less?(@heap[index], @heap[parent_index])
      swap(index, parent_index)
      bubble_up(parent_index)
    end
  end

  def bubble_down(index)
    left_child = 2 * index + 1
    right_child = 2 * index + 2
    smallest = index

    if left_child < @heap.size && less?(@heap[left_child], @heap[smallest])
      smallest = left_child
    end

    if right_child < @heap.size && less?(@heap[right_child], @heap[smallest])
      smallest = right_child
    end

    if smallest != index
      swap(index, smallest)
      bubble_down(smallest)
    end
  end

  def swap(i, j)
    @heap[i], @heap[j] = @heap[j], @heap[i]
  end
end
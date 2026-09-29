# https://leetcode.com/problems/my-calendar-i


class MyCalendar
    def initialize()
        @store = []
    end


=begin
    :type start_time: Integer
    :type end_time: Integer
    :rtype: Boolean
=end
    def book(start_time, end_time)
        left = 0
        right = @store.size - 1
        
        while left <= right
            mid = left + (right - left) / 2
            cur_start, cur_end = @store[mid]

            if [start_time, cur_start].max < [end_time, cur_end].min
                return false
            elsif start_time >= cur_end
                left = mid + 1
            else
                right = mid - 1
            end
        end

        @store.insert(left, [start_time, end_time])

        true 
    end


end



########################################################## AI


class TreeNode
  attr_accessor :start, :end, :left, :right

  def initialize(start_time, end_time)
    @start = start_time
    @end = end_time
    @left = nil
    @right = nil
  end
end

class MyCalendar
  def initialize
    @root = nil
  end

  def book(start_time, end_time)
    if @root.nil?
      @root = TreeNode.new(start_time, end_time)
      return true
    end

    insert_node(@root, start_time, end_time)
  end

  private

  def insert_node(node, start_time, end_time)
    # 1. Overlap Condition: max(start1, start2) < min(end1, end2)
    if [start_time, node.start].max < [end_time, node.end].min
      return false
    end

    # 2. New interval ends before current node starts -> Go Left
    if end_time <= node.start
      if node.left.nil?
        node.left = TreeNode.new(start_time, end_time)
        return true
      else
        return insert_node(node.left, start_time, end_time)
      end
    # 3. New interval starts on or after current node ends -> Go Right
    else
      if node.right.nil?
        node.right = TreeNode.new(start_time, end_time)
        return true
      else
        return insert_node(node.right, start_time, end_time)
      end
    end
  end
end



########################################################## AI


class SegmentNode
  attr_accessor :left, :right, :booked

  def initialize
    @left = nil
    @right = nil
    @booked = false
  end
end

class MyCalendar
  MAX_RANGE = 1_000_000_000

  def initialize
    @root = SegmentNode.new
  end

  def book(start_time, end_time)
    # Query range is [start_time, end_time - 1] because end_time is non-inclusive
    if query(@root, 0, MAX_RANGE, start_time, end_time - 1)
      return false # Already booked in this range
    end

    update(@root, 0, MAX_RANGE, start_time, end_time - 1)
    true
  end

  private

  # Returns true if any part of [ql, qr] is already booked
  def query(node, l, r, ql, qr)
    return false if node.nil? || ql > r || qr < l
    return true if node.booked

    mid = l + (r - l) / 2
    query(node.left, l, mid, ql, qr) || query(node.right, mid + 1, r, ql, qr)
  end

  # Marks interval [ql, qr] as booked
  def update(node, l, r, ql, qr)
    return if ql > r || qr < l

    if ql <= l && r <= qr
      node.booked = true
      return
    end

    mid = l + (r - l) / 2
    node.left ||= SegmentNode.new
    node.right ||= SegmentNode.new

    update(node.left, l, mid, ql, qr)
    update(node.right, mid + 1, r, ql, qr)

    node.booked = node.left.booked && node.right.booked
  end
end
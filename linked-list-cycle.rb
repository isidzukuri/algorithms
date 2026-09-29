# https://leetcode.com/problems/linked-list-cycle/


# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} head
# @return {Boolean}
def hasCycle(head)
  slow = head
  fast = head

  while fast && fast.next
    slow = slow.next
    fast = fast.next.next
    
    return true if slow == fast
  end

  false
end

# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} head
# @return {Boolean}
def hasCycle(head)
    visited = []
    while nxt = head&.next do
        return true if visited.include?(nxt.object_id)
        return false if nxt.nil?
        visited.push(nxt.object_id)
        head = nxt
    end
    false
end



# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} head
# @return {Boolean}
def hasCycle(head)
    slow_pointer = head
    fast_pointer = head

    loop do
        slow_pointer = slow_pointer&.next
        fast_pointer = fast_pointer&.next&.next
        return false if fast_pointer.nil? && slow_pointer.nil?
        return true if fast_pointer == slow_pointer
    end
    false
end
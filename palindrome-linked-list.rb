# https://leetcode.com/problems/palindrome-linked-list

################# V1 ############################

# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {Boolean}
def is_palindrome(head)
    # find second part of the queue
    fast = head
    slow = head 

    while fast && fast.next do
        fast = fast.next.next
        slow = slow.next
    end

    # rotate second part of the queue
    prev = nil
    curr = slow
    while curr do
        nxt = curr.next
        curr.next = prev
        prev = curr
        curr = nxt
    end

    #compare 2 parts of the queue
    p1 = head
    p2 = prev
    while p1 && p2 do
        return false unless p1.val == p2.val
        p1 = p1.next
        p2 = p2.next
    end
    true
end


################# V2 ############################


# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {Boolean}
def is_palindrome(head)
    stack = []
    
    item = head
    loop do
        stack << item.val
        break unless item = item.next
    end

    si = 0
    ei = stack.count - 1

    while si <= ei do
        return false unless stack[si] == stack[ei]
        si +=1
        ei -=1
    end
    true
end



################# AI ############################

def is_palindrome(head)
  return true unless head&.next

  fast = head
  slow = head
  prev = nil

  # Step 1: Find middle AND reverse first half simultaneously
  while fast && fast.next
    fast = fast.next.next
    
    # Standard linked-list reversal inline
    nxt = slow.next
    slow.next = prev
    prev = slow
    slow = nxt
  end

  # Step 2: Handle odd-length lists
  # If fast is not nil, the list length is odd, so advance slow past the exact middle node
  slow = slow.next if fast

  # Step 3: Compare reversed first half (prev) with second half (slow)
  while prev
    return false if prev.val != slow.val

    prev = prev.next
    slow = slow.next
  end

  true
end
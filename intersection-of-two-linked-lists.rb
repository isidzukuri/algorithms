# https://leetcode.com/problems/intersection-of-two-linked-lists/


# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} headA
# @param {ListNode} headB
# @return {ListNode}
def getIntersectionNode(headA, headB)
    visited = {}

    while headA || headB do
        if headA
            return headA if visited[headA.object_id]
            visited[headA.object_id] = true
            headA = headA.next
        end

        if headB
            return headB if visited[headB.object_id]
            visited[headB.object_id] = true
            headB = headB.next
        end
    end
    nil
end





# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} headA
# @param {ListNode} headB
# @return {ListNode}
def getIntersectionNode(headA, headB)
    visited_a = {}
    visited_b = {}

    while headA || headB do
        if headA
            return headA if visited_b[headA.object_id]
            visited_a[headA.object_id] = true
            headA = headA.next
        end

        if headB
            return headB if visited_a[headB.object_id]
            visited_b[headB.object_id] = true
            headB = headB.next
        end
    end
    nil
end

# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} headA
# @param {ListNode} headB
# @return {ListNode}
def getIntersectionNode(headA, headB)
    visited = Set.new()

    while headA || headB do
        if headA
            return headA if visited.include?(headA.object_id)
            visited.add(headA.object_id)
            headA = headA.next
        end

        if headB
            return headB if visited.include?(headB.object_id)
            visited.add(headB.object_id)
            headB = headB.next
        end
    end
    nil
end





# AI
def getIntersectionNode(headA, headB)
  return nil unless headA && headB

  # Step 1: Find the lengths of both lists
  len_a = 0
  curr_a = headA
  while curr_a
    len_a += 1
    curr_a = curr_a.next
  end

  len_b = 0
  curr_b = headB
  while curr_b
    len_b += 1
    curr_b = curr_b.next
  end

  # Step 2: Align starting positions by advancing the longer list
  curr_a = headA
  curr_b = headB

  while len_a > len_b
    curr_a = curr_a.next
    len_a -= 1
  end

  while len_b > len_a
    curr_b = curr_b.next
    len_b -= 1
  end

  # Step 3: Advance together until they meet or hit nil
  while curr_a != curr_b
    curr_a = curr_a.next
    curr_b = curr_b.next
  end

  curr_a
end
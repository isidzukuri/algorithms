# https://leetcode.com/problems/longest-uploaded-prefix

class LUPrefix

=begin
    :type n: Integer
=end
    def initialize(n)
        @store = Array.new(n)
        @longest = 0
    end


=begin
    :type video: Integer
    :rtype: Void
=end
    def upload(video)
        @store[video-1] = true
    end


=begin
    :rtype: Integer
=end
    def longest()
        (@longest..(@store.size-1)).each do |idx|
            if @store[idx] 
                @longest += 1
            else
                break
            end
        end

        @longest
    end


end

# Your LUPrefix object will be instantiated and called as such:
# obj = LUPrefix.new(n)
# obj.upload(video)
# param_2 = obj.longest()

###################################################################### AI

class LUPrefix
    def initialize(n)
        # Using a boolean array for fast O(1) lookups
        @uploaded = Array.new(n + 1, false)
        @longest_prefix = 0
    end

    def upload(video)
        @uploaded[video] = true
        
        # Greedily advance the pointer if consecutive videos are already uploaded
        while @uploaded[@longest_prefix + 1]
            @longest_prefix += 1
        end
    end

    def longest()
        @longest_prefix
    end
end
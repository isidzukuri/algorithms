# https://leetcode.com/problems/design-ride-sharing-system

class RideSharingSystem
    def initialize()
        @riders = Array.new(1000)
        @driver_q = Queue.new
        @rider_q = []
    end


=begin
    :type rider_id: Integer
    :rtype: Void
=end
    def add_rider(rider_id)
       @riders[rider_id] = true 
       @rider_q.push(rider_id)
    end


=begin
    :type driver_id: Integer
    :rtype: Void
=end
    def add_driver(driver_id)
       @driver_q.push(driver_id)
    end


=begin
    :rtype: Integer[]
=end
    def match_driver_with_rider()
        return [-1, -1] if @driver_q.empty?
            
        # cur = 0
        while rider_id_q = @rider_q.shift
            if @riders[rider_id_q]
                @riders[rider_id_q] = nil
                rider_id = rider_id_q
                break
            end
        end

        return [-1, -1] unless rider_id

        [@driver_q.pop, rider_id]
    end


=begin
    :type rider_id: Integer
    :rtype: Void
=end
    def cancel_rider(rider_id)
       @riders[rider_id] = false
    end


end

# Your RideSharingSystem object will be instantiated and called as such:
# obj = RideSharingSystem.new()
# obj.add_rider(rider_id)
# obj.add_driver(driver_id)
# param_3 = obj.match_driver_with_rider()
# obj.cancel_rider(rider_id)

################################################################################# AI


class RideSharingSystem
    def initialize()
        @rider_q = Queue.new
        @driver_q = Queue.new
        @active_riders = {} # Tracks riders who have active, uncancelled requests
    end

=begin
    :type rider_id: Integer
    :rtype: Void
=end
    def add_rider(rider_id)
        @active_riders[rider_id] = true
        @rider_q.push(rider_id)
    end

=begin
    :type driver_id: Integer
    :rtype: Void
=end
    def add_driver(driver_id)
        @driver_q.push(driver_id)
    end

=begin
    :rtype: Integer[]
=end
    def match_driver_with_rider()
        # Return early if no drivers are available
        return [-1, -1] if @driver_q.empty?

        # Find the earliest valid (non-cancelled) rider
        valid_rider_id = nil
        while !@rider_q.empty?
            rid = @rider_q.pop
            if @active_riders[rid]
                @active_riders.delete(rid) # Remove from active list as they are now matched
                valid_rider_id = rid
                break
            end
        end

        # If no valid riders were found, return [-1, -1]
        return [-1, -1] if valid_rider_id.nil?

        # Safely pop the driver now that a match is guaranteed
        driver_id = @driver_q.pop
        [driver_id, valid_rider_id]
    end

=begin
    :type rider_id: Integer
    :rtype: Void
=end
    def cancel_rider(rider_id)
        @active_riders.delete(rider_id)
    end
end

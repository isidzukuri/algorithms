# https://leetcode.com/problems/design-parking-system

class ParkingSystem

=begin
    :type big: Integer
    :type medium: Integer
    :type small: Integer
=end
    def initialize(big, medium, small)
        @slots = [nil, big, medium, small]
    end


=begin
    :type car_type: Integer
    :rtype: Boolean
=end
    def add_car(car_type)
        if @slots[car_type] > 0 
            @slots[car_type] -=1
            true
        else
            false
        end
    end


end

# Your ParkingSystem object will be instantiated and called as such:
# obj = ParkingSystem.new(big, medium, small)
# param_1 = obj.add_car(car_type)

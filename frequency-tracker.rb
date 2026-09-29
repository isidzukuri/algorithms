# https://leetcode.com/problems/frequency-tracker

class FrequencyTracker
    def initialize()
        @numbers = {}
        @freq = {}
    end


=begin
    :type number: Integer
    :rtype: Void
=end
    def add(number)
        if @numbers[number] && @numbers[number] > 0
            @freq[@numbers[number]] -=1
        else
            @numbers[number] = 0
            @freq[1] ||= 0
        end
        @numbers[number] +=1
        @freq[@numbers[number]] ||= 0
        @freq[@numbers[number]] +=1
    end


=begin
    :type number: Integer
    :rtype: Void
=end
    def delete_one(number)
        return if @numbers[number].nil? || @numbers[number] == 0

        @freq[@numbers[number]] -=1
        @numbers[number] -= 1
        @freq[@numbers[number]] ||= 0
        @freq[@numbers[number]] +=1
    end


=begin
    :type frequency: Integer
    :rtype: Boolean
=end
    def has_frequency(frequency)
        return false if @freq[frequency].nil? || @freq[frequency] == 0
        
        true
    end


end

# Your FrequencyTracker object will be instantiated and called as such:
# obj = FrequencyTracker.new()
# obj.add(number)
# obj.delete_one(number)
# param_3 = obj.has_frequency(frequency)
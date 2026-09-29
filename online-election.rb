# https://leetcode.com/problems/online-election

class TopVotedCandidate

=begin
    :type persons: Integer[]
    :type times: Integer[]
=end
    def initialize(persons, times)
        @persons = persons
        @times = times
    end

    def count
        return if @max_at_time

        @max_at_time = {}
        @counter = {}

        max = nil
        @times.each_with_index do |time, idx|
            person = @persons[idx]
            @counter[person] ||= 0
            @counter[person] += 1

            if max
                if @counter[person] >= @counter[max]
                    max = person
                end
            else
                max = person
            end

            @max_at_time[time] = max
        end
    end


=begin
    :type t: Integer
    :rtype: Integer
=end
    def q(t)
        count
        time = less_or_eq(t)
        @max_at_time[time]
    end

    def less_or_eq(max_time)
        left = 0
        right = @times.size-1
        ans_idx = 0

        while left <= right
            mid = left + (right - left)/2

            if @times[mid] <= max_time 
                left = mid + 1 
                ans_idx = mid
            else
                right = mid - 1
            end
        end
        @times[ans_idx]
    end


end


###################################################################### AI 


class TopVotedCandidate
=begin
    :type persons: Integer[]
    :type times: Integer[]
=end
    def initialize(persons, times)
        @times = times
        @winners = [] # Parallel array: @winners[i] corresponds to the winner at @times[i]
        
        counter = Hash.new(0)
        max_person = nil
        
        times.each_with_index do |time, idx|
            person = persons[idx]
            counter[person] += 1
            
            if max_person.nil? || counter[person] >= counter[max_person]
                max_person = person
            end
            
            @winners << max_person
        end
    end

=begin
    :type t: Integer
    :rtype: Integer
=end
    def q(t)
        # Find the index of the first time greater than t
        idx = @times.bsearch_index { |time| time > t }
        
        if idx.nil?
            # t is greater than or equal to all recorded times; return the last winner
            @winners.last
        elsif idx == 0
            # t is smaller than the very first time
            @winners.first
        else
            # The largest time <= t is at idx - 1
            @winners[idx - 1]
        end
    end
end
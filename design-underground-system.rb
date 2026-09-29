# https://leetcode.com/problems/design-underground-system

class UndergroundSystem
    Checkin = Struct.new(:station_id, :time)
    
    def initialize()
        @stations = {}
        @statistic = []
        @checkins = {}
    end

    def station_id(station_name)
        @stations[station_name] ||= @stations.size
    end


=begin
    :type id: Integer
    :type station_name: String
    :type t: Integer
    :rtype: Void
=end
    def check_in(id, station_name, t)
        @checkins[id] = Checkin.new(station_id(station_name), t) 
    end


=begin
    :type id: Integer
    :type station_name: String
    :type t: Integer
    :rtype: Void
=end
    def check_out(id, station_name, t)
        checkin = @checkins[id]
        checkout_station_id = station_id(station_name)
        @statistic[checkin.station_id] ||= []
        @statistic[checkin.station_id][checkout_station_id] ||= []
        @statistic[checkin.station_id][checkout_station_id] << t - checkin.time
        @checkins.delete(id)
    end


=begin
    :type start_station: String
    :type end_station: String
    :rtype: Float
=end
    def get_average_time(start_station, end_station)
        durations = @statistic[station_id(start_station)][station_id(end_station)]
        durations.sum.to_f / durations.size
    end

end

# Your UndergroundSystem object will be instantiated and called as such:
# obj = UndergroundSystem.new()
# obj.check_in(id, station_name, t)
# obj.check_out(id, station_name, t)
# param_3 = obj.get_average_time(start_station, end_station)



############################################################################ AI

class UndergroundSystem
  def initialize
    # id => [start_station, check_in_time]
    @check_ins = {}
    
    # [start_station, end_station] => [total_time, trip_count]
    @route_stats = Hash.new { |hash, key| hash[key] = [0, 0] }
  end

  def check_in(id, station_name, t)
    @check_ins[id] = [station_name, t]
  end

  def check_out(id, station_name, t)
    start_station, check_in_time = @check_ins.delete(id)
    duration = t - check_in_time
    
    route_key = [start_station, station_name]
    stats = @route_stats[route_key]
    stats[0] += duration  # Accumulate total time
    stats[1] += 1         # Increment trip count
  end

  def get_average_time(start_station, end_station)
    total_time, count = @route_stats[[start_station, end_station]]
    total_time.to_f / count
  end
end
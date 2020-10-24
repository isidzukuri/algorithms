# Mon, Thu: 09:00-13:00, 16:00-20:00
# Wed: 16:00-20:00
# Tue, Fri: 09:00-13:00
# Sat: 09:00-14:00



require 'json'

responce = '{
     "mon_1_open": "09:00",
     "mon_1_close": "13:00",
     "tue_1_open": "09:00",
     "tue_1_close": "13:00",
     "wed_1_open": "16:00",
     "wed_1_close": "20:00",
     "thu_1_open": "09:00",
     "thu_1_close": "13:00",
     "fri_1_open": "09:00",
     "fri_1_close": "13:00",
     "sat_1_open": "09:00",
     "sat_1_close": "14:00",
     "mon_2_open": "16:00",
     "mon_2_close": "20:00",
     "thu_2_open": "16:00",
     "thu_2_close": "20:00"
}'


data = JSON.parse(responce)

schedule = {}

data.each do |key,time|
  day = key[0, 3]
  part = key[4,1].to_i

  schedule[day] ||= [{}, {}]

  current_part = schedule[day][part-1]
  
  if key.include?('close')
    current_part[:close] = time
  elsif key.include?('open')
    current_part[:open] = time
  end
end

short_schedule = {}

schedule.each do |day, data_array|
  key_parts = []
  data_array.each do |data|
    next if data.empty?

    key_parts << "#{data[:open]}-#{data[:close]}"
  end

  key = key_parts.join(', ')
  short_schedule[key] ||= []
  short_schedule[key] << day.capitalize
end

short_schedule.each do |time, days|
  p "#{days.join(', ')}: #{time}"
end

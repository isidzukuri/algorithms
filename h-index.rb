# https://leetcode.com/problems/h-index/

# @param {Integer[]} citations
# @return {Integer}
def h_index(citations)
    stats = citations.tally
    ordered_keys = stats.keys.sort_by! {|item| -item }

    max_h = 0
    higher_articles = 0
    ordered_keys.each do |published_times|
        return max_h if max_h > published_times
        if published_times > 0
            max_h += stats[published_times]
            return published_times if max_h >= published_times
        end
    end
    max_h
end





########################################## AI

# @param {Integer[]} citations
# @return {Integer}
def h_index(citations)
  n = citations.length
  counts = Array.new(n + 1, 0)

  # Count frequency of citations (capping counts > n to n)
  citations.each do |c|
    counts[c > n ? n : c] += 1
  end

  # Accumulate paper counts from highest citation count down to 0
  total_papers = 0
  n.downto(0) do |h|
    total_papers += counts[h]
    return h if total_papers >= h
  end

  0
end

# @param {Integer[]} citations
# @return {Integer}
def h_index(citations)
  citations.sort!.reverse_each.with_index.count { |c, i| c > i }
end



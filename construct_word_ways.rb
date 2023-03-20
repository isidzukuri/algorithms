# ways to construct given word from parts/

def construct_word_ways(target, parts, cache = {})
  return cache[target] if cache[target]
  return [[]] if target == ''

  result = []

  parts.each do |word|
    next if !target.start_with?(word)
    suffix = target.delete_prefix(word)

    suffix_result = construct_word_ways(suffix, parts, cache)

    result << suffix_result.map {|item| item.flatten.prepend(word) }
  end

  cache[target] = result.reject(&:empty?).flatten(1)
end



p construct_word_ways('purple', ['purp', 'p', 'ur', 'le', 'purpl']) # [["purp", "le"], ["p", "ur", "p", "le"]]
p construct_word_ways('abcdef', ['ab', 'abc', 'cd', 'def', 'abcd', 'ef', 'c']) # [["ab", "cd", "ef"], ["ab", "c", "def"], ["abc", "def"], ["abcd", "ef"]]
p construct_word_ways('skateboard', ['bo', 'rd', 'ate', 't', 'ska', 'sk', 'boar']) # []
p construct_word_ways('aaaaaaaaaaaaaaaaaaaaaaaaz', ['a', 'aa', 'aaa', 'aaaa']) # []
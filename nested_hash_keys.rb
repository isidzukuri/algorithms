require 'amazing_print'

hash_obj = {
  a: 1,
  b: [],
  c: {
    c1: [],
    c2: {
      x: '',
      z: '',
      y: ''
    }
  }
}

def nested_hash_keys(hash_obj, parent = nil)
  pathes = []

  hash_obj.each do |key, value|
    if value.is_a? Hash
      nested_hash_keys(value, [parent, key]).each do |path|
        pathes << [key].push(path).flatten
      end
    else
      pathes << key
    end
  end
  pathes
end
nested_hash_keys(hash_obj)
#[:a, :b, [:c, :c1], [:c, :c2, :x], [:c, :c2, :z], [:c, :c2, :y]]


ap nested_hash_keys(hash_obj)
p nested_hash_keys(hash_obj)


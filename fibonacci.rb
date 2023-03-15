def fibonacci(n, cache = {})
  return 1 if n <= 2

  cache[n.to_s] ||= fibonacci(n - 1, cache) + fibonacci(n - 2, cache) 
end

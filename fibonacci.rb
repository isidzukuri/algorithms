def fibonacci(n, cache = {})
  return 0 if n == 0
  return 1 if n <= 2

  cache[n.to_s] ||= fibonacci(n - 1, cache) + fibonacci(n - 2, cache) 
end

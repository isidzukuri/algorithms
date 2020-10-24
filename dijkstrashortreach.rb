# frozen_string_literal: true

require 'benchmark'
require 'ruby-prof'
require 'awesome_print'

class Node
  attr_reader :id, :edges

  def initialize(id)
    @id = id
    @edges = Set.new
  end

  def price_to(destination_id)
    pathes_to(destination_id).map do |a|
      a.inject(0) { |sum, e| sum + e.price }
    end.min || -1
  end

  protected

  def pathes_to(destination_id, vector_pathes = [])
    array_of_pathes = []

    edges.each do |edge|
      if edge.direct?(destination_id, id)
        array_of_pathes << [edge]
      else
        neighbor_node = edge.other_node(id)

        if vector_pathes.include?(edge)
          next
        else
          vector_pathes << edge
        end

        neighbor_array_of_pathes = neighbor_node.pathes_to(destination_id, vector_pathes)

        neighbor_array_of_pathes.each do |join_path|
          array_of_pathes << [edge] + join_path
        end
      end
    end

    array_of_pathes
  end
end

class Edge
  attr_reader :node1, :node2, :price, :nodes_ids

  def initialize(node1, node2, price)
    @node1 = node1
    @node2 = node2
    @price = price

    node1.edges << self
    node2.edges << self

    @nodes_ids = [node1.id, node2.id]
  end

  def direct?(from, to)
    nodes_ids.include?(from) &&
      nodes_ids.include?(to)
  end

  def other_node(from)
    to_id = (nodes_ids - [from]).first

    node1.id == to_id ? node1 : node2
  end
end

def shortestReach(n, edges, s)
  nodes = {}

  n.times do |i|
    id = i + 1
    nodes[id] = Node.new(id)
  end

  edges.each do |edge|
    from = edge[0]
    to = edge[1]
    price = edge[2]

    Edge.new(nodes[from], nodes[to], price)
  end

  pathes = []
  nodes_ids = nodes.keys

  # nodes_ids.each do |from|
  nodes_ids.each do |to|
    next if s == to

    price = nodes[s].price_to(to)

    pathes << { from: s, to: to, price: price }
  end
  # end

  uniq_pathes = []

  pathes.each do |path|
    next unless path[:from] == s || path[:to] == s

    uniq_path = uniq_pathes.select do |pt|
      (pt[:from] == path[:from] && pt[:to] == path[:to]) ||
        (pt[:to] == path[:from] && pt[:from] == path[:to])
    end.first

    if uniq_path
      uniq_path[:price] = path[:price] if uniq_path[:price] > path[:price]
    else
      uniq_pathes << path
    end
  end

  uniq_pathes.map { |pt| pt[:price] }
end

def edge_key(node1, node2)
  [node1, node2].sort.join('_')
end


class PathFinderOld
  attr_reader :edges, :nodes_count

  def initialize(n, edgs)
    @nodes_count = n
    @edges = {}

    edgs.each do |edge|
      edges[edge_key(edge[0], edge[1])] = edge
    end
  end

  def edge_key(node1, node2)
    [node1, node2].sort.join('_')
  end

  def to_all_from(start_id)
    lowest_prices = []

    nodes_count.times do |i|
      to = i + 1

      next if start_id == to

      price = array_of_pathes(start_id, to).map do |path|
        path.inject(0) { |sum, edge| sum + edges[edge][2] }
      end.min || -1

      lowest_prices << price
    end

    lowest_prices
  end

  def array_of_pathes(start_id, to, vector_pathes = [])
    arr_of_edge_keys = []

    edge = edges[edge_key(start_id, to)]

    arr_of_edge_keys << [edge_key(start_id, to)] if edge

    edg_to_neighbors = edges.select do |k, edg|
      k != edge_key(start_id, to) &&
        (edg[0] == start_id || edg[1] == start_id) &&
        !vector_pathes.include?(k)
    end

    edg_to_neighbors.each do |edg_key, edg_to_neighbor|
      new_vector_path = vector_pathes << edg_key

      start_id = edg_to_neighbor[0] == start_id ? edg_to_neighbor[1] : edg_to_neighbor[0]

      array_of_pathes(start_id, to, new_vector_path).each do |neighbor_path|
        arr_of_edge_keys << ([edg_key] + neighbor_path).sort
      end
    end

    arr_of_edge_keys
  end
end








class PathFinder
  attr_reader :edges, :nodes_count

  def initialize(n, edgs)
    @nodes_count = n
    @edges = edgs
  end

  def to_all_from(start_id)
    lowest_prices = []

    nodes_count.times do |i|
      to = i + 1

      if edges.select { |e| e[0] == to || e[1] == to }.empty?
        lowest_prices << -1
        next
      end
      next if start_id == to

      lowest_prices << array_of_pathes(start_id, to).map do |path|
        path.inject(0) { |sum, edge| sum + edge[2] }
      end.min 
    end

    lowest_prices
  end

  def array_of_pathes(start_id, to, vector_pathes = [])
    arr_of_edges = []

    edge_result = edges.select do |e|
      (e[0] == start_id && e[1] == to) ||
        (e[1] == start_id && e[0] == to)
    end

    arr_of_edges << edge_result unless edge_result.empty?

    edg_to_neighbors = edges.select do |edg|
      edg != edge_result.first &&
        (edg[0] == start_id || edg[1] == start_id) &&
        !vector_pathes.include?(edg)
    end

    edg_to_neighbors.each do |edg_to_neighbor|
      start_id = edg_to_neighbor[0] == start_id ? edg_to_neighbor[1] : edg_to_neighbor[0]

      array_of_pathes(start_id, to, vector_pathes << edg_to_neighbor).each do |neighbor_path|
        arr_of_edges << (neighbor_path << edg_to_neighbor)
      end
    end

    arr_of_edges
  end
end

def shortestReach(n, edges, s)
  PathFinder.new(n, edges).to_all_from(s)
end

n = 10
edges =
  [
    [1, 2, 24],
    [1, 4, 20],
    [3, 1, 3],
    [4, 3, 12]
  ]

# edges =
# [
# [1, 2, 10],
# [1, 3, 6],
# [2, 4, 8]
# ]
s = 1

puts Benchmark.measure { 500.times { shortestReach(n, edges, s) } }


RubyProf.start
shortestReach(n, edges, s)
result = RubyProf.stop

printer = RubyProf::FlatPrinter.new(result)
printer.print(STDOUT)



shortestReach(n, edges, s)

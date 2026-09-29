// https://leetcode.com/problems/intersection-of-two-arrays/submissions/2074949351/

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        nums1.sort_unstable();
        nums1.dedup();

        let mut result = vec![];
        for num in nums2 {
            if let Ok(index) = nums1.binary_search(&num) {
                result.push(num);
                nums1.remove(index); // Prevents duplicates in result
            }
        }
        result.sort_unstable();
        result
    }
}

// 1: Time Complexity vs. CPU Performance (Binary Search vs. HashMap)

// Operation               binary_search (sorted slice)      HashMap Lookup
// -----------------------------------------------------------------------------------------
// Big-O Complexity        O(log N)                          O(1) average
// Memory Allocations      Zero (inline in memory)           Dynamic heap allocation
// CPU Cache Friendliness  Extremely High                    Low (pointer chasing)
// Hashing Overhead        None                              High (SipHash-1,3 default)


// Benchmark Rule of Thumb
//  Dataset Size (N)       Optimal Strategy                    Why It Wins
// -------------------------------------------------------------------------------------------------------------
// N < 20                 Linear Search (slice.iter().find)   Fits in a single CPU cache line with zero branching penalty.
// 20 <= N <= 10,000      Binary Search (binary_search)       Log2(10,000) is only ~13 comparisons, hitting cache instantly.
// N > 100,000+           HashMap (pre-built & heavy queries) O(1) lookups eventually overcome hashing overhead at scale.





// 2: Feature Comparison (sort vs. sort_unstable)
// Feature                             slice::sort()               slice::sort_unstable()
// -----------------------------------------------------------------------------------------
// Stability                           Stable(preserves duplicates) Unstable (reorders duplicates)
// Algorithm                           Drifting-sort / Merge sort  Pattern-defeating quicksort
// Time Complexity (Worst)             O(N log N)                  O(N log N)
// Time Complexity (Best/Presorted)    O(N)                        O(N)
// Memory Allocation                   Extra memory (O(N) heap)    Zero extra memory (O(log N) stack)
// Performance                         Slower (copy/allocate)      Faster (20% - 50%+ boost)





// 3: Performance Comparison Matrix (Deduplicating i32)
// Metric                 Vec::sort_unstable + dedup    HashSet<i32> (Default)
// ---------------------------------------------------------------------------
// Small Data (N < 1k)    Fastest                       Slowest               
// Medium Data (N < 100k) Fastest                       Slower                
// Massive Data (N > 1M)  Fast                          Faster                
// Preserves Order?       No                            No                    
// Memory Footprint       Low (In-place)                High (O(N) capacity)  


impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut seen = [false; 1001];
        for num in nums1 {
            seen[num as usize] = true;
        }

        let mut result = Vec::new();
        for num in nums2 {
            if seen[num as usize] {
                result.push(num);
                seen[num as usize] = false; // Prevents duplicate additions to result
            }
        }

        result
    }
}
# DP Solutions — Part 5: Bitmask DP (Java)
### 12 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

## 1. Can I Win

**Problem:** Two players alternately pick numbers from 1 to `maxChoosableInteger` (no repeats), adding to a running total. The first player to make the total ≥ `desiredTotal` wins. Determine if the first player can force a win with optimal play.

**Example:**
```
Input: maxChoosableInteger = 10, desiredTotal = 11
Output: false
Explanation: No matter what the first player picks, the second player can
always force the total to reach 11 first.
```

**Brute force:** simulate every game tree without memoizing visited states → exponential, revisits identical states repeatedly.
**Optimized:** memoize on the bitmask of "used numbers" — `dp[mask] = can the player to move force a win from this state`.
```java
class CanIWin {
    public boolean canIWin(int maxChoosableInteger, int desiredTotal) {
        if (desiredTotal <= 0) return true;
        int sum = maxChoosableInteger * (maxChoosableInteger + 1) / 2;
        if (sum < desiredTotal) return false;

        Map<Integer, Boolean> memo = new HashMap<>();
        return canWin(maxChoosableInteger, desiredTotal, 0, memo);
    }

    private boolean canWin(int maxChoosable, int remaining, int usedMask, Map<Integer, Boolean> memo) {
        if (memo.containsKey(usedMask)) return memo.get(usedMask);

        for (int i = 1; i <= maxChoosable; i++) {
            int bit = 1 << (i - 1);
            if ((usedMask & bit) != 0) continue;
            // if this move wins outright, OR opponent can't win after this move
            if (i >= remaining || !canWin(maxChoosable, remaining - i, usedMask | bit, memo)) {
                memo.put(usedMask, true);
                return true;
            }
        }
        memo.put(usedMask, false);
        return false;
    }
}
```
**Complexity:** O(2ⁿ · n) time (n = maxChoosableInteger, 2ⁿ distinct masks each trying n moves), O(2ⁿ) space.

---

## 2. Partition to K Equal Sum Subsets

**Problem:** Given an array and an integer k, determine if it's possible to partition the array into k non-empty subsets each with equal sum.

**Example:**
```
Input: nums = [4,3,2,3,5,2,1], k = 4
Output: true
Explanation: Sum = 20, target per subset = 5. Valid partition: (5), (1,4), (2,3), (2,3).
```

**Brute force:** try every way to assign each element to one of k buckets → O(k^n).
**Optimized:** bitmask DP — `dp[mask] = current bucket's running sum (mod target) once the elements in mask have been placed`, processing masks in increasing numeric order so every transition source is already resolved.
```java
class PartitionKEqualSumSubsets {
    public boolean canPartitionKSubsets(int[] nums, int k) {
        int sum = Arrays.stream(nums).sum();
        if (sum % k != 0) return false;
        int target = sum / k;
        Arrays.sort(nums);
        int n = nums.length;
        if (nums[n - 1] > target) return false;

        int[] dp = new int[1 << n]; // dp[mask] = running sum of the CURRENT bucket, mod target
        Arrays.fill(dp, -1);
        dp[0] = 0;

        for (int mask = 0; mask < (1 << n); mask++) {
            if (dp[mask] == -1) continue;
            for (int i = 0; i < n; i++) {
                if ((mask & (1 << i)) == 0 && dp[mask] + nums[i] <= target) {
                    int newMask = mask | (1 << i);
                    if (dp[newMask] == -1) {
                        dp[newMask] = (dp[mask] + nums[i]) % target; // hits target -> resets to 0, new bucket
                    }
                }
            }
        }
        return dp[(1 << n) - 1] == 0;
    }
}
```
**Complexity:** O(n · 2ⁿ) time, O(2ⁿ) space. (An alternative DFS+pruning approach with sorted-descending order + early termination is often faster in practice despite the same worst-case bound.)

---

## 3. Stickers to Spell Word

**Problem:** Given a list of stickers (each a string of lowercase letters) and a target string, find the minimum number of stickers needed (letters can be reused within a sticker, extra letters ignored) to form the target by cutting out individual letters. Return -1 if impossible.

**Example:**
```
Input: stickers = ["with","example","science"], target = "thehat"
Output: 3
Explanation: "with" supplies t,h; "example" supplies a; combine with another
"with" for the second t and h — total stickers used = 3.
```

**Brute force:** try every combination of stickers, greedily check coverage → exponential.
**Optimized:** bitmask DP over target's letters — `dp[mask] = min stickers to cover the target letters represented by mask`.
```java
class StickersToSpellWord {
    public int minStickers(String[] stickers, String target) {
        int n = target.length();
        int[] dp = new int[1 << n];
        Arrays.fill(dp, -1);
        dp[0] = 0;

        for (int mask = 0; mask < (1 << n); mask++) {
            if (dp[mask] == -1) continue;
            for (String sticker : stickers) {
                int newMask = mask;
                int[] count = new int[26];
                for (char c : sticker.toCharArray()) count[c - 'a']++;

                for (int i = 0; i < n; i++) {
                    if ((newMask & (1 << i)) != 0) continue; // already covered
                    char c = target.charAt(i);
                    if (count[c - 'a'] > 0) {
                        count[c - 'a']--;
                        newMask |= (1 << i);
                    }
                }
                if (dp[newMask] == -1 || dp[newMask] > dp[mask] + 1) {
                    dp[newMask] = dp[mask] + 1;
                }
            }
        }
        return dp[(1 << n) - 1];
    }
}
```
**Complexity:** O(2ⁿ · S · n) time (n = target length, S = number of stickers), O(2ⁿ) space.

---

## 4. Shortest Path Visiting All Nodes

**Problem:** Given an undirected connected graph, find the length of the shortest path that visits every node at least once (can revisit nodes/edges, can start anywhere).

**Example:**
```
Input: graph = [[1,2,3],[0],[0],[0]]
Output: 4
Explanation: One shortest path: 1 -> 0 -> 2 -> 0 -> 3, length 4.
```

**Brute force:** DFS trying every path, no memo → exponential, revisits identical (node, visited-set) states.
**Optimized:** BFS over states `(node, visitedMask)` — multi-source BFS starting from every node with only itself visited.
```java
class ShortestPathVisitingAllNodes {
    public int shortestPathLength(int[][] graph) {
        int n = graph.length;
        if (n == 1) return 0;
        int fullMask = (1 << n) - 1;

        Queue<int[]> queue = new LinkedList<>(); // {node, mask}
        boolean[][] visited = new boolean[n][1 << n];

        for (int i = 0; i < n; i++) {
            queue.offer(new int[]{i, 1 << i});
            visited[i][1 << i] = true;
        }

        int steps = 0;
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int s = 0; s < size; s++) {
                int[] curr = queue.poll();
                int node = curr[0], mask = curr[1];
                if (mask == fullMask) return steps;

                for (int nei : graph[node]) {
                    int newMask = mask | (1 << nei);
                    if (!visited[nei][newMask]) {
                        visited[nei][newMask] = true;
                        queue.offer(new int[]{nei, newMask});
                    }
                }
            }
            steps++;
        }
        return -1;
    }
}
```
**Complexity:** O(n² · 2ⁿ) time, O(n · 2ⁿ) space.

---

## 5. Smallest Sufficient Team

**Problem:** Given a list of required skills and a list of people each with a subset of skills, find the smallest team of people (by index) that collectively covers all required skills.

**Example:**
```
Input: req_skills = ["java","nodejs","reactjs"], 
       people = [["java"],["nodejs"],["nodejs","reactjs"]]
Output: [0,2]
Explanation: Person 0 covers "java", person 2 covers "nodejs" and "reactjs" — 
together they cover everything with just 2 people.
```

**Brute force:** try every subset of people, check skill coverage → O(2^people).
**Optimized:** bitmask DP over skill coverage — `dp[skillMask] = smallest team (as a list of people) achieving this skill coverage`.
```java
class SmallestSufficientTeam {
    public int[] smallestSufficientTeam(String[] req_skills, List<List<String>> people) {
        int m = req_skills.length;
        Map<String, Integer> skillIndex = new HashMap<>();
        for (int i = 0; i < m; i++) skillIndex.put(req_skills[i], i);

        int n = people.size();
        int[] personMask = new int[n];
        for (int i = 0; i < n; i++) {
            for (String skill : people.get(i)) {
                personMask[i] |= (1 << skillIndex.get(skill));
            }
        }

        int fullMask = (1 << m) - 1;
        long[] dp = new long[1 << m]; // dp[mask] encodes the team as a bitmask of people (packed in a long)
        Arrays.fill(dp, -1);
        dp[0] = 0;

        for (int mask = 0; mask <= fullMask; mask++) {
            if (dp[mask] == -1) continue;
            for (int i = 0; i < n; i++) {
                int newMask = mask | personMask[i];
                if (newMask == mask) continue;
                long newTeam = dp[mask] | (1L << i);
                if (dp[newMask] == -1 || Long.bitCount(dp[newMask]) > Long.bitCount(newTeam)) {
                    dp[newMask] = newTeam;
                }
            }
        }

        long teamBits = dp[fullMask];
        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            if ((teamBits & (1L << i)) != 0) result.add(i);
        }
        return result.stream().mapToInt(Integer::intValue).toArray();
    }
}
```
**Complexity:** O(2ᵐ · n) time (m = number of required skills, typically ≤ 16), O(2ᵐ) space.

---

## 6. Maximum Students Taking Exam

**Problem:** Given a classroom seat matrix ('.' = usable, '#' = broken), seat students such that no student can see another's exam: no student can have a directly adjacent left/right neighbor, or diagonal neighbors in the row immediately in front/behind. Maximize seated students.

**Example:**
```
Input: seats = [[1,0,1,0,1],[0,0,1,0,0],[1,0,1,0,1]]  (1='.', 0='#' conceptually)
Output: 4
Explanation: A valid seating of 4 students avoiding all forbidden adjacency
patterns is achievable.
```

**Brute force:** try every subset of valid seats per row combined across rows → exponential.
**Optimized:** bitmask DP row by row — `dp[row][mask] = max students seated through this row with this row's seating pattern = mask`, checking compatibility with the previous row's mask (no same-row adjacency, no diagonal adjacency with prev row).
```java
class MaxStudentsTakingExam {
    public int maxStudents(char[][] seats) {
        int rows = seats.length, cols = seats[0].length;
        int[] validMask = new int[rows];
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (seats[r][c] == '.') validMask[r] |= (1 << c);
            }
        }

        Map<Integer, Integer>[] dp = new HashMap[rows];
        for (int r = 0; r < rows; r++) dp[r] = new HashMap<>();

        for (int mask = 0; mask <= validMask[0]; mask++) {
            if ((mask & validMask[0]) != mask) continue;
            if ((mask & (mask << 1)) != 0) continue; // no horizontal adjacency
            dp[0].put(mask, Integer.bitCount(mask));
        }

        for (int r = 1; r < rows; r++) {
            for (int mask = 0; mask <= validMask[r]; mask++) {
                if ((mask & validMask[r]) != mask) continue;
                if ((mask & (mask << 1)) != 0) continue;

                for (Map.Entry<Integer, Integer> prevEntry : dp[r - 1].entrySet()) {
                    int prevMask = prevEntry.getKey();
                    if ((mask & (prevMask << 1)) != 0) continue; // diagonal left
                    if ((mask & (prevMask >> 1)) != 0) continue; // diagonal right
                    int val = prevEntry.getValue() + Integer.bitCount(mask);
                    dp[r].merge(mask, val, Math::max);
                }
            }
        }

        int result = 0;
        for (int val : dp[rows - 1].values()) result = Math.max(result, val);
        return result;
    }
}
```
**Complexity:** O(rows · 2^(2·cols)) time worst case (pairs of row masks), O(2^cols) space per row — feasible since cols is typically ≤ 8 in this problem's constraints.

---

## 7. Number of Ways to Wear Different Hats to Each Other

**Problem:** Given `n` people and their preferred hats (numbered 1-40), count the ways to assign each person a distinct preferred hat.

**Example:**
```
Input: hats = [[3,4],[4,5],[5]]
Output: 1
Explanation: Only one valid assignment: person0→3, person1→4, person2→5.
```

**Brute force:** try every permutation of hat assignments → O(40!).
**Optimized:** bitmask DP over PEOPLE (not hats, since hats ≤ 40 but people ≤ 10 typically) — `dp[hat][peopleMask] = ways to assign hats 1..hat covering peopleMask`.
```java
class NumWaysWearDifferentHats {
    public int numberWays(List<List<Integer>> hats) {
        long MOD = 1_000_000_007;
        int n = hats.size();
        List<List<Integer>> hatToPeople = new ArrayList<>();
        for (int i = 0; i <= 40; i++) hatToPeople.add(new ArrayList<>());
        for (int p = 0; p < n; p++) {
            for (int hat : hats.get(p)) hatToPeople.get(hat).add(p);
        }

        int fullMask = (1 << n) - 1;
        long[][] dp = new long[41][1 << n];
        dp[0][0] = 1;

        for (int hat = 1; hat <= 40; hat++) {
            for (int mask = 0; mask <= fullMask; mask++) {
                dp[hat][mask] = dp[hat - 1][mask]; // don't use this hat
                for (int p : hatToPeople.get(hat)) {
                    if ((mask & (1 << p)) != 0) {
                        dp[hat][mask] = (dp[hat][mask] + dp[hat - 1][mask ^ (1 << p)]) % MOD;
                    }
                }
            }
        }
        return (int) dp[40][fullMask];
    }
}
```
**Complexity:** O(40 · 2ⁿ · n) time (n = number of people, ≤ 10 typically), O(40 · 2ⁿ) space.

---

## 8. Minimum Cost to Connect Two Groups of Points

**Problem:** Given cost matrix between points of group1 and group2, connect every point in both groups (each point in group1 connects to ≥1 point in group2 and vice versa) at minimum total cost.

**Example:**
```
Input: cost = [[15,96],[36,2]]
Output: 17
Explanation: Connect group1[0]-group2[1] (cost 96)... actually optimal is 
group1[0]-group2[0] (15) and group1[1]-group2[1] (2), total 17.
```

**Brute force:** try every subset of edges, check both-side coverage → exponential.
**Optimized:** bitmask DP over group2's coverage — `dp[i][mask] = min cost connecting first i points of group1, with mask = which group2 points are covered so far`.
```java
class MinCostConnectTwoGroups {
    public int connectTwoGroups(List<List<Integer>> cost) {
        int size1 = cost.size(), size2 = cost.get(0).size();
        int fullMask = (1 << size2) - 1;

        int[] minCostForPoint = new int[size2]; // cheapest single connection for each group2 point
        Arrays.fill(minCostForPoint, Integer.MAX_VALUE);
        for (int j = 0; j < size2; j++) {
            for (int i = 0; i < size1; i++) {
                minCostForPoint[j] = Math.min(minCostForPoint[j], cost.get(i).get(j));
            }
        }

        Integer[][] memo = new Integer[size1][1 << size2];
        return solve(0, 0, cost, size1, size2, fullMask, minCostForPoint, memo);
    }

    private int solve(int i, int mask, List<List<Integer>> cost, int size1, int size2,
                       int fullMask, int[] minCostForPoint, Integer[][] memo) {
        if (i == size1) {
            int remaining = 0;
            for (int j = 0; j < size2; j++) {
                if ((mask & (1 << j)) == 0) remaining += minCostForPoint[j];
            }
            return remaining;
        }
        if (memo[i][mask] != null) return memo[i][mask];

        int best = Integer.MAX_VALUE;
        for (int j = 0; j < size2; j++) {
            int newMask = mask | (1 << j);
            int c = cost.get(i).get(j) + solve(i + 1, newMask, cost, size1, size2, fullMask, minCostForPoint, memo);
            best = Math.min(best, c);
        }
        memo[i][mask] = best;
        return best;
    }
}
```
**Complexity:** O(size1 · 2^size2 · size2) time, O(size1 · 2^size2) space.

---

## 9. Maximum Number of Achievable Transfer Requests

**Problem:** Given `n` buildings and a list of employee transfer requests (from, to), find the max number of requests that can be satisfied simultaneously such that every building's net employee count stays the same (in-degree = out-degree for every building).

**Example:**
```
Input: n = 5, requests = [[0,1],[1,0],[0,1],[1,2],[2,1]]
Output: 4
Explanation: Choosing requests 0,1,2,3 keeps every building balanced 
(one request must be dropped since 5 requests can't all balance simultaneously).
```

**Brute force / actual approach:** with request counts typically small (≤16), try every subset (bitmask), check balance feasibility — this IS the standard/optimal approach for this problem's constraints.
```java
class MaxAchievableTransferRequests {
    public int maximumRequests(int n, int[][] requests) {
        int m = requests.length;
        int best = 0;

        for (int mask = 0; mask < (1 << m); mask++) {
            int[] netChange = new int[n];
            int count = Integer.bitCount(mask);
            if (count <= best) continue; // pruning: can't beat current best

            for (int i = 0; i < m; i++) {
                if ((mask & (1 << i)) != 0) {
                    netChange[requests[i][0]]--;
                    netChange[requests[i][1]]++;
                }
            }
            boolean balanced = true;
            for (int change : netChange) {
                if (change != 0) { balanced = false; break; }
            }
            if (balanced) best = count;
        }
        return best;
    }
}
```
**Complexity:** O(2ᵐ · (m + n)) time (m = number of requests, typically ≤ 16), O(n) space.

---

## 10. Distribute Repeating Integers

**Problem:** Given an array `nums` and a list of `quantity` requirements from customers, determine if it's possible to distribute the array's values (grouped by matching value) such that customer `i` receives `quantity[i]` copies of a SINGLE distinct value, with no value split across customers.

**Example:**
```
Input: nums = [1,2,3,4], quantity = [2]
Output: false
Explanation: No single value appears twice in nums, so a request for 2 
identical items can't be satisfied.
```

**Brute force:** try every assignment of customers to distinct values → exponential.
**Optimized:** bitmask DP over CUSTOMERS (since quantity list is typically ≤ 10) — `dp[valueIndex][customerMask] = can we satisfy customerMask using values from valueIndex onward`, trying every subset of the mask to assign to the current value's available count.
```java
class DistributeRepeatingIntegers {
    public boolean canDistribute(int[] nums, int[] quantity) {
        Map<Integer, Integer> countMap = new HashMap<>();
        for (int num : nums) countMap.merge(num, 1, Integer::sum);
        List<Integer> counts = new ArrayList<>(countMap.values());

        int m = quantity.length;
        int fullMask = (1 << m) - 1;
        int[] sumForMask = new int[1 << m];
        for (int mask = 1; mask <= fullMask; mask++) {
            int lowBit = mask & (-mask);
            int idx = Integer.numberOfTrailingZeros(lowBit);
            sumForMask[mask] = sumForMask[mask ^ lowBit] + quantity[idx];
        }

        Boolean[][] memo = new Boolean[counts.size()][1 << m];
        return solve(0, fullMask, counts, sumForMask, fullMask, memo);
    }

    private boolean solve(int idx, int mask, List<Integer> counts, int[] sumForMask, int fullMask, Boolean[][] memo) {
        if (mask == 0) return true;
        if (idx == counts.size()) return false;
        if (memo[idx][mask] != null) return memo[idx][mask];

        boolean result = solve(idx + 1, mask, counts, sumForMask, fullMask, memo); // skip this value
        if (!result) {
            // try every non-empty submask of `mask` as the set of customers served by this value
            for (int sub = mask; sub > 0 && !result; sub = (sub - 1) & mask) {
                if (sumForMask[sub] <= counts.get(idx)) {
                    result = solve(idx + 1, mask ^ sub, counts, sumForMask, fullMask, memo);
                }
            }
        }
        memo[idx][mask] = result;
        return result;
    }
}
```
**Complexity:** O(distinctValues · 3^m) time (submask enumeration over all masks sums to 3^m), O(distinctValues · 2^m) space.

---

## 11. Maximize Grid Happiness

**Problem:** Place introverts and extroverts into an `m x n` grid (each cell empty or one person) to maximize total happiness. Base happiness: introvert +120/-30, extrovert +40/-10, adjusted by ±adjacency rules with neighbors.

**Example:**
```
Input: m = 2, n = 3, introvertsCount = 1, extrovertsCount = 2
Output: 240
Explanation: An optimal placement yields total happiness 240.
```

**Brute force:** try every placement of every person type in every cell → exponential.
**Optimized:** bitmask DP over the previous row's placement (each cell: empty/introvert/extrovert encoded in base-3 per column), processing cell by cell.
```java
class MaximizeGridHappiness {
    public int getMaxGridHappiness(int m, int n, int introvertsCount, int extrovertsCount) {
        int[] pow3 = new int[n + 1];
        pow3[0] = 1;
        for (int i = 1; i <= n; i++) pow3[i] = pow3[i - 1] * 3;

        Map<Long, Integer> memo = new HashMap<>();
        return solve(0, 0, introvertsCount, extrovertsCount, m, n, pow3, memo);
    }

    // state: previous n placements encoded base-3 (0=empty,1=introvert,2=extrovert)
    private int solve(int pos, int mask, int intro, int extro, int m, int n, int[] pow3, Map<Long, Integer> memo) {
        if (pos == m * n) return 0;
        if (intro == 0 && extro == 0) return 0;

        long key = ((long) pos * pow3[n] + mask) * 100 + intro * 10 + extro;
        if (memo.containsKey(key)) return memo.get(key);

        int row = pos / n, col = pos % n;
        int up = (row == 0) ? 0 : (mask / pow3[n - 1]) % 3;
        int left = (col == 0) ? 0 : mask % 3;

        int best = solve(pos + 1, (mask * 3) % pow3[n], intro, extro, m, n, pow3, memo); // leave empty

        if (intro > 0) {
            int happiness = 120;
            if (up == 1) happiness += -30 - 30;
            else if (up == 2) happiness += 40 - 30;
            if (left == 1) happiness += -30 - 30;
            else if (left == 2) happiness += 40 - 30;
            int newMask = (mask * 3 + 1) % pow3[n];
            best = Math.max(best, happiness + solve(pos + 1, newMask, intro - 1, extro, m, n, pow3, memo));
        }
        if (extro > 0) {
            int happiness = 40;
            if (up == 1) happiness += -30 + 40;
            else if (up == 2) happiness += 40 + 40;
            if (left == 1) happiness += -30 + 40;
            else if (left == 2) happiness += 40 + 40;
            int newMask = (mask * 3 + 2) % pow3[n];
            best = Math.max(best, happiness + solve(pos + 1, newMask, intro, extro - 1, m, n, pow3, memo));
        }
        memo.put(key, best);
        return best;
    }
}
```
**Complexity:** O(m·n · 3ⁿ · intro · extro) time, O(m·n · 3ⁿ · intro · extro) space — feasible since n ≤ 5 in this problem's constraints.

---

## 12. Find Minimum Time to Finish All Jobs

**Problem:** Assign `n` jobs to `k` workers, minimizing the maximum total job-time assigned to any single worker.

**Example:**
```
Input: jobs = [3,2,3], k = 3
Output: 3
Explanation: Assign each job to its own worker: max load = 3.
```

**Brute force:** try every assignment of jobs to workers → O(k^n).
**Optimized Solution 1 — Bitmask DP:** `dp[worker][mask] = min possible max-load using first `worker` workers to cover job-set `mask``.
```java
class MinTimeFinishJobsDP {
    public int minimumTimeRequired(int[] jobs, int k) {
        int n = jobs.length;
        int[] sumForMask = new int[1 << n];
        for (int mask = 1; mask < (1 << n); mask++) {
            int lowBit = Integer.numberOfTrailingZeros(mask & (-mask));
            sumForMask[mask] = sumForMask[mask ^ (1 << lowBit)] + jobs[lowBit];
        }

        int[][] dp = new int[k][1 << n];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE);
        for (int mask = 0; mask < (1 << n); mask++) dp[0][mask] = sumForMask[mask];

        for (int worker = 1; worker < k; worker++) {
            for (int mask = 0; mask < (1 << n); mask++) {
                for (int sub = mask; sub > 0; sub = (sub - 1) & mask) {
                    if (dp[worker - 1][mask ^ sub] == Integer.MAX_VALUE) continue;
                    int candidate = Math.max(dp[worker - 1][mask ^ sub], sumForMask[sub]);
                    dp[worker][mask] = Math.min(dp[worker][mask], candidate);
                }
            }
        }
        return dp[k - 1][(1 << n) - 1];
    }
}
```
**Complexity:** O(k · 3ⁿ) time (submask enumeration), O(k · 2ⁿ) space.

### Optimized Solution 2 — Binary Search on Answer + Backtracking (often faster in practice)
```java
class MinTimeFinishJobsBS {
    public int minimumTimeRequired(int[] jobs, int k) {
        Arrays.sort(jobs);
        // reverse to descending for better pruning
        for (int i = 0, j = jobs.length - 1; i < j; i++, j--) {
            int t = jobs[i]; jobs[i] = jobs[j]; jobs[j] = t;
        }

        int lo = jobs[0], hi = Arrays.stream(jobs).sum();
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            int[] workerLoads = new int[k];
            if (canAssign(jobs, 0, workerLoads, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    private boolean canAssign(int[] jobs, int idx, int[] workerLoads, int limit) {
        if (idx == jobs.length) return true;
        for (int i = 0; i < workerLoads.length; i++) {
            if (workerLoads[i] + jobs[idx] <= limit) {
                workerLoads[i] += jobs[idx];
                if (canAssign(jobs, idx + 1, workerLoads, limit)) return true;
                workerLoads[i] -= jobs[idx];
            }
            if (workerLoads[i] == 0) break; // pruning: identical empty-worker states
        }
        return false;
    }
}
```
**Complexity:** O(log(sum) · k^n) worst case theoretically, but pruning (sorted descending + skip-duplicate-empty-worker) makes this very fast in practice — often outperforms the bitmask DP for this problem's typical constraints.

---

## 🎯 Part 5 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Can I Win | O(2ⁿ·n) | O(2ⁿ) |
| 2 | Partition K Equal Sum Subsets | O(n·2ⁿ) | O(2ⁿ) |
| 3 | Stickers to Spell Word | O(2ⁿ·S·n) | O(2ⁿ) |
| 4 | Shortest Path Visiting All Nodes | O(n²·2ⁿ) | O(n·2ⁿ) |
| 5 | Smallest Sufficient Team | O(2ᵐ·n) | O(2ᵐ) |
| 6 | Max Students Taking Exam | O(rows·2^(2cols)) | O(2^cols) |
| 7 | Ways to Wear Different Hats | O(40·2ⁿ·n) | O(40·2ⁿ) |
| 8 | Min Cost Connect Two Groups | O(size1·2^size2·size2) | O(size1·2^size2) |
| 9 | Max Achievable Transfer Requests | O(2ᵐ·(m+n)) | O(n) |
| 10 | Distribute Repeating Integers | O(values·3ᵐ) | O(values·2ᵐ) |
| 11 | Maximize Grid Happiness | O(mn·3ⁿ·intro·extro) | O(mn·3ⁿ·intro·extro) |
| 12 | Min Time Finish Jobs (bitmask DP) | O(k·3ⁿ) | O(k·2ⁿ) |
| 12b | Min Time Finish Jobs (binary search) | O(log(sum)·kⁿ) w/ pruning | O(k) |

---

**Next: Part 6 — Digit DP (3 problems).** Say "continue" to proceed, or name a category to jump to.

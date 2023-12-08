(ns day08
  (:require
    [utils :as utils]))


(defn parse-node
  [line]
  (let [[_ from left right] (re-matches #"(\w+) = \((\w+), (\w+)\)" line)]
    [from {\L left, \R right}]))


(defn parse-node-map
  [block]
  (->> block
       utils/to-lines
       (map parse-node)))


(defn parse-input
  [input]
  (let [[instructions node-map] (utils/to-blocks input)]
    {:instructions instructions,
     :node-map (into {} (parse-node-map node-map))}))


(defn solve-1
  "Follow the instructions until we reach ZZZ starting at AAA"
  [{instructions :instructions, node-map :node-map}]
  (loop [current "AAA"
         instruction (cycle (seq instructions))
         n-moves 0]
    (if (= current "ZZZ")
      n-moves
      (recur ((node-map current) (first instruction))
             (rest instruction)
             (inc n-moves)))))


(defn part-1
  [input]
  (->> input
       parse-input
       solve-1))


(part-1 (utils/read-input "inputs/day08.txt"))


;; Not super happy with this part as you need to find the structure by guessing
;; and running some experiments... It is not in the problem text, and is in no
;; way a feature of the general problem.
(defn find-loop
  "The solutions loop, so if you find any solution it will loop with the same frequency"
  [{instructions :instructions, node-map :node-map} start]
  (loop [current start
         dirs (cycle (seq instructions))
         idx 0]
    (if (= (last current) \Z)
      idx
      (recur ((node-map current) (first dirs)) (rest dirs) (inc idx)))))


(defn find-all-loops
  "Run find-loops on all start-positions, all pos ending in A"
  [input]
  (map #(find-loop input %) (filter #(= (last %) \A) (keys (input :node-map)))))


(defn gcd
  [a b]
  (.gcd (biginteger a) (biginteger b)))


(defn smallest-multiple
  "Find the smallest multiple of all loops we need to go through before we find a match"
  [numbers]
  (let [x (reduce gcd numbers)] (* (apply * (map #(/ % x) numbers)) x)))


(defn part-2
  [input]
  (->> input
       parse-input
       find-all-loops
       smallest-multiple))


(part-2 (utils/read-input "inputs/day08.txt"))


;; Small unused utility function used to discover the structure of the data
(defn debug-2
  [{instructions :instructions, node-map :node-map}]
  (for [c (filter #(= (last %) \A) (keys node-map))]
    (loop [current c
           idx 0
           prev #{}]
      (if (prev [idx current])
        (filter #(= (last (last %)) \Z) prev)
        (recur ((node-map current) (get instructions idx))
               (mod (inc idx) (count instructions))
               (conj prev [idx current]))))))

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
  (let [[dirs node-map] (utils/to-blocks input)]
    {:dirs dirs, :node-map (into {} (parse-node-map node-map))}))


(defn count-moves-to-target
  "Starting at start, following the map and directions, count the number of
  moves until we reach the predicate."
  [{dirs :dirs, node-map :node-map} start pred]
  (loop [current start
         dirs (cycle (seq dirs))
         n-moves 0]
    (if (pred current)
      n-moves
      (recur ((node-map current) (first dirs)) (rest dirs) (inc n-moves)))))


(defn solve-1
  "Follow the instructions until we reach ZZZ starting at AAA"
  [problem]
  (count-moves-to-target problem "AAA" #(= % "ZZZ")))


(defn part-1
  [input]
  (->> input
       parse-input
       solve-1))


(part-1 (utils/read-input "inputs/day08.txt"))


;; Not super happy with this part as you need to find the structure by guessing
;; and running some experiments... It is not in the problem text, and is in no
;; way a feature of the general problem.
(defn find-loops
  "Find loops for every starting positions. The data is so that when we reach
  and ending position we loop perfectly."
  [input]
  (->> (input :node-map)
       keys
       (filter #(= (last %) \A))
       (map (fn [start] (count-moves-to-target input start #(= (last %) \Z))))))


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
       find-loops
       smallest-multiple))


(part-2 (utils/read-input "inputs/day08.txt"))


;; Small unused utility function used to discover the structure of the data
(defn debug-2
  [{dirs :dirs, node-map :node-map}]
  (for [c (filter #(= (last %) \A) (keys node-map))]
    (loop [current c
           idx 0
           prev #{}]
      (if (prev [idx current])
        (filter #(= (last (last %)) \Z) prev)
        (recur ((node-map current) (get dirs idx))
               (mod (inc idx) (count dirs))
               (conj prev [idx current]))))))

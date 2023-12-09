(ns day09
  (:require
    [utils :as utils]))


(defn parse-integers
  [line]
  (map parse-long (re-seq #"-?\d+" line)))


(defn parse-input
  [input]
  (->> input
       utils/to-lines
       (map parse-integers)))


(defn diff-seqs
  "Find the all diffs between elements of a sequence until they are all zero"
  [nums]
  (loop [coll nums
         seqs []]
    (if (every? zero? coll)
      (conj seqs coll)
      (recur (map #(- %2 %1) (butlast coll) (rest coll)) (conj seqs coll)))))


(defn next-in-sequence
  "Find the previous element in the sequence by sequentially adding the last
  item in the diff sequences"
  [nums]
  (reduce + (map last (diff-seqs nums))))


(defn part-1
  [input]
  (->> input
       parse-input
       (map next-in-sequence)
       (apply +)))


(part-1 (utils/read-input "inputs/day09.txt"))


(defn previous-in-sequence
  "Find the previous element in the sequence by sequentially subtracting the
  first item in the diff sequences"
  [nums]
  (reduce #(- %2 %1) (reverse (map first (diff-seqs nums)))))


(defn part-2
  [input]
  (->> input
       parse-input
       (map previous-in-sequence)
       (apply +)))


(part-2 (utils/read-input "inputs/day09.txt"))

(ns day11
  (:require
    [clojure.math.combinatorics :as combo]
    [utils :as utils]))


(defn cols
  "Get the colums as lists, a transpose"
  [matrix]
  (partition (count matrix) (apply interleave matrix)))


(defn empty-rows
  "Get the indices of the rows with only empty space"
  [matrix]
  (map first
       (filter (fn [x] (every? #(= % \.) (last x))) (utils/indexed matrix))))


(defn find-loc
  "Find the location of a single star taking expansion into account"
  [[r c] rows cols expansion]
  [(+ r (* (count (filter #(> r %) rows)) expansion))
   (+ c (* (count (filter #(> c %) cols)) expansion))])


(defn find-star-locs
  "Find the locations of all the stars taking expansion into account"
  ([matrix] (find-star-locs 1 matrix))
  ([expansion matrix]
   (let [rows (empty-rows matrix)
         cols (empty-rows (cols matrix))]
     (for [r (range (count matrix))
           c (range (count (first matrix)))
           :when (= (get-in matrix [r c]) \#)]
       (find-loc [r c] rows cols expansion)))))


(defn calculate-distances
  "Calculate the distance between every pair of stars with the taxi-driver-metric"
  [locs]
  (map (fn [[[c1 r1] [c2 r2]]] (+ (abs (- c1 c2)) (abs (- r1 r2))))
       (combo/combinations locs 2)))


(defn part-1
  [input]
  (->> input
       utils/to-matrix
       find-star-locs
       calculate-distances
       (apply +)))


(part-1 (utils/read-input "inputs/day11.txt"))


(defn part-2
  [input]
  (->> input
       utils/to-matrix
       (find-star-locs (- 1000000 1))
       calculate-distances
       (apply +)))


(part-2 (utils/read-input "inputs/day11.txt"))

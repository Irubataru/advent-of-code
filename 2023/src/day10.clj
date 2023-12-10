(ns day10
  (:require
    [clojure.math :refer [ceil floor sqrt]]
    [clojure.string :as str]
    [utils :as utils]))


(defn parse-input
  [input]
  (utils/to-matrix input))


(def pipe-types
  {\| [[-1 0] [1 0]],
   \- [[0 -1] [0 1]],
   \L [[0 1] [-1 0]],
   \J [[0 -1] [-1 0]],
   \7 [[0 -1] [1 0]],
   \F [[0 1] [1 0]]})


(defn next-move
  [pipes prev curr]
  (let [[x1 x2] (pipe-types (get-in pipes curr))
        dir (if (= x1 (mapv - prev curr)) x2 x1)]
    (mapv + curr dir)))


(defn move-until-loop
  [mat]
  (loop [curr [128 37]
         prev [128 36]
         elements #{prev}]
    (if (= (get-in mat curr) \S)
      elements
      (recur (next-move mat prev curr) curr (conj elements curr)))))


(defn part-1
  [input]
  (->> input
       parse-input
       move-until-loop
       (#(/ (count %) 2))))


(part-1 (utils/read-input "inputs/day10.txt"))


(defn count-walls-to-top
  [pipes loop-pos [y0 x]]
  (loop [y (dec y0)
         n-walls 0
         res \.]
    (if (= y -1)
      n-walls
      (if (loop-pos [y x])
        (let [item (get-in pipes [y x])]
          (recur (dec y)
                 (cond (= item \-) (inc n-walls)
                       (and (= res \L) (= item \7)) (inc n-walls)
                       (and (= res \J) (#{\F \S} item)) (inc n-walls)
                       :else n-walls)
                 (if (= item \|) res item)))
        (recur (dec y) n-walls res)))))


(defn count-enclosed
  [pipes]
  (let [num-rows (count pipes)
        num-cols (count (first pipes))
        loop-pos (move-until-loop pipes)]
    (for [x (range num-cols)
          y (range num-rows)
          :when (not (loop-pos [y x]))]
      (count-walls-to-top pipes loop-pos [y x]))))


(defn part-2
  [input]
  (->> input
       parse-input
       count-enclosed
       (filter odd?)
       count))


(part-2 (utils/read-input "inputs/day10.txt"))

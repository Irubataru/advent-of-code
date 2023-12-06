(ns day06
  (:require
    [clojure.math :refer [ceil floor sqrt]]
    [clojure.string :as str]
    [utils :as utils]))


(defn parse-part-1
  "Parse input for part 1 of the problem"
  [input]
  (let [[times distances] (str/split-lines input)]
    (partition 2
               (interleave (utils/parse-integers times)
                           (utils/parse-integers distances)))))


(defn distance
  "Distance travelled given total time and time spent charging"
  [ttot t]
  (* t (- ttot t)))


(defn score
  "Score of a race with time t and previous record d"
  [[t d]]
  (let [th (* 0.5 t)
        r (- (* th th) d)]
    (if (< r 0.0)
      0
      (let [s1 (long (ceil (- th (sqrt r))))
            s2 (long (floor (+ th (sqrt r))))
            s1 (if (= d (distance t s1)) (inc s1) s1)
            s2 (if (= d (distance t s2)) (dec s2) s2)]
        (inc (- s2 s1))))))


(defn part-1
  [input]
  (->> input
       parse-part-1
       (map score)
       (apply *)))


(part-1 (utils/read-input "inputs/day06.txt"))


(defn parse-single-race
  "Parse the lines of the problem so that we get a single race from the input"
  [line]
  (parse-long (apply str (re-seq #"\d+" line))))


(defn parse-part-2
  "Parse input for part 2 of the problem"
  [input]
  (let [[times distances] (str/split-lines input)]
    [(parse-single-race times) (parse-single-race distances)]))


(defn part-2
  [input]
  (->> input
       parse-part-2
       score))


(part-2 (utils/read-input "inputs/day06.txt"))

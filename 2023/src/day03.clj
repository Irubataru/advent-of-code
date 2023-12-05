(ns day03
  (:require
    [utils :as utils]))


(def not-part? #{\1 \2 \3 \4 \5 \6 \7 \8 \9 \0 \.})
(def part? (complement not-part?))
(def gear? #{\*})


(defn parse-ids
  "Parse the sequence of potential part IDs (numbers)"
  [input]
  (map #(utils/re-pos #"\d+" %) (utils/to-lines input)))


(defn parse-input
  "Parse the input both as the matrix of characters and as the sequence of IDs"
  [input]
  (let [ids (parse-ids input)
        board (utils/to-matrix input)]
    {:board board, :ids ids}))


(defn valid-index?
  "Check whether an index is on the board"
  [row col max-row max-col]
  (and (>= row 0) (< row max-row) (>= col 0) (< col max-col)))


(defn part-number?
  "Check if a number starting at row and col is a part number"
  [board row col num]
  (let [num-len (count num)
        max-col (count board)
        max-row (count (first board))]
    (some true?
          (for [x (range (dec row) (+ row 2))
                y (range (dec col) (+ col num-len 1))
                :when (valid-index? x y max-row max-col)]
            (part? (get-in board [x y]))))))


(defn indexed
  "Return an indexed (enumerated) vector from a sequence"
  [coll]
  (map-indexed vector coll))


(defn get-part-numbers
  "Get the part numbers for a puzzle input"
  [{board :board, ids :ids}]
  (for [[row numbers] (indexed ids)
        [col num] numbers
        :when (part-number? board row col num)]
    (Integer/parseInt num)))


(defn part-1
  [input]
  (->> input
       parse-input
       get-part-numbers
       (apply +)))


(part-1 (utils/read-input "inputs/day03.txt"))


(defn within?
  "Check if a value is within (inclusive) two bounds"
  [val min max]
  (and (<= val max) (>= val min)))


(defn get-adjecent-ids
  "Get all the adjecent IDs of a position on the board"
  [ids gear-row gear-col]
  (remove nil?
          (for [[row numbers] (indexed ids)
                [col num] numbers]
            (let [num-len (count num)]
              (when (and (within? gear-row (dec row) (inc row))
                         (within? gear-col (dec col) (+ col num-len)))
                (Integer/parseInt num))))))


(defn get-gear-ratios
  "Get the gear ratios of all the gears on the board"
  [{board :board, ids :ids}]
  (let [num-rows (count board)
        num-cols (count (first board))]
    (for [x (range num-rows)
          y (range num-cols)
          :when (gear? (get-in board [x y]))]
      (let [gear-numbers (get-adjecent-ids ids x y)]
        (if (= (count gear-numbers) 2) (apply * gear-numbers) 0)))))


(defn part-2
  [input]
  (->> input
       parse-input
       get-gear-ratios
       (apply +)))


(part-2 (utils/read-input "inputs/day03.txt"))

(ns day04
  (:require
    [clojure.math.numeric-tower :as math]
    [utils :as utils]))


(defn parse-numbers
  [line]
  (map #(Integer/parseInt %) (re-seq #"\d+" line)))


(defn parse-game
  "Parse a scratchcard game from a line"
  [line]
  (let [[_ game winning-numbers our-numbers]
        (re-matches #"Card +(\d+): (.*?) \| (.*)" line)]
    {:game (Integer/parseInt game),
     :winning (set (parse-numbers winning-numbers)),
     :ours (parse-numbers our-numbers)}))


(defn winning-entries
  "Find the winning entries of a card"
  [{winning :winning, ours :ours}]
  (filter winning ours))


(defn score
  "Calculate the score of a game with part 1 scoring"
  [card]
  (let [correct (count (winning-entries card))]
    (if (> correct 0) (math/expt 2 (dec correct)) 0)))


(defn part-1
  [input]
  (->> input
       utils/to-lines
       (map parse-game)
       (map score)
       (apply +)))


(part-1 (utils/read-input "inputs/day04.txt"))


(defn pad
  "Pad a sequence with a value intil it is n long"
  [n coll val]
  (take n (concat coll (repeat val))))


(defn add-colls
  "Add two sequences, but keep the longest and pad the sortest with 0"
  [coll-1 coll-2]
  (let [n (max (count coll-1) (count coll-2))]
    (map + (pad n coll-1 0) (pad n coll-2 0))))


(defn score-2
  "Calculate the score of the games with the part 2 scoring"
  [games]
  (loop [total-cards 0
         cards (repeat (count games) 1)
         correct (map #(count (winning-entries %)) games)]
    (if (empty? cards)
      total-cards
      (recur (+ total-cards (first cards))
             (add-colls (rest cards) (repeat (first correct) (first cards)))
             (rest correct)))))


(defn part-2
  [input]
  (->> input
       utils/to-lines
       (map parse-game)
       (score-2)))


(part-2 (utils/read-input "inputs/day04.txt"))

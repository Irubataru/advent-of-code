(ns day07
  (:require
    [clojure.math :refer [ceil floor sqrt]]
    [clojure.string :as str]
    [utils :as utils]))


(def card-score {\A 13, \K 12, \Q 11, \J 10, \T 9, \9 8, \8 7, \7 6, \6 5, \5 4, \4 3, \3 2, \2 1})


(defn parse-line
  [line]
  (let [[hand bid] (str/split line #" ")] {:hand hand, :bid (parse-long bid)}))


(defn parse-input
  [input]
  (->> input
       utils/to-lines
       (map parse-line)))


(defn freqs
  "Frequenes of each card, sorted"
  [coll]
  (sort > (vals (frequencies coll))))


(defn cmp-hands
  "Compare two hands, first by hand type, then by card value"
  [hand-1 hand-2]
  (let [hand-cmp (utils/lex-cmp (freqs hand-1) (freqs hand-2))]
    (if (zero? hand-cmp)
      (utils/lex-cmp (map card-score hand-1) (map card-score hand-2))
      hand-cmp)))


(defn sort-hands
  "Sort a collection of hands"
  ([pred hands] (sort-by #(% :hand) pred hands))
  ([hands] (sort-hands cmp-hands hands)))


(defn tally-bids
  "Calculate the total bid where each bid is multipled by its order in the sequence"
  [hands]
  (->> hands
       utils/indexed
       (map (fn [[index {bid :bid}]] (* (inc index) bid)))
       (apply +)))


(defn part-1
  [input]
  (->> input
       parse-input
       sort-hands
       tally-bids))


(part-1 (utils/read-input "inputs/day07.txt"))


(defn freqs-joker
  "Count and sort frequencies assuming \J is a joker"
  [hand]
  (let [frqs (frequencies hand)
        jokers (get frqs \J 0)
        remainder (sort > (vals (dissoc frqs \J)))]
    (conj (rest remainder) (+ (or (first remainder) 0) jokers))))


(def card-score-joker
  "Individual card scores where joker is worth 0"
  (assoc card-score \J 0))


(defn cmp-hands-joker
  "Compare hands assuming \J is a joker"
  [hand-1 hand-2]
  (let [hand-cmp (utils/lex-cmp (freqs-joker hand-1) (freqs-joker hand-2))]
    (if (zero? hand-cmp)
      (utils/lex-cmp (map card-score-joker hand-1)
                     (map card-score-joker hand-2))
      hand-cmp)))


(defn part-2
  [input]
  (->> input
       parse-input
       (sort-hands cmp-hands-joker)
       tally-bids))


(part-2 (utils/read-input "inputs/day07.txt"))

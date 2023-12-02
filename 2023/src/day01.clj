(ns day01
  (:require
    [utils :as utils]))


(defn parse-digits
  [line]
  (map #(Integer/parseInt %) (re-seq #"\d" line)))


(defn get-value
  [integers]
  (+ (* (first integers) 10) (last integers)))


(defn part-1
  [input]
  (->> input
       utils/to-lines
       (map parse-digits)
       (map get-value)
       (apply +)))


(def number-map
  {"one" 1,
   "two" 2,
   "three" 3,
   "four" 4,
   "five" 5,
   "six" 6,
   "seven" 7,
   "eight" 8,
   "nine" 9})


(def text-regex
  (re-pattern
    (str "(?=([0-9]" (reduce str (map #(str "|" %) (keys number-map))) "))")))


(defn text-to-digit
  [text]
  (if (contains? number-map text) (number-map text) (Integer/parseInt text)))


(defn parse-digits-with-text
  [line]
  (map (comp text-to-digit last) (re-seq text-regex line)))


(defn part-2
  [input]
  (->> input
       utils/to-lines
       (map parse-digits-with-text)
       (map get-value)
       (apply +)))


(let [filename "inputs/day01.txt"] (part-1 (utils/read-input filename)))

(let [filename "inputs/day01.txt"] (part-2 (utils/read-input filename)))

(ns day01
  (:require
    [clojure.string :as str]))


(defn read-input
  [file]
  (slurp file))


(defn digits
  [line]
  (->> (filter #(Character/isDigit %) line)
       (map #(Character/digit % 10))))


(defn return-first
  [values index-fn return-fn]
  (let [value (apply min-key index-fn values)]
    (return-fn value)))


(defn return-last
  [values index-fn return-fn]
  (let [value (apply max-key index-fn values)]
    (return-fn value)))


(def number-map {"one" 1, "two" 2, "three" 3, "four" 4, "five" 5, "six" 6, "seven" 7, "eight" 8, "nine" 9})


(defn transform-first-digit
  [text]
  (return-first (vec (keys number-map))
                #(let [index (.indexOf text %)] (if (= index -1) Long/MAX_VALUE index))
                #(str/replace text % (str (get number-map %)))))


(defn transform-last-digit
  [text]
  (return-last (vec (keys number-map))
               #(let [index (.lastIndexOf text %)] (if (= index -1) Long/MIN_VALUE index))
               #(str/replace text % (str (get number-map %)))))


(defn part-1
  [file]
  (reduce + (let [lines (str/split-lines (read-input file))]
              (map (fn [line]
                     (+ (* (first (digits line)) 10)
                        (last (digits line)))) lines))))


(defn part-2
  [file]
  (reduce + (let [lines (str/split-lines (read-input file))]
              (map (fn [line]
                     (+ (* (first (digits (transform-first-digit line))) 10)
                        (last (digits (transform-last-digit line))))) lines))))


(let [filename "inputs/day01.txt"]
  (part-1 filename))


(let [filename "inputs/day01.txt"]
  (part-2 filename))

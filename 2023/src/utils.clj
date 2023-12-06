(ns utils
  (:require
    [clojure.string :as str]))


(defn read-input
  "Read in the content of the given day-file and return as a blob"
  [day]
  (slurp day))


(defn to-lines
  "Turn a blob or block into a seq of lines"
  [input]
  (str/split-lines input))


(defn to-blocks
  "Turn a blob (probably from `slurp`) into a seq of blocks"
  [input]
  (str/split input #"\n\n"))


(defn to-matrix
  "Turn a blob (or block) into a vector of vectors"
  [input]
  (->> input
       to-lines
       (mapv vec)))


(defn parse-integers
  "Parse integers from a string containing them"
  [line]
  (map parse-long (re-seq #"\d+" line)))


;; Taken from https://stackoverflow.com/a/3266877/6421
;;
;; Get matches for a given regexp *and* their position within the string.
(defn re-pos
  "Return a list of pairs of (index, string) for all matches of `re` in `s`"
  [re s]
  (loop [m (re-matcher re s), res ()]
    (if (.find m)
      (recur m (cons (list (.start m) (.group m)) res))
      (reverse res))))

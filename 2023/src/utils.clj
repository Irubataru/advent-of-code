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


(defn indexed
  "Return an indexed (enumerated) vector from a sequence"
  [coll]
  (map-indexed vector coll))

;; Taken from https://stackoverflow.com/a/3266877/6421
;;
;; Get matches for a given regexp *and* their position within the string.
(defn re-pos
  "Return a list of pairs of (index, string) for all matches of `re` in `s`"
  [re s]
  (loop [m (re-matcher re s)
         res ()]
    (if (.find m)
      (recur m (cons (list (.start m) (.group m)) res))
      (reverse res))))


(defn lex-cmp
  "Lexicographically compare two collections"
  [coll-1 coll-2]
  (loop [s1 coll-1
         s2 coll-2]
    (cond (and (empty? s1) (empty? s2)) 0           ; Both vectors are exhausted and are equal
          (empty? s1) -1                          ; Vector 1 is shorter, so it's "smaller"
          (empty? s2) 1                           ; Vector 2 is shorter, so it's "smaller"
          :else (let [cmp (compare (first s1) (first s2))]
                  (if (zero? cmp)
                    (recur (rest s1) (rest s2)) ; Elements are equal, compare the next position
                    cmp)))))

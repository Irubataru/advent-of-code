(ns day05
  (:require
    [clojure.string :as str]
    [utils :as utils]))


(defn parse-range
  [line]
  (let [[dest-start source-start range] (map parse-long (str/split line #" "))]
    {:dest-start dest-start, :source-start source-start, :range range}))


(defn parse-maps
  [block]
  (let [[name & maps] (str/split-lines block)]
    (let [[_ from to] (re-matches #"(\w+)-to-(\w+) map:" name)]
      {(keyword from) {:to (keyword to), :ranges (map parse-range maps)}})))


(defn parse-seeds
  [line]
  (map parse-long (re-seq #"\d+" line)))


(defn parse-instructions
  [input]
  (let [blocks (str/split input #"\n\n")]
    {:seed (parse-seeds (first blocks)),
     :maps (into {} (map parse-maps (rest blocks)))}))


(defn included?
  [elem {start :source-start, len :range}]
  (and (>= elem start) (<= elem (+ start len))))


(defn next-stage-single
  [item {ranges :ranges}]
  (let [range (first (filter #(included? item %) ranges))]
    (if (nil? range)
      item
      (+ (range :dest-start) (- item (range :source-start))))))


(defn next-stage
  [maps stage items]
  (map #(next-stage-single % (maps stage)) items))


(defn loop-stages
  [{maps :maps, seeds :seed}]
  (loop [vals seeds
         type :seed]
    (if (= type :location)
      vals
      (recur (next-stage maps type vals) ((maps type) :to)))))


(defn part-1
  [input]
  (->> input
       parse-instructions
       loop-stages
       (apply min)))


(part-1 (utils/read-input "inputs/day05.txt"))


(defn not-overlap?
  [from-1 to-1 from-2 to-2]
  (or (< to-1 from-2) (< to-2 from-1)))


(def overlap? (complement not-overlap?))


(defn overlap
  [from-1 to-1 {from :source-start, range :range}]
  (let [from-2 from
        to-2 (+ from range)]
    (if (overlap? from-1 to-1 from-2 to-2)
      [(max from-1 from-2) (min to-1 to-2)])))


(defn overlap-mapped
  [from-1 to-1 {from-2 :source-start, dfrom :dest-start, range :range}]
  (let [to-2 (+ from-2 range)]
    (if (overlap? from-1 to-1 from-2 to-2)
      (let [from (max from-1 from-2)
            to (min to-1 to-2)
            mfrom (+ dfrom (- from from-2))
            mto (+ mfrom (- to from))]
        [mfrom mto]))))


(defn overlap-remainder
  [from-1 to-1 {from :source-start, range :range}]
  (let [from-2 from
        to-2 (+ from range)]
    (if (overlap? from-1 to-1 from-2 to-2)
      (if (<= from-2 from-1)
        (if (>= to-2 to-1) nil [(inc to-2) to-1])
        [from-1 (dec from-2)])
      [from-1 to-1])))


(defn parse-instructions-2
  [input]
  (let [blocks (str/split input #"\n\n")]
    {:seed (map (fn [[x y]] [x (+ x y)])
                (partition 2 (parse-seeds (first blocks)))),
     :maps (into {} (map parse-maps (rest blocks)))}))


(defn next-stage-2
  [maps stage input-ranges]
  (let [stage-map (maps stage)]
    (loop [remaining input-ranges
           mapped []]
      (if (empty? remaining)
        mapped
        (let [[from to] (first remaining)
              overlaps (for [rng (stage-map :ranges)
                             :when (overlap? from
                                             to
                                             (rng :source-start)
                                             (+ (rng :source-start)
                                                (rng :range)))]
                         [(overlap-mapped from to rng)
                          (overlap-remainder from to rng)])]
          (if (empty? overlaps)
            (recur (rest remaining) (conj mapped [from to]))
            (recur (into (rest remaining) (remove nil? (map last overlaps)))
                   (into mapped (remove nil? (map first overlaps))))))))))


(defn loop-stages-2
  [{maps :maps, seed-ranges :seed}]
  (loop [vals seed-ranges
         type :seed]
    (if (= type :location)
      vals
      (recur (next-stage-2 maps type vals) ((maps type) :to)))))


(defn part-2
  [input]
  (->> input
       parse-instructions-2
       loop-stages-2
       (map first)
       (apply min)))


(part-2 (utils/read-input "inputs/day05.txt"))

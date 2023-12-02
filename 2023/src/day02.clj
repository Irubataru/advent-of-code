(ns day02
  (:require
    [clojure.string :as str]
    [utils :as utils]))


(defn parse-colour-count
  [_ count colour]
  {(keyword colour) (Integer/parseInt count)})


(defn parse-round
  [text]
  (let [round (re-seq #"(\d+) (red|green|blue)" text)]
    (into {} (map #(apply parse-colour-count %) round))))


(defn parse-line
  [line]
  (let [[_ game content] (re-matches #"Game (\d+): (.*)" line)
        rounds (str/split content #"; ")
        game (Integer/parseInt game)]
    {:game game, :rounds (map parse-round rounds)}))


(defn- not-possible
  [r g b round]
  (or (< r (get round :red 0))
      (< g (get round :green 0))
      (< b (get round :blue 0))))


(defn- is-possible?
  [r g b {rounds :rounds}]
  (not (some #(not-possible r g b %) rounds)))


(defn possible-games
  [r g b games]
  (map :game (filter #(is-possible? r g b %) games)))


(defn part-1
  [input]
  (->> input
       utils/to-lines
       (map parse-line)
       (possible-games 12 13 14)
       (apply +)))


(let [input (utils/read-input "inputs/day02.txt")] (part-1 input))


(defn smallest-game
  [{rounds :rounds}]
  [(apply max (map #(get % :red 0) rounds))
   (apply max (map #(get % :green 0) rounds))
   (apply max (map #(get % :blue 0) rounds))])


(defn part-2
  [input]
  (->> input
       utils/to-lines
       (map parse-line)
       (map smallest-game)
       (map #(apply * %))
       (reduce +)))


(let [input (utils/read-input "inputs/day02.txt")] (part-2 input))

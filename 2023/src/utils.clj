(ns utils
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(defn read-input
  "Read in the content of the given day-file and return as a blob"
  [day]
  (slurp day))

(defn to-lines
  "Turn a blob or block into a seq of lines"
  [input]
  (str/split-lines input))

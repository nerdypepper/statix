[
  # no match
  ([1 2] ++ [3 4])

  # unnecessary left
  ([] ++ [1 2 3])

  # unnecessary right
  ([1 2 3] ++ [])

  # collapses to a single array
  ([] ++ [])

  # multiple empties
  ([] ++ [] ++ [])
]
import Data.List (maximumBy)
import Data.Function (on)

collatzLength :: Int -> Int -> Int
collatzLength acc 1 = acc
collatzLength acc n
    | even n = collatzLength (acc+1) (n `div` 2)
    | odd  n = collatzLength (acc+1) (3*n + 1)

main = print $ maximumBy (compare `on` (collatzLength 1)) [1..1000000]

-- 837799

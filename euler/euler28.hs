main :: IO ()
main = print $ sum $ scanl1 (+) $ 1:(concat $ map (replicate 4) [2,4..1000])

-- 669171001

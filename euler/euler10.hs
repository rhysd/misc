main = print $ sum . filter isPrime $ [2..2000000]
  where
    isPrime n = null $ filter (\x->n `mod` x==0) [2..floor $ sqrt $ fromIntegral n]

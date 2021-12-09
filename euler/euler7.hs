main = print $ filter isPrime [2..] !! 10000
  where
    isPrime n = null $ filter (\x->n `mod` x==0) [2..floor $ sqrt $ fromIntegral n]
    -- n が素数かどうかを見るには，[2..sqrt(n)] の数が約数に含まれているかを見れば良い．
        
import Data.Char (digitToInt)

main :: IO ()
main = print $ sum $ map digitToInt $ show $ factorial 1 100
    where
        factorial acc 1 = acc
        factorial acc n = factorial (acc*n) (n-1)

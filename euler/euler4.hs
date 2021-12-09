isPalindromeNum :: Int -> Bool
isPalindromeNum n = s == reverse s
    where s = show n

main = print . maximum $ [x*y| x<-[100..999],y<-[x..999], isPalindromeNum $ x*y]

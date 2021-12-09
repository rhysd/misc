main = print . head $ [ x | x <- [1..], allDivided 20 x ]
    where
        allDivided 1 _ = True
        allDivided n x = x `mod` n == 0 && allDivided (n-1) x

-- もっと真面目に解ける．
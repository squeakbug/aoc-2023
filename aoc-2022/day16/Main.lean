def hello := "world"

part1 :: [(Int, Int)] -> Int
part1 = sum
      . map (abs . uncurry (-))
      . sortAndZip


part2 :: [(Int, Int)] -> Int
part2 xs = sum $ map (\x -> x * similarity first x) second
    where
        (first, second) = unzip xs

main :: IO ()
main = do
    args <- getArgs
    content <- readFile (head args)
    let input = parseFile content
    let part1_sln = part1 input
    let part2_sln = part2 input
    putStrLn $ printf "part1_sln = %d" part1_sln
    putStrLn $ printf "part2_sln = %d" part2_sln

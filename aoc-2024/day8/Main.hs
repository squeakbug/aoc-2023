{-# LANGUAGE OverloadedStrings #-}

module Main where

import System.IO
import Control.Applicative
import Data.Void (Void)
import Data.Maybe
import Data.Functor
import System.Environment (getArgs)
import Text.Printf (printf)

part1 :: String -> Int
part1 input = 0

part2 :: String -> Int
part2 input = 0

main :: IO ()
main = do
    args <- getArgs
    input <- readFile (head args)
    let part1_sln = part1 input
    let part2_sln = part2 input
    putStrLn $ printf "part1_sln = %d" part1_sln
    putStrLn $ printf "part2_sln = %d" part2_sln


import Data.Char (ord, chr)
import Data.List (unfoldr, foldl')

decodeBase94 :: String -> Int
decodeBase94 = foldl' (\acc c -> acc * 94 + (ord c - ord '!')) 0

encodeString :: Int -> String
encodeString 0 = "!"
encodeString n = reverse (unfoldr f n)
    where f 0 = Nothing
          f x = Just (chr (ord '!' + x `mod` 94), x `div` 94)

main :: IO ()
main = putStrLn (("L" ++ (((\v1 -> ((\v1 -> ((\v2 -> (v1 (v2 v2))) (\v2 -> (v1 (v2 v2))))) (\v3 -> (\v2 -> if (v2 == 1) then v1 else (v1 ++ (v3 (v2 - 1))))))) ".") 199)))


{-# LANGUAGE NoMonomorphismRestriction #-}
module ICFP where


main :: IO ()
main = print result

result :: String
result = ("L" ++ (((\v1 -> ((\v1 -> ((\v2 -> (v1 (v2 v2))) (\v2 -> (v1 (v2 v2))))) (\v3 -> (\v2 -> (if (v2 == 1) then v1 else (v1 ++ (v3 (v2 - 1)))))))) ".") 199))

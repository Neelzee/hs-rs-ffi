module Option (stringToInt) where
import Text.Read (readMaybe)

stringToInt :: String -> Maybe Int
stringToInt s = readMaybe s :: Maybe Int
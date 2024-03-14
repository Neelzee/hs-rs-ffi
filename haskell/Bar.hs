module Bar (HaskellType, getHaskellType) where

data HaskellType = HaskellType Int String HaskellType | Nested Int

getHaskellType :: HaskellType
getHaskellType = HaskellType 42 "Hello, From Haskell!" (Nested 42)
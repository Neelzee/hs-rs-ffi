module Foo (foobar) where

foreign export ccall foobar :: Int -> Int -> Int

foobar :: Int -> Int -> Int
foobar = (*)
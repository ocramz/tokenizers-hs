{-# LANGUAGE ForeignFunctionInterface #-}
module Tokenizers where

import Foreign
import Foreign.C.Types
import Foreign.ForeignPtr

foreign import ccall "encode"
  c_encode :: CString -> CString

encode = c_encode

-- fact :: Int -> Int
-- fact = fromIntegral . c_fact . fromIntegral


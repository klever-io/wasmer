use wasmer_runtime_core:: {
    wasmparser::Operator
};

pub fn get_opcode_index(op: &Operator) -> usize {
    match *op {
        Operator::Unreachable { .. } => { 0 }
        Operator::Nop { .. } => { 1 }
        Operator::Block { .. } => { 2 }
        Operator::Loop { .. } => { 3 }
        Operator::If { .. } => { 4 }
        Operator::Else { .. } => { 5 }
        Operator::End { .. } => { 6 }
        Operator::Br { .. } => { 7 }
        Operator::BrIf { .. } => { 8 }
        Operator::BrTable { .. } => { 9 }
        Operator::Return { .. } => { 10 }
        Operator::Call { .. } => { 11 }
        Operator::CallIndirect { .. } => { 12 }
        Operator::Drop { .. } => { 13 }
        Operator::Select { .. } => { 14 }
        Operator::LocalGet { .. } => { 15 }
        Operator::LocalSet { .. } => { 16 }
        Operator::LocalTee { .. } => { 17 }
        Operator::GlobalGet { .. } => { 18 }
        Operator::GlobalSet { .. } => { 19 }
        Operator::I32Load { .. } => { 20 }
        Operator::I64Load { .. } => { 21 }
        Operator::F32Load { .. } => { 22 }
        Operator::F64Load { .. } => { 23 }
        Operator::I32Load8S { .. } => { 24 }
        Operator::I32Load8U { .. } => { 25 }
        Operator::I32Load16S { .. } => { 26 }
        Operator::I32Load16U { .. } => { 27 }
        Operator::I64Load8S { .. } => { 28 }
        Operator::I64Load8U { .. } => { 29 }
        Operator::I64Load16S { .. } => { 30 }
        Operator::I64Load16U { .. } => { 31 }
        Operator::I64Load32S { .. } => { 32 }
        Operator::I64Load32U { .. } => { 33 }
        Operator::I32Store { .. } => { 34 }
        Operator::I64Store { .. } => { 35 }
        Operator::F32Store { .. } => { 36 }
        Operator::F64Store { .. } => { 37 }
        Operator::I32Store8 { .. } => { 38 }
        Operator::I32Store16 { .. } => { 39 }
        Operator::I64Store8 { .. } => { 40 }
        Operator::I64Store16 { .. } => { 41 }
        Operator::I64Store32 { .. } => { 42 }
        Operator::MemorySize { .. } => { 43 }
        Operator::MemoryGrow { .. } => { 44 }
        Operator::I32Const { .. } => { 45 }
        Operator::I64Const { .. } => { 46 }
        Operator::F32Const { .. } => { 47 }
        Operator::F64Const { .. } => { 48 }
        Operator::RefNull { .. } => { 49 }
        Operator::RefIsNull { .. } => { 50 }
        Operator::I32Eqz { .. } => { 51 }
        Operator::I32Eq { .. } => { 52 }
        Operator::I32Ne { .. } => { 53 }
        Operator::I32LtS { .. } => { 54 }
        Operator::I32LtU { .. } => { 55 }
        Operator::I32GtS { .. } => { 56 }
        Operator::I32GtU { .. } => { 57 }
        Operator::I32LeS { .. } => { 58 }
        Operator::I32LeU { .. } => { 59 }
        Operator::I32GeS { .. } => { 60 }
        Operator::I32GeU { .. } => { 61 }
        Operator::I64Eqz { .. } => { 62 }
        Operator::I64Eq { .. } => { 63 }
        Operator::I64Ne { .. } => { 64 }
        Operator::I64LtS { .. } => { 65 }
        Operator::I64LtU { .. } => { 66 }
        Operator::I64GtS { .. } => { 67 }
        Operator::I64GtU { .. } => { 68 }
        Operator::I64LeS { .. } => { 69 }
        Operator::I64LeU { .. } => { 70 }
        Operator::I64GeS { .. } => { 71 }
        Operator::I64GeU { .. } => { 72 }
        Operator::F32Eq { .. } => { 73 }
        Operator::F32Ne { .. } => { 74 }
        Operator::F32Lt { .. } => { 75 }
        Operator::F32Gt { .. } => { 76 }
        Operator::F32Le { .. } => { 77 }
        Operator::F32Ge { .. } => { 78 }
        Operator::F64Eq { .. } => { 79 }
        Operator::F64Ne { .. } => { 80 }
        Operator::F64Lt { .. } => { 81 }
        Operator::F64Gt { .. } => { 82 }
        Operator::F64Le { .. } => { 83 }
        Operator::F64Ge { .. } => { 84 }
        Operator::I32Clz { .. } => { 85 }
        Operator::I32Ctz { .. } => { 86 }
        Operator::I32Popcnt { .. } => { 87 }
        Operator::I32Add { .. } => { 88 }
        Operator::I32Sub { .. } => { 89 }
        Operator::I32Mul { .. } => { 90 }
        Operator::I32DivS { .. } => { 91 }
        Operator::I32DivU { .. } => { 92 }
        Operator::I32RemS { .. } => { 93 }
        Operator::I32RemU { .. } => { 94 }
        Operator::I32And { .. } => { 95 }
        Operator::I32Or { .. } => { 96 }
        Operator::I32Xor { .. } => { 97 }
        Operator::I32Shl { .. } => { 98 }
        Operator::I32ShrS { .. } => { 99 }
        Operator::I32ShrU { .. } => { 100 }
        Operator::I32Rotl { .. } => { 101 }
        Operator::I32Rotr { .. } => { 102 }
        Operator::I64Clz { .. } => { 103 }
        Operator::I64Ctz { .. } => { 104 }
        Operator::I64Popcnt { .. } => { 105 }
        Operator::I64Add { .. } => { 106 }
        Operator::I64Sub { .. } => { 107 }
        Operator::I64Mul { .. } => { 108 }
        Operator::I64DivS { .. } => { 109 }
        Operator::I64DivU { .. } => { 110 }
        Operator::I64RemS { .. } => { 111 }
        Operator::I64RemU { .. } => { 112 }
        Operator::I64And { .. } => { 113 }
        Operator::I64Or { .. } => { 114 }
        Operator::I64Xor { .. } => { 115 }
        Operator::I64Shl { .. } => { 116 }
        Operator::I64ShrS { .. } => { 117 }
        Operator::I64ShrU { .. } => { 118 }
        Operator::I64Rotl { .. } => { 119 }
        Operator::I64Rotr { .. } => { 120 }
        Operator::F32Abs { .. } => { 121 }
        Operator::F32Neg { .. } => { 122 }
        Operator::F32Ceil { .. } => { 123 }
        Operator::F32Floor { .. } => { 124 }
        Operator::F32Trunc { .. } => { 125 }
        Operator::F32Nearest { .. } => { 126 }
        Operator::F32Sqrt { .. } => { 127 }
        Operator::F32Add { .. } => { 128 }
        Operator::F32Sub { .. } => { 129 }
        Operator::F32Mul { .. } => { 130 }
        Operator::F32Div { .. } => { 131 }
        Operator::F32Min { .. } => { 132 }
        Operator::F32Max { .. } => { 133 }
        Operator::F32Copysign { .. } => { 134 }
        Operator::F64Abs { .. } => { 135 }
        Operator::F64Neg { .. } => { 136 }
        Operator::F64Ceil { .. } => { 137 }
        Operator::F64Floor { .. } => { 138 }
        Operator::F64Trunc { .. } => { 139 }
        Operator::F64Nearest { .. } => { 140 }
        Operator::F64Sqrt { .. } => { 141 }
        Operator::F64Add { .. } => { 142 }
        Operator::F64Sub { .. } => { 143 }
        Operator::F64Mul { .. } => { 144 }
        Operator::F64Div { .. } => { 145 }
        Operator::F64Min { .. } => { 146 }
        Operator::F64Max { .. } => { 147 }
        Operator::F64Copysign { .. } => { 148 }
        Operator::I32WrapI64 { .. } => { 149 }
        Operator::I32TruncF32S { .. } => { 150 }
        Operator::I32TruncF32U { .. } => { 151 }
        Operator::I32TruncF64S { .. } => { 152 }
        Operator::I32TruncF64U { .. } => { 153 }
        Operator::I64ExtendI32S { .. } => { 154 }
        Operator::I64ExtendI32U { .. } => { 155 }
        Operator::I64TruncF32S { .. } => { 156 }
        Operator::I64TruncF32U { .. } => { 157 }
        Operator::I64TruncF64S { .. } => { 158 }
        Operator::I64TruncF64U { .. } => { 159 }
        Operator::F32ConvertI32S { .. } => { 160 }
        Operator::F32ConvertI32U { .. } => { 161 }
        Operator::F32ConvertI64S { .. } => { 162 }
        Operator::F32ConvertI64U { .. } => { 163 }
        Operator::F32DemoteF64 { .. } => { 164 }
        Operator::F64ConvertI32S { .. } => { 165 }
        Operator::F64ConvertI32U { .. } => { 166 }
        Operator::F64ConvertI64S { .. } => { 167 }
        Operator::F64ConvertI64U { .. } => { 168 }
        Operator::F64PromoteF32 { .. } => { 169 }
        Operator::I32ReinterpretF32 { .. } => { 170 }
        Operator::I64ReinterpretF64 { .. } => { 171 }
        Operator::F32ReinterpretI32 { .. } => { 172 }
        Operator::F64ReinterpretI64 { .. } => { 173 }
        Operator::I32Extend8S { .. } => { 174 }
        Operator::I32Extend16S { .. } => { 175 }
        Operator::I64Extend8S { .. } => { 176 }
        Operator::I64Extend16S { .. } => { 177 }
        Operator::I64Extend32S { .. } => { 178 }
        Operator::I32TruncSatF32S { .. } => { 179 }
        Operator::I32TruncSatF32U { .. } => { 180 }
        Operator::I32TruncSatF64S { .. } => { 181 }
        Operator::I32TruncSatF64U { .. } => { 182 }
        Operator::I64TruncSatF32S { .. } => { 183 }
        Operator::I64TruncSatF32U { .. } => { 184 }
        Operator::I64TruncSatF64S { .. } => { 185 }
        Operator::I64TruncSatF64U { .. } => { 186 }
        Operator::MemoryInit { .. } => { 187 }
        Operator::DataDrop { .. } => { 188 }
        Operator::MemoryCopy { .. } => { 189 }
        Operator::MemoryFill { .. } => { 190 }
        Operator::TableInit { .. } => { 191 }
        Operator::ElemDrop { .. } => { 192 }
        Operator::TableCopy { .. } => { 193 }
        Operator::TableGet { .. } => { 194 }
        Operator::TableSet { .. } => { 195 }
        Operator::TableGrow { .. } => { 196 }
        Operator::TableSize { .. } => { 197 }
        Operator::AtomicNotify { .. } => { 198 }
        Operator::I32AtomicWait { .. } => { 199 }
        Operator::I64AtomicWait { .. } => { 200 }
        Operator::AtomicFence { .. } => { 201 }
        Operator::I32AtomicLoad { .. } => { 202 }
        Operator::I64AtomicLoad { .. } => { 203 }
        Operator::I32AtomicLoad8U { .. } => { 204 }
        Operator::I32AtomicLoad16U { .. } => { 205 }
        Operator::I64AtomicLoad8U { .. } => { 206 }
        Operator::I64AtomicLoad16U { .. } => { 207 }
        Operator::I64AtomicLoad32U { .. } => { 208 }
        Operator::I32AtomicStore { .. } => { 209 }
        Operator::I64AtomicStore { .. } => { 210 }
        Operator::I32AtomicStore8 { .. } => { 211 }
        Operator::I32AtomicStore16 { .. } => { 212 }
        Operator::I64AtomicStore8 { .. } => { 213 }
        Operator::I64AtomicStore16 { .. } => { 214 }
        Operator::I64AtomicStore32 { .. } => { 215 }
        Operator::I32AtomicRmwAdd { .. } => { 216 }
        Operator::I64AtomicRmwAdd { .. } => { 217 }
        Operator::I32AtomicRmw8AddU { .. } => { 218 }
        Operator::I32AtomicRmw16AddU { .. } => { 219 }
        Operator::I64AtomicRmw8AddU { .. } => { 220 }
        Operator::I64AtomicRmw16AddU { .. } => { 221 }
        Operator::I64AtomicRmw32AddU { .. } => { 222 }
        Operator::I32AtomicRmwSub { .. } => { 223 }
        Operator::I64AtomicRmwSub { .. } => { 224 }
        Operator::I32AtomicRmw8SubU { .. } => { 225 }
        Operator::I32AtomicRmw16SubU { .. } => { 226 }
        Operator::I64AtomicRmw8SubU { .. } => { 227 }
        Operator::I64AtomicRmw16SubU { .. } => { 228 }
        Operator::I64AtomicRmw32SubU { .. } => { 229 }
        Operator::I32AtomicRmwAnd { .. } => { 230 }
        Operator::I64AtomicRmwAnd { .. } => { 231 }
        Operator::I32AtomicRmw8AndU { .. } => { 232 }
        Operator::I32AtomicRmw16AndU { .. } => { 233 }
        Operator::I64AtomicRmw8AndU { .. } => { 234 }
        Operator::I64AtomicRmw16AndU { .. } => { 235 }
        Operator::I64AtomicRmw32AndU { .. } => { 236 }
        Operator::I32AtomicRmwOr { .. } => { 237 }
        Operator::I64AtomicRmwOr { .. } => { 238 }
        Operator::I32AtomicRmw8OrU { .. } => { 239 }
        Operator::I32AtomicRmw16OrU { .. } => { 240 }
        Operator::I64AtomicRmw8OrU { .. } => { 241 }
        Operator::I64AtomicRmw16OrU { .. } => { 242 }
        Operator::I64AtomicRmw32OrU { .. } => { 243 }
        Operator::I32AtomicRmwXor { .. } => { 244 }
        Operator::I64AtomicRmwXor { .. } => { 245 }
        Operator::I32AtomicRmw8XorU { .. } => { 246 }
        Operator::I32AtomicRmw16XorU { .. } => { 247 }
        Operator::I64AtomicRmw8XorU { .. } => { 248 }
        Operator::I64AtomicRmw16XorU { .. } => { 249 }
        Operator::I64AtomicRmw32XorU { .. } => { 250 }
        Operator::I32AtomicRmwXchg { .. } => { 251 }
        Operator::I64AtomicRmwXchg { .. } => { 252 }
        Operator::I32AtomicRmw8XchgU { .. } => { 253 }
        Operator::I32AtomicRmw16XchgU { .. } => { 254 }
        Operator::I64AtomicRmw8XchgU { .. } => { 255 }
        Operator::I64AtomicRmw16XchgU { .. } => { 256 }
        Operator::I64AtomicRmw32XchgU { .. } => { 257 }
        Operator::I32AtomicRmwCmpxchg { .. } => { 258 }
        Operator::I64AtomicRmwCmpxchg { .. } => { 259 }
        Operator::I32AtomicRmw8CmpxchgU { .. } => { 260 }
        Operator::I32AtomicRmw16CmpxchgU { .. } => { 261 }
        Operator::I64AtomicRmw8CmpxchgU { .. } => { 262 }
        Operator::I64AtomicRmw16CmpxchgU { .. } => { 263 }
        Operator::I64AtomicRmw32CmpxchgU { .. } => { 264 }
        Operator::V128Load { .. } => { 265 }
        Operator::V128Store { .. } => { 266 }
        Operator::V128Const { .. } => { 267 }
        Operator::I8x16Splat { .. } => { 268 }
        Operator::I8x16ExtractLaneS { .. } => { 269 }
        Operator::I8x16ExtractLaneU { .. } => { 270 }
        Operator::I8x16ReplaceLane { .. } => { 271 }
        Operator::I16x8Splat { .. } => { 272 }
        Operator::I16x8ExtractLaneS { .. } => { 273 }
        Operator::I16x8ExtractLaneU { .. } => { 274 }
        Operator::I16x8ReplaceLane { .. } => { 275 }
        Operator::I32x4Splat { .. } => { 276 }
        Operator::I32x4ExtractLane { .. } => { 277 }
        Operator::I32x4ReplaceLane { .. } => { 278 }
        Operator::I64x2Splat { .. } => { 279 }
        Operator::I64x2ExtractLane { .. } => { 280 }
        Operator::I64x2ReplaceLane { .. } => { 281 }
        Operator::F32x4Splat { .. } => { 282 }
        Operator::F32x4ExtractLane { .. } => { 283 }
        Operator::F32x4ReplaceLane { .. } => { 284 }
        Operator::F64x2Splat { .. } => { 285 }
        Operator::F64x2ExtractLane { .. } => { 286 }
        Operator::F64x2ReplaceLane { .. } => { 287 }
        Operator::I8x16Eq { .. } => { 288 }
        Operator::I8x16Ne { .. } => { 289 }
        Operator::I8x16LtS { .. } => { 290 }
        Operator::I8x16LtU { .. } => { 291 }
        Operator::I8x16GtS { .. } => { 292 }
        Operator::I8x16GtU { .. } => { 293 }
        Operator::I8x16LeS { .. } => { 294 }
        Operator::I8x16LeU { .. } => { 295 }
        Operator::I8x16GeS { .. } => { 296 }
        Operator::I8x16GeU { .. } => { 297 }
        Operator::I16x8Eq { .. } => { 298 }
        Operator::I16x8Ne { .. } => { 299 }
        Operator::I16x8LtS { .. } => { 300 }
        Operator::I16x8LtU { .. } => { 301 }
        Operator::I16x8GtS { .. } => { 302 }
        Operator::I16x8GtU { .. } => { 303 }
        Operator::I16x8LeS { .. } => { 304 }
        Operator::I16x8LeU { .. } => { 305 }
        Operator::I16x8GeS { .. } => { 306 }
        Operator::I16x8GeU { .. } => { 307 }
        Operator::I32x4Eq { .. } => { 308 }
        Operator::I32x4Ne { .. } => { 309 }
        Operator::I32x4LtS { .. } => { 310 }
        Operator::I32x4LtU { .. } => { 311 }
        Operator::I32x4GtS { .. } => { 312 }
        Operator::I32x4GtU { .. } => { 313 }
        Operator::I32x4LeS { .. } => { 314 }
        Operator::I32x4LeU { .. } => { 315 }
        Operator::I32x4GeS { .. } => { 316 }
        Operator::I32x4GeU { .. } => { 317 }
        Operator::F32x4Eq { .. } => { 318 }
        Operator::F32x4Ne { .. } => { 319 }
        Operator::F32x4Lt { .. } => { 320 }
        Operator::F32x4Gt { .. } => { 321 }
        Operator::F32x4Le { .. } => { 322 }
        Operator::F32x4Ge { .. } => { 323 }
        Operator::F64x2Eq { .. } => { 324 }
        Operator::F64x2Ne { .. } => { 325 }
        Operator::F64x2Lt { .. } => { 326 }
        Operator::F64x2Gt { .. } => { 327 }
        Operator::F64x2Le { .. } => { 328 }
        Operator::F64x2Ge { .. } => { 329 }
        Operator::V128Not { .. } => { 330 }
        Operator::V128And { .. } => { 331 }
        Operator::V128Or { .. } => { 332 }
        Operator::V128Xor { .. } => { 333 }
        Operator::V128Bitselect { .. } => { 334 }
        Operator::I8x16Neg { .. } => { 335 }
        Operator::I8x16AnyTrue { .. } => { 336 }
        Operator::I8x16AllTrue { .. } => { 337 }
        Operator::I8x16Shl { .. } => { 338 }
        Operator::I8x16ShrS { .. } => { 339 }
        Operator::I8x16ShrU { .. } => { 340 }
        Operator::I8x16Add { .. } => { 341 }
        Operator::I8x16AddSaturateS { .. } => { 342 }
        Operator::I8x16AddSaturateU { .. } => { 343 }
        Operator::I8x16Sub { .. } => { 344 }
        Operator::I8x16SubSaturateS { .. } => { 345 }
        Operator::I8x16SubSaturateU { .. } => { 346 }
        Operator::I8x16Mul { .. } => { 347 }
        Operator::I16x8Neg { .. } => { 348 }
        Operator::I16x8AnyTrue { .. } => { 349 }
        Operator::I16x8AllTrue { .. } => { 350 }
        Operator::I16x8Shl { .. } => { 351 }
        Operator::I16x8ShrS { .. } => { 352 }
        Operator::I16x8ShrU { .. } => { 353 }
        Operator::I16x8Add { .. } => { 354 }
        Operator::I16x8AddSaturateS { .. } => { 355 }
        Operator::I16x8AddSaturateU { .. } => { 356 }
        Operator::I16x8Sub { .. } => { 357 }
        Operator::I16x8SubSaturateS { .. } => { 358 }
        Operator::I16x8SubSaturateU { .. } => { 359 }
        Operator::I16x8Mul { .. } => { 360 }
        Operator::I32x4Neg { .. } => { 361 }
        Operator::I32x4AnyTrue { .. } => { 362 }
        Operator::I32x4AllTrue { .. } => { 363 }
        Operator::I32x4Shl { .. } => { 364 }
        Operator::I32x4ShrS { .. } => { 365 }
        Operator::I32x4ShrU { .. } => { 366 }
        Operator::I32x4Add { .. } => { 367 }
        Operator::I32x4Sub { .. } => { 368 }
        Operator::I32x4Mul { .. } => { 369 }
        Operator::I64x2Neg { .. } => { 370 }
        Operator::I64x2AnyTrue { .. } => { 371 }
        Operator::I64x2AllTrue { .. } => { 372 }
        Operator::I64x2Shl { .. } => { 373 }
        Operator::I64x2ShrS { .. } => { 374 }
        Operator::I64x2ShrU { .. } => { 375 }
        Operator::I64x2Add { .. } => { 376 }
        Operator::I64x2Sub { .. } => { 377 }
        Operator::F32x4Abs { .. } => { 378 }
        Operator::F32x4Neg { .. } => { 379 }
        Operator::F32x4Sqrt { .. } => { 380 }
        Operator::F32x4Add { .. } => { 381 }
        Operator::F32x4Sub { .. } => { 382 }
        Operator::F32x4Mul { .. } => { 383 }
        Operator::F32x4Div { .. } => { 384 }
        Operator::F32x4Min { .. } => { 385 }
        Operator::F32x4Max { .. } => { 386 }
        Operator::F64x2Abs { .. } => { 387 }
        Operator::F64x2Neg { .. } => { 388 }
        Operator::F64x2Sqrt { .. } => { 389 }
        Operator::F64x2Add { .. } => { 390 }
        Operator::F64x2Sub { .. } => { 391 }
        Operator::F64x2Mul { .. } => { 392 }
        Operator::F64x2Div { .. } => { 393 }
        Operator::F64x2Min { .. } => { 394 }
        Operator::F64x2Max { .. } => { 395 }
        Operator::I32x4TruncSatF32x4S { .. } => { 396 }
        Operator::I32x4TruncSatF32x4U { .. } => { 397 }
        Operator::I64x2TruncSatF64x2S { .. } => { 398 }
        Operator::I64x2TruncSatF64x2U { .. } => { 399 }
        Operator::F32x4ConvertI32x4S { .. } => { 400 }
        Operator::F32x4ConvertI32x4U { .. } => { 401 }
        Operator::F64x2ConvertI64x2S { .. } => { 402 }
        Operator::F64x2ConvertI64x2U { .. } => { 403 }
        Operator::V8x16Swizzle { .. } => { 404 }
        Operator::V8x16Shuffle { .. } => { 405 }
        Operator::V8x16LoadSplat { .. } => { 406 }
        Operator::V16x8LoadSplat { .. } => { 407 }
        Operator::V32x4LoadSplat { .. } => { 408 }
        Operator::V64x2LoadSplat { .. } => { 409 }
    }
}

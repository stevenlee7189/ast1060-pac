#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi000: Spi000,
    spi004: Spi004,
    spi008: Spi008,
    spi00c: Spi00c,
    spi010: Spi010,
    spi014: Spi014,
    _reserved6: [u8; 0x18],
    spi030: Spi030,
    spi034: Spi034,
    _reserved8: [u8; 0x1c],
    spi054: Spi054,
    _reserved9: [u8; 0x28],
    spi080: Spi080,
    spi084: Spi084,
    spi088: Spi088,
    spi08c: Spi08c,
    spi090: Spi090,
    spi094: Spi094,
    spi098: Spi098,
    _reserved16: [u8; 0x04],
    spi0a0: Spi0a0,
    spi0a4: Spi0a4,
    spi0a8: Spi0a8,
    spi0ac: Spi0ac,
    spi0b0: Spi0b0,
    spi0b4: Spi0b4,
    spi0b8: Spi0b8,
    spi0bc: Spi0bc,
    spi0c0: Spi0c0,
    spi0c4: Spi0c4,
    _reserved26: [u8; 0x38],
    spi100: Spi100,
    spi104: Spi104,
    spi108: Spi108,
    spi10c: Spi10c,
    spi110: Spi110,
    spi114: Spi114,
    spi118: Spi118,
    spi11c: Spi11c,
    spi120: Spi120,
    spi124: Spi124,
    spi128: Spi128,
    spi12c: Spi12c,
    spi130: Spi130,
    spi134: Spi134,
    spi138: Spi138,
    spi13c: Spi13c,
    spi140: Spi140,
    spi144: Spi144,
    spi148: Spi148,
    _reserved45: [u8; 0x04],
    spi150: Spi150,
    spi154: Spi154,
    spi158: Spi158,
    spi15c: Spi15c,
    spi160: Spi160,
    spi164: Spi164,
    spi168: Spi168,
    spi16c: Spi16c,
    spi170: Spi170,
    spi174: Spi174,
    spi178: Spi178,
    spi17c: Spi17c,
    _reserved57: [u8; 0x80],
    spi200: Spi200,
    spi204: Spi204,
    spi208: Spi208,
    spi20c: Spi20c,
    spi210: Spi210,
    spi214: Spi214,
    spi218: Spi218,
    spi21c: Spi21c,
    spi220: Spi220,
    spi224: Spi224,
    spi228: Spi228,
    spi22c: Spi22c,
    spi230: Spi230,
    spi234: Spi234,
    spi238: Spi238,
    spi23c: Spi23c,
    spi240: Spi240,
    spi244: Spi244,
    spi248: Spi248,
    spi24c: Spi24c,
    spi250: Spi250,
    spi254: Spi254,
    spi258: Spi258,
    spi25c: Spi25c,
    spi260: Spi260,
    spi264: Spi264,
    spi268: Spi268,
    spi26c: Spi26c,
    spi270: Spi270,
    spi274: Spi274,
    spi278: Spi278,
    spi27c: Spi27c,
    spi280: Spi280,
    spi284: Spi284,
    spi288: Spi288,
    spi28c: Spi28c,
    spi290: Spi290,
    spi294: Spi294,
    spi298: Spi298,
    spi29c: Spi29c,
    spi2a0: Spi2a0,
    spi2a4: Spi2a4,
    spi2a8: Spi2a8,
    spi2ac: Spi2ac,
    spi2b0: Spi2b0,
    spi2b4: Spi2b4,
    spi2b8: Spi2b8,
    spi2bc: Spi2bc,
    spi2c0: Spi2c0,
    spi2c4: Spi2c4,
    spi2c8: Spi2c8,
    spi2cc: Spi2cc,
    spi2d0: Spi2d0,
    spi2d4: Spi2d4,
    spi2d8: Spi2d8,
    spi2dc: Spi2dc,
    spi2e0: Spi2e0,
    spi2e4: Spi2e4,
    spi2e8: Spi2e8,
    spi2ec: Spi2ec,
    spi2f0: Spi2f0,
    spi2f4: Spi2f4,
    spi2f8: Spi2f8,
    spi2fc: Spi2fc,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Flash Configuration Register"]
    #[inline(always)]
    pub const fn spi000(&self) -> &Spi000 {
        &self.spi000
    }
    #[doc = "0x04 - CE Control Register"]
    #[inline(always)]
    pub const fn spi004(&self) -> &Spi004 {
        &self.spi004
    }
    #[doc = "0x08 - Interrupt Control and Status Register"]
    #[inline(always)]
    pub const fn spi008(&self) -> &Spi008 {
        &self.spi008
    }
    #[doc = "0x0c - Command Control Register"]
    #[inline(always)]
    pub const fn spi00c(&self) -> &Spi00c {
        &self.spi00c
    }
    #[doc = "0x10 - CE0 Control Register"]
    #[inline(always)]
    pub const fn spi010(&self) -> &Spi010 {
        &self.spi010
    }
    #[doc = "0x14 - CE1 Control Register"]
    #[inline(always)]
    pub const fn spi014(&self) -> &Spi014 {
        &self.spi014
    }
    #[doc = "0x30 - CE0 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi030(&self) -> &Spi030 {
        &self.spi030
    }
    #[doc = "0x34 - CE1 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi034(&self) -> &Spi034 {
        &self.spi034
    }
    #[doc = "0x54 - SPI Dummy Cycle Data Register"]
    #[inline(always)]
    pub const fn spi054(&self) -> &Spi054 {
        &self.spi054
    }
    #[doc = "0x80 - DMA Control/Status Register"]
    #[inline(always)]
    pub const fn spi080(&self) -> &Spi080 {
        &self.spi080
    }
    #[doc = "0x84 - DMA Flash Side Address"]
    #[inline(always)]
    pub const fn spi084(&self) -> &Spi084 {
        &self.spi084
    }
    #[doc = "0x88 - DMA DRAM/SRAM Side Address"]
    #[inline(always)]
    pub const fn spi088(&self) -> &Spi088 {
        &self.spi088
    }
    #[doc = "0x8c - DMA Length Register"]
    #[inline(always)]
    pub const fn spi08c(&self) -> &Spi08c {
        &self.spi08c
    }
    #[doc = "0x90 - CheckSum Calculation Result"]
    #[inline(always)]
    pub const fn spi090(&self) -> &Spi090 {
        &self.spi090
    }
    #[doc = "0x94 - CE0 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi094(&self) -> &Spi094 {
        &self.spi094
    }
    #[doc = "0x98 - CE1 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi098(&self) -> &Spi098 {
        &self.spi098
    }
    #[doc = "0xa0 - Command Filter Control Register"]
    #[inline(always)]
    pub const fn spi0a0(&self) -> &Spi0a0 {
        &self.spi0a0
    }
    #[doc = "0xa4 - Write Address Filter Control Register"]
    #[inline(always)]
    pub const fn spi0a4(&self) -> &Spi0a4 {
        &self.spi0a4
    }
    #[doc = "0xa8 - Register Lock Control Register (SRST\\#)"]
    #[inline(always)]
    pub const fn spi0a8(&self) -> &Spi0a8 {
        &self.spi0a8
    }
    #[doc = "0xac - Register Lock Control Register (Watchdog)"]
    #[inline(always)]
    pub const fn spi0ac(&self) -> &Spi0ac {
        &self.spi0ac
    }
    #[doc = "0xb0 - Write Address Filter Register \\#1"]
    #[inline(always)]
    pub const fn spi0b0(&self) -> &Spi0b0 {
        &self.spi0b0
    }
    #[doc = "0xb4 - Write Address Filter Register \\#2"]
    #[inline(always)]
    pub const fn spi0b4(&self) -> &Spi0b4 {
        &self.spi0b4
    }
    #[doc = "0xb8 - Write Address Filter Register \\#3"]
    #[inline(always)]
    pub const fn spi0b8(&self) -> &Spi0b8 {
        &self.spi0b8
    }
    #[doc = "0xbc - Write Address Filter Register \\#4"]
    #[inline(always)]
    pub const fn spi0bc(&self) -> &Spi0bc {
        &self.spi0bc
    }
    #[doc = "0xc0 - Write Address Filter Register \\#5"]
    #[inline(always)]
    pub const fn spi0c0(&self) -> &Spi0c0 {
        &self.spi0c0
    }
    #[doc = "0xc4 - Write Address Filter Register \\#6"]
    #[inline(always)]
    pub const fn spi0c4(&self) -> &Spi0c4 {
        &self.spi0c4
    }
    #[doc = "0x100 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi100(&self) -> &Spi100 {
        &self.spi100
    }
    #[doc = "0x104 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi104(&self) -> &Spi104 {
        &self.spi104
    }
    #[doc = "0x108 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi108(&self) -> &Spi108 {
        &self.spi108
    }
    #[doc = "0x10c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi10c(&self) -> &Spi10c {
        &self.spi10c
    }
    #[doc = "0x110 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi110(&self) -> &Spi110 {
        &self.spi110
    }
    #[doc = "0x114 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi114(&self) -> &Spi114 {
        &self.spi114
    }
    #[doc = "0x118 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi118(&self) -> &Spi118 {
        &self.spi118
    }
    #[doc = "0x11c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi11c(&self) -> &Spi11c {
        &self.spi11c
    }
    #[doc = "0x120 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi120(&self) -> &Spi120 {
        &self.spi120
    }
    #[doc = "0x124 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi124(&self) -> &Spi124 {
        &self.spi124
    }
    #[doc = "0x128 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi128(&self) -> &Spi128 {
        &self.spi128
    }
    #[doc = "0x12c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi12c(&self) -> &Spi12c {
        &self.spi12c
    }
    #[doc = "0x130 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi130(&self) -> &Spi130 {
        &self.spi130
    }
    #[doc = "0x134 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi134(&self) -> &Spi134 {
        &self.spi134
    }
    #[doc = "0x138 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi138(&self) -> &Spi138 {
        &self.spi138
    }
    #[doc = "0x13c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi13c(&self) -> &Spi13c {
        &self.spi13c
    }
    #[doc = "0x140 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi140(&self) -> &Spi140 {
        &self.spi140
    }
    #[doc = "0x144 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi144(&self) -> &Spi144 {
        &self.spi144
    }
    #[doc = "0x148 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi148(&self) -> &Spi148 {
        &self.spi148
    }
    #[doc = "0x150 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi150(&self) -> &Spi150 {
        &self.spi150
    }
    #[doc = "0x154 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi154(&self) -> &Spi154 {
        &self.spi154
    }
    #[doc = "0x158 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi158(&self) -> &Spi158 {
        &self.spi158
    }
    #[doc = "0x15c - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi15c(&self) -> &Spi15c {
        &self.spi15c
    }
    #[doc = "0x160 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi160(&self) -> &Spi160 {
        &self.spi160
    }
    #[doc = "0x164 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi164(&self) -> &Spi164 {
        &self.spi164
    }
    #[doc = "0x168 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi168(&self) -> &Spi168 {
        &self.spi168
    }
    #[doc = "0x16c - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi16c(&self) -> &Spi16c {
        &self.spi16c
    }
    #[doc = "0x170 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi170(&self) -> &Spi170 {
        &self.spi170
    }
    #[doc = "0x174 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi174(&self) -> &Spi174 {
        &self.spi174
    }
    #[doc = "0x178 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi178(&self) -> &Spi178 {
        &self.spi178
    }
    #[doc = "0x17c - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi17c(&self) -> &Spi17c {
        &self.spi17c
    }
    #[doc = "0x200 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi200(&self) -> &Spi200 {
        &self.spi200
    }
    #[doc = "0x204 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi204(&self) -> &Spi204 {
        &self.spi204
    }
    #[doc = "0x208 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi208(&self) -> &Spi208 {
        &self.spi208
    }
    #[doc = "0x20c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi20c(&self) -> &Spi20c {
        &self.spi20c
    }
    #[doc = "0x210 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi210(&self) -> &Spi210 {
        &self.spi210
    }
    #[doc = "0x214 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi214(&self) -> &Spi214 {
        &self.spi214
    }
    #[doc = "0x218 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi218(&self) -> &Spi218 {
        &self.spi218
    }
    #[doc = "0x21c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi21c(&self) -> &Spi21c {
        &self.spi21c
    }
    #[doc = "0x220 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi220(&self) -> &Spi220 {
        &self.spi220
    }
    #[doc = "0x224 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi224(&self) -> &Spi224 {
        &self.spi224
    }
    #[doc = "0x228 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi228(&self) -> &Spi228 {
        &self.spi228
    }
    #[doc = "0x22c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi22c(&self) -> &Spi22c {
        &self.spi22c
    }
    #[doc = "0x230 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi230(&self) -> &Spi230 {
        &self.spi230
    }
    #[doc = "0x234 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi234(&self) -> &Spi234 {
        &self.spi234
    }
    #[doc = "0x238 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi238(&self) -> &Spi238 {
        &self.spi238
    }
    #[doc = "0x23c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi23c(&self) -> &Spi23c {
        &self.spi23c
    }
    #[doc = "0x240 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi240(&self) -> &Spi240 {
        &self.spi240
    }
    #[doc = "0x244 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi244(&self) -> &Spi244 {
        &self.spi244
    }
    #[doc = "0x248 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi248(&self) -> &Spi248 {
        &self.spi248
    }
    #[doc = "0x24c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi24c(&self) -> &Spi24c {
        &self.spi24c
    }
    #[doc = "0x250 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi250(&self) -> &Spi250 {
        &self.spi250
    }
    #[doc = "0x254 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi254(&self) -> &Spi254 {
        &self.spi254
    }
    #[doc = "0x258 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi258(&self) -> &Spi258 {
        &self.spi258
    }
    #[doc = "0x25c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi25c(&self) -> &Spi25c {
        &self.spi25c
    }
    #[doc = "0x260 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi260(&self) -> &Spi260 {
        &self.spi260
    }
    #[doc = "0x264 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi264(&self) -> &Spi264 {
        &self.spi264
    }
    #[doc = "0x268 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi268(&self) -> &Spi268 {
        &self.spi268
    }
    #[doc = "0x26c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi26c(&self) -> &Spi26c {
        &self.spi26c
    }
    #[doc = "0x270 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi270(&self) -> &Spi270 {
        &self.spi270
    }
    #[doc = "0x274 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi274(&self) -> &Spi274 {
        &self.spi274
    }
    #[doc = "0x278 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi278(&self) -> &Spi278 {
        &self.spi278
    }
    #[doc = "0x27c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi27c(&self) -> &Spi27c {
        &self.spi27c
    }
    #[doc = "0x280 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi280(&self) -> &Spi280 {
        &self.spi280
    }
    #[doc = "0x284 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi284(&self) -> &Spi284 {
        &self.spi284
    }
    #[doc = "0x288 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi288(&self) -> &Spi288 {
        &self.spi288
    }
    #[doc = "0x28c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi28c(&self) -> &Spi28c {
        &self.spi28c
    }
    #[doc = "0x290 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi290(&self) -> &Spi290 {
        &self.spi290
    }
    #[doc = "0x294 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi294(&self) -> &Spi294 {
        &self.spi294
    }
    #[doc = "0x298 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi298(&self) -> &Spi298 {
        &self.spi298
    }
    #[doc = "0x29c - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi29c(&self) -> &Spi29c {
        &self.spi29c
    }
    #[doc = "0x2a0 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2a0(&self) -> &Spi2a0 {
        &self.spi2a0
    }
    #[doc = "0x2a4 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2a4(&self) -> &Spi2a4 {
        &self.spi2a4
    }
    #[doc = "0x2a8 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2a8(&self) -> &Spi2a8 {
        &self.spi2a8
    }
    #[doc = "0x2ac - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2ac(&self) -> &Spi2ac {
        &self.spi2ac
    }
    #[doc = "0x2b0 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2b0(&self) -> &Spi2b0 {
        &self.spi2b0
    }
    #[doc = "0x2b4 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2b4(&self) -> &Spi2b4 {
        &self.spi2b4
    }
    #[doc = "0x2b8 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2b8(&self) -> &Spi2b8 {
        &self.spi2b8
    }
    #[doc = "0x2bc - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2bc(&self) -> &Spi2bc {
        &self.spi2bc
    }
    #[doc = "0x2c0 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2c0(&self) -> &Spi2c0 {
        &self.spi2c0
    }
    #[doc = "0x2c4 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2c4(&self) -> &Spi2c4 {
        &self.spi2c4
    }
    #[doc = "0x2c8 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2c8(&self) -> &Spi2c8 {
        &self.spi2c8
    }
    #[doc = "0x2cc - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2cc(&self) -> &Spi2cc {
        &self.spi2cc
    }
    #[doc = "0x2d0 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2d0(&self) -> &Spi2d0 {
        &self.spi2d0
    }
    #[doc = "0x2d4 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2d4(&self) -> &Spi2d4 {
        &self.spi2d4
    }
    #[doc = "0x2d8 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2d8(&self) -> &Spi2d8 {
        &self.spi2d8
    }
    #[doc = "0x2dc - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2dc(&self) -> &Spi2dc {
        &self.spi2dc
    }
    #[doc = "0x2e0 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2e0(&self) -> &Spi2e0 {
        &self.spi2e0
    }
    #[doc = "0x2e4 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2e4(&self) -> &Spi2e4 {
        &self.spi2e4
    }
    #[doc = "0x2e8 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2e8(&self) -> &Spi2e8 {
        &self.spi2e8
    }
    #[doc = "0x2ec - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2ec(&self) -> &Spi2ec {
        &self.spi2ec
    }
    #[doc = "0x2f0 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2f0(&self) -> &Spi2f0 {
        &self.spi2f0
    }
    #[doc = "0x2f4 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2f4(&self) -> &Spi2f4 {
        &self.spi2f4
    }
    #[doc = "0x2f8 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2f8(&self) -> &Spi2f8 {
        &self.spi2f8
    }
    #[doc = "0x2fc - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spi2fc(&self) -> &Spi2fc {
        &self.spi2fc
    }
}
#[doc = "SPI000 (rw) register accessor: SPI Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi000`] module"]
#[doc(alias = "SPI000")]
pub type Spi000 = crate::Reg<spi000::Spi000Spec>;
#[doc = "SPI Flash Configuration Register"]
pub mod spi000;
#[doc = "SPI004 (rw) register accessor: CE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi004`] module"]
#[doc(alias = "SPI004")]
pub type Spi004 = crate::Reg<spi004::Spi004Spec>;
#[doc = "CE Control Register"]
pub mod spi004;
#[doc = "SPI008 (rw) register accessor: Interrupt Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi008`] module"]
#[doc(alias = "SPI008")]
pub type Spi008 = crate::Reg<spi008::Spi008Spec>;
#[doc = "Interrupt Control and Status Register"]
pub mod spi008;
#[doc = "SPI00C (rw) register accessor: Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi00c`] module"]
#[doc(alias = "SPI00C")]
pub type Spi00c = crate::Reg<spi00c::Spi00cSpec>;
#[doc = "Command Control Register"]
pub mod spi00c;
#[doc = "SPI010 (rw) register accessor: CE0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi010`] module"]
#[doc(alias = "SPI010")]
pub type Spi010 = crate::Reg<spi010::Spi010Spec>;
#[doc = "CE0 Control Register"]
pub mod spi010;
#[doc = "SPI014 (rw) register accessor: CE1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi014`] module"]
#[doc(alias = "SPI014")]
pub type Spi014 = crate::Reg<spi014::Spi014Spec>;
#[doc = "CE1 Control Register"]
pub mod spi014;
#[doc = "SPI030 (rw) register accessor: CE0 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi030`] module"]
#[doc(alias = "SPI030")]
pub type Spi030 = crate::Reg<spi030::Spi030Spec>;
#[doc = "CE0 Address Decoding Range Register"]
pub mod spi030;
#[doc = "SPI034 (rw) register accessor: CE1 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi034`] module"]
#[doc(alias = "SPI034")]
pub type Spi034 = crate::Reg<spi034::Spi034Spec>;
#[doc = "CE1 Address Decoding Range Register"]
pub mod spi034;
#[doc = "SPI054 (rw) register accessor: SPI Dummy Cycle Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi054`] module"]
#[doc(alias = "SPI054")]
pub type Spi054 = crate::Reg<spi054::Spi054Spec>;
#[doc = "SPI Dummy Cycle Data Register"]
pub mod spi054;
#[doc = "SPI080 (rw) register accessor: DMA Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi080`] module"]
#[doc(alias = "SPI080")]
pub type Spi080 = crate::Reg<spi080::Spi080Spec>;
#[doc = "DMA Control/Status Register"]
pub mod spi080;
#[doc = "SPI084 (rw) register accessor: DMA Flash Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi084`] module"]
#[doc(alias = "SPI084")]
pub type Spi084 = crate::Reg<spi084::Spi084Spec>;
#[doc = "DMA Flash Side Address"]
pub mod spi084;
#[doc = "SPI088 (rw) register accessor: DMA DRAM/SRAM Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi088`] module"]
#[doc(alias = "SPI088")]
pub type Spi088 = crate::Reg<spi088::Spi088Spec>;
#[doc = "DMA DRAM/SRAM Side Address"]
pub mod spi088;
#[doc = "SPI08C (rw) register accessor: DMA Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi08c`] module"]
#[doc(alias = "SPI08C")]
pub type Spi08c = crate::Reg<spi08c::Spi08cSpec>;
#[doc = "DMA Length Register"]
pub mod spi08c;
#[doc = "SPI090 (rw) register accessor: CheckSum Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`spi090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi090`] module"]
#[doc(alias = "SPI090")]
pub type Spi090 = crate::Reg<spi090::Spi090Spec>;
#[doc = "CheckSum Calculation Result"]
pub mod spi090;
#[doc = "SPI094 (rw) register accessor: CE0 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi094`] module"]
#[doc(alias = "SPI094")]
pub type Spi094 = crate::Reg<spi094::Spi094Spec>;
#[doc = "CE0 SPI Flash Read Timing Compensation"]
pub mod spi094;
#[doc = "SPI098 (rw) register accessor: CE1 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi098`] module"]
#[doc(alias = "SPI098")]
pub type Spi098 = crate::Reg<spi098::Spi098Spec>;
#[doc = "CE1 SPI Flash Read Timing Compensation"]
pub mod spi098;
#[doc = "SPI0A0 (rw) register accessor: Command Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a0`] module"]
#[doc(alias = "SPI0A0")]
pub type Spi0a0 = crate::Reg<spi0a0::Spi0a0Spec>;
#[doc = "Command Filter Control Register"]
pub mod spi0a0;
#[doc = "SPI0A4 (rw) register accessor: Write Address Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a4`] module"]
#[doc(alias = "SPI0A4")]
pub type Spi0a4 = crate::Reg<spi0a4::Spi0a4Spec>;
#[doc = "Write Address Filter Control Register"]
pub mod spi0a4;
#[doc = "SPI0A8 (rw) register accessor: Register Lock Control Register (SRST\\#)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a8`] module"]
#[doc(alias = "SPI0A8")]
pub type Spi0a8 = crate::Reg<spi0a8::Spi0a8Spec>;
#[doc = "Register Lock Control Register (SRST\\#)"]
pub mod spi0a8;
#[doc = "SPI0AC (rw) register accessor: Register Lock Control Register (Watchdog)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0ac`] module"]
#[doc(alias = "SPI0AC")]
pub type Spi0ac = crate::Reg<spi0ac::Spi0acSpec>;
#[doc = "Register Lock Control Register (Watchdog)"]
pub mod spi0ac;
#[doc = "SPI0B0 (rw) register accessor: Write Address Filter Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0b0`] module"]
#[doc(alias = "SPI0B0")]
pub type Spi0b0 = crate::Reg<spi0b0::Spi0b0Spec>;
#[doc = "Write Address Filter Register \\#1"]
pub mod spi0b0;
#[doc = "SPI0B4 (rw) register accessor: Write Address Filter Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0b4`] module"]
#[doc(alias = "SPI0B4")]
pub type Spi0b4 = crate::Reg<spi0b4::Spi0b4Spec>;
#[doc = "Write Address Filter Register \\#2"]
pub mod spi0b4;
#[doc = "SPI0B8 (rw) register accessor: Write Address Filter Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0b8`] module"]
#[doc(alias = "SPI0B8")]
pub type Spi0b8 = crate::Reg<spi0b8::Spi0b8Spec>;
#[doc = "Write Address Filter Register \\#3"]
pub mod spi0b8;
#[doc = "SPI0BC (rw) register accessor: Write Address Filter Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0bc`] module"]
#[doc(alias = "SPI0BC")]
pub type Spi0bc = crate::Reg<spi0bc::Spi0bcSpec>;
#[doc = "Write Address Filter Register \\#4"]
pub mod spi0bc;
#[doc = "SPI0C0 (rw) register accessor: Write Address Filter Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0c0`] module"]
#[doc(alias = "SPI0C0")]
pub type Spi0c0 = crate::Reg<spi0c0::Spi0c0Spec>;
#[doc = "Write Address Filter Register \\#5"]
pub mod spi0c0;
#[doc = "SPI0C4 (rw) register accessor: Write Address Filter Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0c4`] module"]
#[doc(alias = "SPI0C4")]
pub type Spi0c4 = crate::Reg<spi0c4::Spi0c4Spec>;
#[doc = "Write Address Filter Register \\#6"]
pub mod spi0c4;
#[doc = "SPI100 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi100`] module"]
#[doc(alias = "SPI100")]
pub type Spi100 = crate::Reg<spi100::Spi100Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi100;
#[doc = "SPI104 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi104`] module"]
#[doc(alias = "SPI104")]
pub type Spi104 = crate::Reg<spi104::Spi104Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi104;
#[doc = "SPI108 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi108`] module"]
#[doc(alias = "SPI108")]
pub type Spi108 = crate::Reg<spi108::Spi108Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi108;
#[doc = "SPI10C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi10c`] module"]
#[doc(alias = "SPI10C")]
pub type Spi10c = crate::Reg<spi10c::Spi10cSpec>;
#[doc = "Fully Qualified Command"]
pub mod spi10c;
#[doc = "SPI110 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi110`] module"]
#[doc(alias = "SPI110")]
pub type Spi110 = crate::Reg<spi110::Spi110Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi110;
#[doc = "SPI114 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi114`] module"]
#[doc(alias = "SPI114")]
pub type Spi114 = crate::Reg<spi114::Spi114Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi114;
#[doc = "SPI118 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi118`] module"]
#[doc(alias = "SPI118")]
pub type Spi118 = crate::Reg<spi118::Spi118Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi118;
#[doc = "SPI11C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi11c`] module"]
#[doc(alias = "SPI11C")]
pub type Spi11c = crate::Reg<spi11c::Spi11cSpec>;
#[doc = "Fully Qualified Command"]
pub mod spi11c;
#[doc = "SPI120 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi120`] module"]
#[doc(alias = "SPI120")]
pub type Spi120 = crate::Reg<spi120::Spi120Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi120;
#[doc = "SPI124 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi124`] module"]
#[doc(alias = "SPI124")]
pub type Spi124 = crate::Reg<spi124::Spi124Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi124;
#[doc = "SPI128 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi128`] module"]
#[doc(alias = "SPI128")]
pub type Spi128 = crate::Reg<spi128::Spi128Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi128;
#[doc = "SPI12C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi12c`] module"]
#[doc(alias = "SPI12C")]
pub type Spi12c = crate::Reg<spi12c::Spi12cSpec>;
#[doc = "Fully Qualified Command"]
pub mod spi12c;
#[doc = "SPI130 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi130`] module"]
#[doc(alias = "SPI130")]
pub type Spi130 = crate::Reg<spi130::Spi130Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi130;
#[doc = "SPI134 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi134`] module"]
#[doc(alias = "SPI134")]
pub type Spi134 = crate::Reg<spi134::Spi134Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi134;
#[doc = "SPI138 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi138`] module"]
#[doc(alias = "SPI138")]
pub type Spi138 = crate::Reg<spi138::Spi138Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi138;
#[doc = "SPI13C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi13c`] module"]
#[doc(alias = "SPI13C")]
pub type Spi13c = crate::Reg<spi13c::Spi13cSpec>;
#[doc = "Fully Qualified Command"]
pub mod spi13c;
#[doc = "SPI140 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi140`] module"]
#[doc(alias = "SPI140")]
pub type Spi140 = crate::Reg<spi140::Spi140Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi140;
#[doc = "SPI144 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi144`] module"]
#[doc(alias = "SPI144")]
pub type Spi144 = crate::Reg<spi144::Spi144Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi144;
#[doc = "SPI148 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi148`] module"]
#[doc(alias = "SPI148")]
pub type Spi148 = crate::Reg<spi148::Spi148Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi148;
#[doc = "SPI150 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi150`] module"]
#[doc(alias = "SPI150")]
pub type Spi150 = crate::Reg<spi150::Spi150Spec>;
#[doc = "Address Qualified Command"]
pub mod spi150;
#[doc = "SPI154 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi154`] module"]
#[doc(alias = "SPI154")]
pub type Spi154 = crate::Reg<spi154::Spi154Spec>;
#[doc = "Address Qualified Command"]
pub mod spi154;
#[doc = "SPI158 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi158`] module"]
#[doc(alias = "SPI158")]
pub type Spi158 = crate::Reg<spi158::Spi158Spec>;
#[doc = "Address Qualified Command"]
pub mod spi158;
#[doc = "SPI15C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi15c`] module"]
#[doc(alias = "SPI15C")]
pub type Spi15c = crate::Reg<spi15c::Spi15cSpec>;
#[doc = "Address Qualified Command"]
pub mod spi15c;
#[doc = "SPI160 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi160`] module"]
#[doc(alias = "SPI160")]
pub type Spi160 = crate::Reg<spi160::Spi160Spec>;
#[doc = "Address Qualified Command"]
pub mod spi160;
#[doc = "SPI164 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi164`] module"]
#[doc(alias = "SPI164")]
pub type Spi164 = crate::Reg<spi164::Spi164Spec>;
#[doc = "Address Qualified Command"]
pub mod spi164;
#[doc = "SPI168 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi168`] module"]
#[doc(alias = "SPI168")]
pub type Spi168 = crate::Reg<spi168::Spi168Spec>;
#[doc = "Address Qualified Command"]
pub mod spi168;
#[doc = "SPI16C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi16c`] module"]
#[doc(alias = "SPI16C")]
pub type Spi16c = crate::Reg<spi16c::Spi16cSpec>;
#[doc = "Address Qualified Command"]
pub mod spi16c;
#[doc = "SPI170 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi170`] module"]
#[doc(alias = "SPI170")]
pub type Spi170 = crate::Reg<spi170::Spi170Spec>;
#[doc = "Address Qualified Command"]
pub mod spi170;
#[doc = "SPI174 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi174`] module"]
#[doc(alias = "SPI174")]
pub type Spi174 = crate::Reg<spi174::Spi174Spec>;
#[doc = "Address Qualified Command"]
pub mod spi174;
#[doc = "SPI178 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi178`] module"]
#[doc(alias = "SPI178")]
pub type Spi178 = crate::Reg<spi178::Spi178Spec>;
#[doc = "Address Qualified Command"]
pub mod spi178;
#[doc = "SPI17C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi17c`] module"]
#[doc(alias = "SPI17C")]
pub type Spi17c = crate::Reg<spi17c::Spi17cSpec>;
#[doc = "Address Qualified Command"]
pub mod spi17c;
#[doc = "SPI200 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi200`] module"]
#[doc(alias = "SPI200")]
pub type Spi200 = crate::Reg<spi200::Spi200Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi200;
#[doc = "SPI204 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi204`] module"]
#[doc(alias = "SPI204")]
pub type Spi204 = crate::Reg<spi204::Spi204Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi204;
#[doc = "SPI208 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi208`] module"]
#[doc(alias = "SPI208")]
pub type Spi208 = crate::Reg<spi208::Spi208Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi208;
#[doc = "SPI20C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi20c`] module"]
#[doc(alias = "SPI20C")]
pub type Spi20c = crate::Reg<spi20c::Spi20cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi20c;
#[doc = "SPI210 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi210`] module"]
#[doc(alias = "SPI210")]
pub type Spi210 = crate::Reg<spi210::Spi210Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi210;
#[doc = "SPI214 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi214`] module"]
#[doc(alias = "SPI214")]
pub type Spi214 = crate::Reg<spi214::Spi214Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi214;
#[doc = "SPI218 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi218`] module"]
#[doc(alias = "SPI218")]
pub type Spi218 = crate::Reg<spi218::Spi218Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi218;
#[doc = "SPI21C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi21c`] module"]
#[doc(alias = "SPI21C")]
pub type Spi21c = crate::Reg<spi21c::Spi21cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi21c;
#[doc = "SPI220 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi220`] module"]
#[doc(alias = "SPI220")]
pub type Spi220 = crate::Reg<spi220::Spi220Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi220;
#[doc = "SPI224 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi224`] module"]
#[doc(alias = "SPI224")]
pub type Spi224 = crate::Reg<spi224::Spi224Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi224;
#[doc = "SPI228 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi228`] module"]
#[doc(alias = "SPI228")]
pub type Spi228 = crate::Reg<spi228::Spi228Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi228;
#[doc = "SPI22C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi22c`] module"]
#[doc(alias = "SPI22C")]
pub type Spi22c = crate::Reg<spi22c::Spi22cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi22c;
#[doc = "SPI230 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi230`] module"]
#[doc(alias = "SPI230")]
pub type Spi230 = crate::Reg<spi230::Spi230Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi230;
#[doc = "SPI234 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi234`] module"]
#[doc(alias = "SPI234")]
pub type Spi234 = crate::Reg<spi234::Spi234Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi234;
#[doc = "SPI238 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi238`] module"]
#[doc(alias = "SPI238")]
pub type Spi238 = crate::Reg<spi238::Spi238Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi238;
#[doc = "SPI23C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi23c`] module"]
#[doc(alias = "SPI23C")]
pub type Spi23c = crate::Reg<spi23c::Spi23cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi23c;
#[doc = "SPI240 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi240`] module"]
#[doc(alias = "SPI240")]
pub type Spi240 = crate::Reg<spi240::Spi240Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi240;
#[doc = "SPI244 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi244`] module"]
#[doc(alias = "SPI244")]
pub type Spi244 = crate::Reg<spi244::Spi244Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi244;
#[doc = "SPI248 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi248`] module"]
#[doc(alias = "SPI248")]
pub type Spi248 = crate::Reg<spi248::Spi248Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi248;
#[doc = "SPI24C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi24c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi24c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi24c`] module"]
#[doc(alias = "SPI24C")]
pub type Spi24c = crate::Reg<spi24c::Spi24cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi24c;
#[doc = "SPI250 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi250`] module"]
#[doc(alias = "SPI250")]
pub type Spi250 = crate::Reg<spi250::Spi250Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi250;
#[doc = "SPI254 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi254`] module"]
#[doc(alias = "SPI254")]
pub type Spi254 = crate::Reg<spi254::Spi254Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi254;
#[doc = "SPI258 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi258`] module"]
#[doc(alias = "SPI258")]
pub type Spi258 = crate::Reg<spi258::Spi258Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi258;
#[doc = "SPI25C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi25c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi25c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi25c`] module"]
#[doc(alias = "SPI25C")]
pub type Spi25c = crate::Reg<spi25c::Spi25cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi25c;
#[doc = "SPI260 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi260`] module"]
#[doc(alias = "SPI260")]
pub type Spi260 = crate::Reg<spi260::Spi260Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi260;
#[doc = "SPI264 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi264`] module"]
#[doc(alias = "SPI264")]
pub type Spi264 = crate::Reg<spi264::Spi264Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi264;
#[doc = "SPI268 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi268`] module"]
#[doc(alias = "SPI268")]
pub type Spi268 = crate::Reg<spi268::Spi268Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi268;
#[doc = "SPI26C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi26c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi26c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi26c`] module"]
#[doc(alias = "SPI26C")]
pub type Spi26c = crate::Reg<spi26c::Spi26cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi26c;
#[doc = "SPI270 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi270`] module"]
#[doc(alias = "SPI270")]
pub type Spi270 = crate::Reg<spi270::Spi270Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi270;
#[doc = "SPI274 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi274`] module"]
#[doc(alias = "SPI274")]
pub type Spi274 = crate::Reg<spi274::Spi274Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi274;
#[doc = "SPI278 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi278`] module"]
#[doc(alias = "SPI278")]
pub type Spi278 = crate::Reg<spi278::Spi278Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi278;
#[doc = "SPI27C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi27c`] module"]
#[doc(alias = "SPI27C")]
pub type Spi27c = crate::Reg<spi27c::Spi27cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi27c;
#[doc = "SPI280 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi280`] module"]
#[doc(alias = "SPI280")]
pub type Spi280 = crate::Reg<spi280::Spi280Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi280;
#[doc = "SPI284 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi284`] module"]
#[doc(alias = "SPI284")]
pub type Spi284 = crate::Reg<spi284::Spi284Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi284;
#[doc = "SPI288 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi288`] module"]
#[doc(alias = "SPI288")]
pub type Spi288 = crate::Reg<spi288::Spi288Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi288;
#[doc = "SPI28C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi28c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi28c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi28c`] module"]
#[doc(alias = "SPI28C")]
pub type Spi28c = crate::Reg<spi28c::Spi28cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi28c;
#[doc = "SPI290 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi290`] module"]
#[doc(alias = "SPI290")]
pub type Spi290 = crate::Reg<spi290::Spi290Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi290;
#[doc = "SPI294 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi294::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi294::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi294`] module"]
#[doc(alias = "SPI294")]
pub type Spi294 = crate::Reg<spi294::Spi294Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi294;
#[doc = "SPI298 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi298`] module"]
#[doc(alias = "SPI298")]
pub type Spi298 = crate::Reg<spi298::Spi298Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi298;
#[doc = "SPI29C (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi29c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi29c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi29c`] module"]
#[doc(alias = "SPI29C")]
pub type Spi29c = crate::Reg<spi29c::Spi29cSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi29c;
#[doc = "SPI2A0 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2a0`] module"]
#[doc(alias = "SPI2A0")]
pub type Spi2a0 = crate::Reg<spi2a0::Spi2a0Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2a0;
#[doc = "SPI2A4 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2a4`] module"]
#[doc(alias = "SPI2A4")]
pub type Spi2a4 = crate::Reg<spi2a4::Spi2a4Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2a4;
#[doc = "SPI2A8 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2a8`] module"]
#[doc(alias = "SPI2A8")]
pub type Spi2a8 = crate::Reg<spi2a8::Spi2a8Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2a8;
#[doc = "SPI2AC (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2ac`] module"]
#[doc(alias = "SPI2AC")]
pub type Spi2ac = crate::Reg<spi2ac::Spi2acSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2ac;
#[doc = "SPI2B0 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2b0`] module"]
#[doc(alias = "SPI2B0")]
pub type Spi2b0 = crate::Reg<spi2b0::Spi2b0Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2b0;
#[doc = "SPI2B4 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2b4`] module"]
#[doc(alias = "SPI2B4")]
pub type Spi2b4 = crate::Reg<spi2b4::Spi2b4Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2b4;
#[doc = "SPI2B8 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2b8`] module"]
#[doc(alias = "SPI2B8")]
pub type Spi2b8 = crate::Reg<spi2b8::Spi2b8Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2b8;
#[doc = "SPI2BC (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2bc`] module"]
#[doc(alias = "SPI2BC")]
pub type Spi2bc = crate::Reg<spi2bc::Spi2bcSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2bc;
#[doc = "SPI2C0 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2c0`] module"]
#[doc(alias = "SPI2C0")]
pub type Spi2c0 = crate::Reg<spi2c0::Spi2c0Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2c0;
#[doc = "SPI2C4 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2c4`] module"]
#[doc(alias = "SPI2C4")]
pub type Spi2c4 = crate::Reg<spi2c4::Spi2c4Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2c4;
#[doc = "SPI2C8 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2c8`] module"]
#[doc(alias = "SPI2C8")]
pub type Spi2c8 = crate::Reg<spi2c8::Spi2c8Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2c8;
#[doc = "SPI2CC (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2cc`] module"]
#[doc(alias = "SPI2CC")]
pub type Spi2cc = crate::Reg<spi2cc::Spi2ccSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2cc;
#[doc = "SPI2D0 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2d0`] module"]
#[doc(alias = "SPI2D0")]
pub type Spi2d0 = crate::Reg<spi2d0::Spi2d0Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2d0;
#[doc = "SPI2D4 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2d4`] module"]
#[doc(alias = "SPI2D4")]
pub type Spi2d4 = crate::Reg<spi2d4::Spi2d4Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2d4;
#[doc = "SPI2D8 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2d8`] module"]
#[doc(alias = "SPI2D8")]
pub type Spi2d8 = crate::Reg<spi2d8::Spi2d8Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2d8;
#[doc = "SPI2DC (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2dc`] module"]
#[doc(alias = "SPI2DC")]
pub type Spi2dc = crate::Reg<spi2dc::Spi2dcSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2dc;
#[doc = "SPI2E0 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2e0`] module"]
#[doc(alias = "SPI2E0")]
pub type Spi2e0 = crate::Reg<spi2e0::Spi2e0Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2e0;
#[doc = "SPI2E4 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2e4`] module"]
#[doc(alias = "SPI2E4")]
pub type Spi2e4 = crate::Reg<spi2e4::Spi2e4Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2e4;
#[doc = "SPI2E8 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2e8`] module"]
#[doc(alias = "SPI2E8")]
pub type Spi2e8 = crate::Reg<spi2e8::Spi2e8Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2e8;
#[doc = "SPI2EC (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2ec`] module"]
#[doc(alias = "SPI2EC")]
pub type Spi2ec = crate::Reg<spi2ec::Spi2ecSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2ec;
#[doc = "SPI2F0 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2f0`] module"]
#[doc(alias = "SPI2F0")]
pub type Spi2f0 = crate::Reg<spi2f0::Spi2f0Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2f0;
#[doc = "SPI2F4 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2f4`] module"]
#[doc(alias = "SPI2F4")]
pub type Spi2f4 = crate::Reg<spi2f4::Spi2f4Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2f4;
#[doc = "SPI2F8 (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2f8`] module"]
#[doc(alias = "SPI2F8")]
pub type Spi2f8 = crate::Reg<spi2f8::Spi2f8Spec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2f8;
#[doc = "SPI2FC (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2fc`] module"]
#[doc(alias = "SPI2FC")]
pub type Spi2fc = crate::Reg<spi2fc::Spi2fcSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spi2fc;

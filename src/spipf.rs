#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spipf000: Spipf000,
    spipf004: Spipf004,
    spipf008: Spipf008,
    spipf00c: Spipf00c,
    spipf010: Spipf010,
    spipf014: Spipf014,
    spipf018: Spipf018,
    _reserved7: [u8; 0x60],
    spipf07c: Spipf07c,
    spipf080: Spipf080,
    spipf084: Spipf084,
    spipf088: Spipf088,
    spipf08c: Spipf08c,
    spipf090: Spipf090,
    spipf094: Spipf094,
    spipf098: Spipf098,
    spipf09c: Spipf09c,
    spipf0a0: Spipf0a0,
    spipf0a4: Spipf0a4,
    spipf0a8: Spipf0a8,
    spipf0ac: Spipf0ac,
    spipf0b0: Spipf0b0,
    spipf0b4: Spipf0b4,
    spipf0b8: Spipf0b8,
    spipf0bc: Spipf0bc,
    spipf0c0: Spipf0c0,
    spipf0c4: Spipf0c4,
    spipf0c8: Spipf0c8,
    spipf0cc: Spipf0cc,
    spipf0d0: Spipf0d0,
    spipf0d4: Spipf0d4,
    spipf0d8: Spipf0d8,
    spipf0dc: Spipf0dc,
    spipf0e0: Spipf0e0,
    spipf0e4: Spipf0e4,
    spipf0e8: Spipf0e8,
    spipf0ec: Spipf0ec,
    spipf0f0: Spipf0f0,
    spipf0f4: Spipf0f4,
    spipf0f8: Spipf0f8,
    _reserved39: [u8; 0x04],
    spipf100: Spipf100,
    spipf104: Spipf104,
    spipf108: Spipf108,
    spipf10c: Spipf10c,
    spipf110: Spipf110,
    spipf114: Spipf114,
    spipf118: Spipf118,
    spipf11c: Spipf11c,
    spipf120: Spipf120,
    spipf124: Spipf124,
    spipf128: Spipf128,
    spipf12c: Spipf12c,
    spipf130: Spipf130,
    spipf134: Spipf134,
    spipf138: Spipf138,
    spipf13c: Spipf13c,
    spipf140: Spipf140,
    spipf144: Spipf144,
    spipf148: Spipf148,
    spipf14c: Spipf14c,
    spipf150: Spipf150,
    spipf154: Spipf154,
    spipf158: Spipf158,
    spipf15c: Spipf15c,
    spipf160: Spipf160,
    spipf164: Spipf164,
    spipf168: Spipf168,
    spipf16c: Spipf16c,
    spipf170: Spipf170,
    spipf174: Spipf174,
    spipf178: Spipf178,
    spipf17c: Spipf17c,
    spipf180: Spipf180,
    spipf184: Spipf184,
    spipf188: Spipf188,
    spipf18c: Spipf18c,
    spipf190: Spipf190,
    spipf194: Spipf194,
    spipf198: Spipf198,
    spipf19c: Spipf19c,
    spipf1a0: Spipf1a0,
    spipf1a4: Spipf1a4,
    spipf1a8: Spipf1a8,
    spipf1ac: Spipf1ac,
    spipf1b0: Spipf1b0,
    spipf1b4: Spipf1b4,
    spipf1b8: Spipf1b8,
    spipf1bc: Spipf1bc,
    spipf1c0: Spipf1c0,
    spipf1c4: Spipf1c4,
    spipf1c8: Spipf1c8,
    spipf1cc: Spipf1cc,
    spipf1d0: Spipf1d0,
    spipf1d4: Spipf1d4,
    spipf1d8: Spipf1d8,
    spipf1dc: Spipf1dc,
    spipf1e0: Spipf1e0,
    spipf1e4: Spipf1e4,
    spipf1e8: Spipf1e8,
    spipf1ec: Spipf1ec,
    spipf1f0: Spipf1f0,
    spipf1f4: Spipf1f4,
    spipf1f8: Spipf1f8,
    spipf1fc: Spipf1fc,
    spipf200: Spipf200,
    spipf204: Spipf204,
    spipf208: Spipf208,
    spipf20c: Spipf20c,
    spipf210: Spipf210,
    spipf214: Spipf214,
    spipf218: Spipf218,
    spipf21c: Spipf21c,
    spipf220: Spipf220,
    spipf224: Spipf224,
    spipf228: Spipf228,
    spipf22c: Spipf22c,
    spipf230: Spipf230,
    spipf234: Spipf234,
    spipf238: Spipf238,
    spipf23c: Spipf23c,
    spipf240: Spipf240,
    spipf244: Spipf244,
    spipf248: Spipf248,
    spipf24c: Spipf24c,
    spipf250: Spipf250,
    spipf254: Spipf254,
    spipf258: Spipf258,
    spipf25c: Spipf25c,
    spipf260: Spipf260,
    spipf264: Spipf264,
    spipf268: Spipf268,
    spipf26c: Spipf26c,
    spipf270: Spipf270,
    spipf274: Spipf274,
    spipf278: Spipf278,
    spipf27c: Spipf27c,
    spipf280: Spipf280,
    spipf284: Spipf284,
    spipf288: Spipf288,
    spipf28c: Spipf28c,
    spipf290: Spipf290,
    spipf294: Spipf294,
    spipf298: Spipf298,
    spipf29c: Spipf29c,
    spipf2a0: Spipf2a0,
    spipf2a4: Spipf2a4,
    spipf2a8: Spipf2a8,
    spipf2ac: Spipf2ac,
    spipf2b0: Spipf2b0,
    spipf2b4: Spipf2b4,
    spipf2b8: Spipf2b8,
    spipf2bc: Spipf2bc,
    spipf2c0: Spipf2c0,
    spipf2c4: Spipf2c4,
    spipf2c8: Spipf2c8,
    spipf2cc: Spipf2cc,
    spipf2d0: Spipf2d0,
    spipf2d4: Spipf2d4,
    spipf2d8: Spipf2d8,
    spipf2dc: Spipf2dc,
    spipf2e0: Spipf2e0,
    spipf2e4: Spipf2e4,
    spipf2e8: Spipf2e8,
    spipf2ec: Spipf2ec,
    spipf2f0: Spipf2f0,
    spipf2f4: Spipf2f4,
    spipf2f8: Spipf2f8,
    spipf2fc: Spipf2fc,
    spipf300: Spipf300,
    spipf304: Spipf304,
    spipf308: Spipf308,
    spipf30c: Spipf30c,
    spipf310: Spipf310,
    spipf314: Spipf314,
    spipf318: Spipf318,
    spipf31c: Spipf31c,
    spipf320: Spipf320,
    spipf324: Spipf324,
    spipf328: Spipf328,
    spipf32c: Spipf32c,
    spipf330: Spipf330,
    spipf334: Spipf334,
    spipf338: Spipf338,
    spipf33c: Spipf33c,
    spipf340: Spipf340,
    spipf344: Spipf344,
    spipf348: Spipf348,
    spipf34c: Spipf34c,
    spipf350: Spipf350,
    spipf354: Spipf354,
    spipf358: Spipf358,
    spipf35c: Spipf35c,
    spipf360: Spipf360,
    spipf364: Spipf364,
    spipf368: Spipf368,
    spipf36c: Spipf36c,
    spipf370: Spipf370,
    spipf374: Spipf374,
    spipf378: Spipf378,
    spipf37c: Spipf37c,
    spipf380: Spipf380,
    spipf384: Spipf384,
    spipf388: Spipf388,
    spipf38c: Spipf38c,
    spipf390: Spipf390,
    spipf394: Spipf394,
    spipf398: Spipf398,
    spipf39c: Spipf39c,
    spipf3a0: Spipf3a0,
    spipf3a4: Spipf3a4,
    spipf3a8: Spipf3a8,
    spipf3ac: Spipf3ac,
    spipf3b0: Spipf3b0,
    spipf3b4: Spipf3b4,
    spipf3b8: Spipf3b8,
    spipf3bc: Spipf3bc,
    spipf3c0: Spipf3c0,
    spipf3c4: Spipf3c4,
    spipf3c8: Spipf3c8,
    spipf3cc: Spipf3cc,
    spipf3d0: Spipf3d0,
    spipf3d4: Spipf3d4,
    spipf3d8: Spipf3d8,
    spipf3dc: Spipf3dc,
    spipf3e0: Spipf3e0,
    spipf3e4: Spipf3e4,
    spipf3e8: Spipf3e8,
    spipf3ec: Spipf3ec,
    spipf3f0: Spipf3f0,
    spipf3f4: Spipf3f4,
    spipf3f8: Spipf3f8,
    spipf3fc: Spipf3fc,
    spipf400: Spipf400,
    spipf404: Spipf404,
    spipf408: Spipf408,
    spipf40c: Spipf40c,
    spipf410: Spipf410,
    spipf414: Spipf414,
    spipf418: Spipf418,
    spipf41c: Spipf41c,
    spipf420: Spipf420,
    spipf424: Spipf424,
    spipf428: Spipf428,
    spipf42c: Spipf42c,
    spipf430: Spipf430,
    spipf434: Spipf434,
    spipf438: Spipf438,
    spipf43c: Spipf43c,
    spipf440: Spipf440,
    spipf444: Spipf444,
    spipf448: Spipf448,
    spipf44c: Spipf44c,
    spipf450: Spipf450,
    spipf454: Spipf454,
    spipf458: Spipf458,
    spipf45c: Spipf45c,
    spipf460: Spipf460,
    spipf464: Spipf464,
    spipf468: Spipf468,
    spipf46c: Spipf46c,
    spipf470: Spipf470,
    spipf474: Spipf474,
    spipf478: Spipf478,
    spipf47c: Spipf47c,
    spipf480: Spipf480,
    spipf484: Spipf484,
    spipf488: Spipf488,
    spipf48c: Spipf48c,
    spipf490: Spipf490,
    spipf494: Spipf494,
    spipf498: Spipf498,
    spipf49c: Spipf49c,
    spipf4a0: Spipf4a0,
    spipf4a4: Spipf4a4,
    spipf4a8: Spipf4a8,
    spipf4ac: Spipf4ac,
    spipf4b0: Spipf4b0,
    spipf4b4: Spipf4b4,
    spipf4b8: Spipf4b8,
    spipf4bc: Spipf4bc,
    spipf4c0: Spipf4c0,
    spipf4c4: Spipf4c4,
    spipf4c8: Spipf4c8,
    spipf4cc: Spipf4cc,
    spipf4d0: Spipf4d0,
    spipf4d4: Spipf4d4,
    spipf4d8: Spipf4d8,
    spipf4dc: Spipf4dc,
    spipf4e0: Spipf4e0,
    spipf4e4: Spipf4e4,
    spipf4e8: Spipf4e8,
    spipf4ec: Spipf4ec,
    spipf4f0: Spipf4f0,
    spipf4f4: Spipf4f4,
    spipf4f8: Spipf4f8,
    spipf4fc: Spipf4fc,
    spipf500: Spipf500,
    spipf504: Spipf504,
    spipf508: Spipf508,
    spipf50c: Spipf50c,
    spipf510: Spipf510,
    spipf514: Spipf514,
    spipf518: Spipf518,
    spipf51c: Spipf51c,
    spipf520: Spipf520,
    spipf524: Spipf524,
    spipf528: Spipf528,
    spipf52c: Spipf52c,
    spipf530: Spipf530,
    spipf534: Spipf534,
    spipf538: Spipf538,
    spipf53c: Spipf53c,
    spipf540: Spipf540,
    spipf544: Spipf544,
    spipf548: Spipf548,
    spipf54c: Spipf54c,
    spipf550: Spipf550,
    spipf554: Spipf554,
    spipf558: Spipf558,
    spipf55c: Spipf55c,
    spipf560: Spipf560,
    spipf564: Spipf564,
    spipf568: Spipf568,
    spipf56c: Spipf56c,
    spipf570: Spipf570,
    spipf574: Spipf574,
    spipf578: Spipf578,
    spipf57c: Spipf57c,
    spipf580: Spipf580,
    spipf584: Spipf584,
    spipf588: Spipf588,
    spipf58c: Spipf58c,
    spipf590: Spipf590,
    spipf594: Spipf594,
    spipf598: Spipf598,
    spipf59c: Spipf59c,
    spipf5a0: Spipf5a0,
    spipf5a4: Spipf5a4,
    spipf5a8: Spipf5a8,
    spipf5ac: Spipf5ac,
    spipf5b0: Spipf5b0,
    spipf5b4: Spipf5b4,
    spipf5b8: Spipf5b8,
    spipf5bc: Spipf5bc,
    spipf5c0: Spipf5c0,
    spipf5c4: Spipf5c4,
    spipf5c8: Spipf5c8,
    spipf5cc: Spipf5cc,
    spipf5d0: Spipf5d0,
    spipf5d4: Spipf5d4,
    spipf5d8: Spipf5d8,
    spipf5dc: Spipf5dc,
    spipf5e0: Spipf5e0,
    spipf5e4: Spipf5e4,
    spipf5e8: Spipf5e8,
    spipf5ec: Spipf5ec,
    spipf5f0: Spipf5f0,
    spipf5f4: Spipf5f4,
    spipf5f8: Spipf5f8,
    spipf5fc: Spipf5fc,
    spipf600: Spipf600,
    spipf604: Spipf604,
    spipf608: Spipf608,
    spipf60c: Spipf60c,
    spipf610: Spipf610,
    spipf614: Spipf614,
    spipf618: Spipf618,
    spipf61c: Spipf61c,
    spipf620: Spipf620,
    spipf624: Spipf624,
    spipf628: Spipf628,
    spipf62c: Spipf62c,
    spipf630: Spipf630,
    spipf634: Spipf634,
    spipf638: Spipf638,
    spipf63c: Spipf63c,
    spipf640: Spipf640,
    spipf644: Spipf644,
    spipf648: Spipf648,
    spipf64c: Spipf64c,
    spipf650: Spipf650,
    spipf654: Spipf654,
    spipf658: Spipf658,
    spipf65c: Spipf65c,
    spipf660: Spipf660,
    spipf664: Spipf664,
    spipf668: Spipf668,
    spipf66c: Spipf66c,
    spipf670: Spipf670,
    spipf674: Spipf674,
    spipf678: Spipf678,
    spipf67c: Spipf67c,
    spipf680: Spipf680,
    spipf684: Spipf684,
    spipf688: Spipf688,
    spipf68c: Spipf68c,
    spipf690: Spipf690,
    spipf694: Spipf694,
    spipf698: Spipf698,
    spipf69c: Spipf69c,
    spipf6a0: Spipf6a0,
    spipf6a4: Spipf6a4,
    spipf6a8: Spipf6a8,
    spipf6ac: Spipf6ac,
    spipf6b0: Spipf6b0,
    spipf6b4: Spipf6b4,
    spipf6b8: Spipf6b8,
    spipf6bc: Spipf6bc,
    spipf6c0: Spipf6c0,
    spipf6c4: Spipf6c4,
    spipf6c8: Spipf6c8,
    spipf6cc: Spipf6cc,
    spipf6d0: Spipf6d0,
    spipf6d4: Spipf6d4,
    spipf6d8: Spipf6d8,
    spipf6dc: Spipf6dc,
    spipf6e0: Spipf6e0,
    spipf6e4: Spipf6e4,
    spipf6e8: Spipf6e8,
    spipf6ec: Spipf6ec,
    spipf6f0: Spipf6f0,
    spipf6f4: Spipf6f4,
    spipf6f8: Spipf6f8,
    spipf6fc: Spipf6fc,
    spipf700: Spipf700,
    spipf704: Spipf704,
    spipf708: Spipf708,
    spipf70c: Spipf70c,
    spipf710: Spipf710,
    spipf714: Spipf714,
    spipf718: Spipf718,
    spipf71c: Spipf71c,
    spipf720: Spipf720,
    spipf724: Spipf724,
    spipf728: Spipf728,
    spipf72c: Spipf72c,
    spipf730: Spipf730,
    spipf734: Spipf734,
    spipf738: Spipf738,
    spipf73c: Spipf73c,
    spipf740: Spipf740,
    spipf744: Spipf744,
    spipf748: Spipf748,
    spipf74c: Spipf74c,
    spipf750: Spipf750,
    spipf754: Spipf754,
    spipf758: Spipf758,
    spipf75c: Spipf75c,
    spipf760: Spipf760,
    spipf764: Spipf764,
    spipf768: Spipf768,
    spipf76c: Spipf76c,
    spipf770: Spipf770,
    spipf774: Spipf774,
    spipf778: Spipf778,
    spipf77c: Spipf77c,
    spipf780: Spipf780,
    spipf784: Spipf784,
    spipf788: Spipf788,
    spipf78c: Spipf78c,
    spipf790: Spipf790,
    spipf794: Spipf794,
    spipf798: Spipf798,
    spipf79c: Spipf79c,
    spipf7a0: Spipf7a0,
    spipf7a4: Spipf7a4,
    spipf7a8: Spipf7a8,
    spipf7ac: Spipf7ac,
    spipf7b0: Spipf7b0,
    spipf7b4: Spipf7b4,
    spipf7b8: Spipf7b8,
    spipf7bc: Spipf7bc,
    spipf7c0: Spipf7c0,
    spipf7c4: Spipf7c4,
    spipf7c8: Spipf7c8,
    spipf7cc: Spipf7cc,
    spipf7d0: Spipf7d0,
    spipf7d4: Spipf7d4,
    spipf7d8: Spipf7d8,
    spipf7dc: Spipf7dc,
    spipf7e0: Spipf7e0,
    spipf7e4: Spipf7e4,
    spipf7e8: Spipf7e8,
    spipf7ec: Spipf7ec,
    spipf7f0: Spipf7f0,
    spipf7f4: Spipf7f4,
    spipf7f8: Spipf7f8,
    spipf7fc: Spipf7fc,
    spipf800: Spipf800,
    spipf804: Spipf804,
    spipf808: Spipf808,
    spipf80c: Spipf80c,
    spipf810: Spipf810,
    spipf814: Spipf814,
    spipf818: Spipf818,
    spipf81c: Spipf81c,
    spipf820: Spipf820,
    spipf824: Spipf824,
    spipf828: Spipf828,
    spipf82c: Spipf82c,
    spipf830: Spipf830,
    spipf834: Spipf834,
    spipf838: Spipf838,
    spipf83c: Spipf83c,
    spipf840: Spipf840,
    spipf844: Spipf844,
    spipf848: Spipf848,
    spipf84c: Spipf84c,
    spipf850: Spipf850,
    spipf854: Spipf854,
    spipf858: Spipf858,
    spipf85c: Spipf85c,
    spipf860: Spipf860,
    spipf864: Spipf864,
    spipf868: Spipf868,
    spipf86c: Spipf86c,
    spipf870: Spipf870,
    spipf874: Spipf874,
    spipf878: Spipf878,
    spipf87c: Spipf87c,
    spipf880: Spipf880,
    spipf884: Spipf884,
    spipf888: Spipf888,
    spipf88c: Spipf88c,
    spipf890: Spipf890,
    spipf894: Spipf894,
    spipf898: Spipf898,
    spipf89c: Spipf89c,
    spipf8a0: Spipf8a0,
    spipf8a4: Spipf8a4,
    spipf8a8: Spipf8a8,
    spipf8ac: Spipf8ac,
    spipf8b0: Spipf8b0,
    spipf8b4: Spipf8b4,
    spipf8b8: Spipf8b8,
    spipf8bc: Spipf8bc,
    spipf8c0: Spipf8c0,
    spipf8c4: Spipf8c4,
    spipf8c8: Spipf8c8,
    spipf8cc: Spipf8cc,
    spipf8d0: Spipf8d0,
    spipf8d4: Spipf8d4,
    spipf8d8: Spipf8d8,
    spipf8dc: Spipf8dc,
    spipf8e0: Spipf8e0,
    spipf8e4: Spipf8e4,
    spipf8e8: Spipf8e8,
    spipf8ec: Spipf8ec,
    spipf8f0: Spipf8f0,
    spipf8f4: Spipf8f4,
    spipf8f8: Spipf8f8,
}
impl RegisterBlock {
    #[doc = "0x00 - Engine Control Register"]
    #[inline(always)]
    pub const fn spipf000(&self) -> &Spipf000 {
        &self.spipf000
    }
    #[doc = "0x04 - Interrupt Enable and Status Register"]
    #[inline(always)]
    pub const fn spipf004(&self) -> &Spipf004 {
        &self.spipf004
    }
    #[doc = "0x08 - EAR and Over Speed Register"]
    #[inline(always)]
    pub const fn spipf008(&self) -> &Spipf008 {
        &self.spipf008
    }
    #[doc = "0x0c - Block FIFO Data Register"]
    #[inline(always)]
    pub const fn spipf00c(&self) -> &Spipf00c {
        &self.spipf00c
    }
    #[doc = "0x10 - Block Log DMA Base Address"]
    #[inline(always)]
    pub const fn spipf010(&self) -> &Spipf010 {
        &self.spipf010
    }
    #[doc = "0x14 - Block Log DMA Size"]
    #[inline(always)]
    pub const fn spipf014(&self) -> &Spipf014 {
        &self.spipf014
    }
    #[doc = "0x18 - Block Log DMA Write Pointer"]
    #[inline(always)]
    pub const fn spipf018(&self) -> &Spipf018 {
        &self.spipf018
    }
    #[doc = "0x7c - Write Disable Register"]
    #[inline(always)]
    pub const fn spipf07c(&self) -> &Spipf07c {
        &self.spipf07c
    }
    #[doc = "0x80 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf080(&self) -> &Spipf080 {
        &self.spipf080
    }
    #[doc = "0x84 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf084(&self) -> &Spipf084 {
        &self.spipf084
    }
    #[doc = "0x88 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf088(&self) -> &Spipf088 {
        &self.spipf088
    }
    #[doc = "0x8c - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf08c(&self) -> &Spipf08c {
        &self.spipf08c
    }
    #[doc = "0x90 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf090(&self) -> &Spipf090 {
        &self.spipf090
    }
    #[doc = "0x94 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf094(&self) -> &Spipf094 {
        &self.spipf094
    }
    #[doc = "0x98 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf098(&self) -> &Spipf098 {
        &self.spipf098
    }
    #[doc = "0x9c - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf09c(&self) -> &Spipf09c {
        &self.spipf09c
    }
    #[doc = "0xa0 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0a0(&self) -> &Spipf0a0 {
        &self.spipf0a0
    }
    #[doc = "0xa4 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0a4(&self) -> &Spipf0a4 {
        &self.spipf0a4
    }
    #[doc = "0xa8 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0a8(&self) -> &Spipf0a8 {
        &self.spipf0a8
    }
    #[doc = "0xac - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0ac(&self) -> &Spipf0ac {
        &self.spipf0ac
    }
    #[doc = "0xb0 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0b0(&self) -> &Spipf0b0 {
        &self.spipf0b0
    }
    #[doc = "0xb4 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0b4(&self) -> &Spipf0b4 {
        &self.spipf0b4
    }
    #[doc = "0xb8 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0b8(&self) -> &Spipf0b8 {
        &self.spipf0b8
    }
    #[doc = "0xbc - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0bc(&self) -> &Spipf0bc {
        &self.spipf0bc
    }
    #[doc = "0xc0 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0c0(&self) -> &Spipf0c0 {
        &self.spipf0c0
    }
    #[doc = "0xc4 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0c4(&self) -> &Spipf0c4 {
        &self.spipf0c4
    }
    #[doc = "0xc8 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0c8(&self) -> &Spipf0c8 {
        &self.spipf0c8
    }
    #[doc = "0xcc - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0cc(&self) -> &Spipf0cc {
        &self.spipf0cc
    }
    #[doc = "0xd0 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0d0(&self) -> &Spipf0d0 {
        &self.spipf0d0
    }
    #[doc = "0xd4 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0d4(&self) -> &Spipf0d4 {
        &self.spipf0d4
    }
    #[doc = "0xd8 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0d8(&self) -> &Spipf0d8 {
        &self.spipf0d8
    }
    #[doc = "0xdc - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0dc(&self) -> &Spipf0dc {
        &self.spipf0dc
    }
    #[doc = "0xe0 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0e0(&self) -> &Spipf0e0 {
        &self.spipf0e0
    }
    #[doc = "0xe4 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0e4(&self) -> &Spipf0e4 {
        &self.spipf0e4
    }
    #[doc = "0xe8 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0e8(&self) -> &Spipf0e8 {
        &self.spipf0e8
    }
    #[doc = "0xec - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0ec(&self) -> &Spipf0ec {
        &self.spipf0ec
    }
    #[doc = "0xf0 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0f0(&self) -> &Spipf0f0 {
        &self.spipf0f0
    }
    #[doc = "0xf4 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0f4(&self) -> &Spipf0f4 {
        &self.spipf0f4
    }
    #[doc = "0xf8 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipf0f8(&self) -> &Spipf0f8 {
        &self.spipf0f8
    }
    #[doc = "0x100 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf100(&self) -> &Spipf100 {
        &self.spipf100
    }
    #[doc = "0x104 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf104(&self) -> &Spipf104 {
        &self.spipf104
    }
    #[doc = "0x108 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf108(&self) -> &Spipf108 {
        &self.spipf108
    }
    #[doc = "0x10c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf10c(&self) -> &Spipf10c {
        &self.spipf10c
    }
    #[doc = "0x110 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf110(&self) -> &Spipf110 {
        &self.spipf110
    }
    #[doc = "0x114 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf114(&self) -> &Spipf114 {
        &self.spipf114
    }
    #[doc = "0x118 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf118(&self) -> &Spipf118 {
        &self.spipf118
    }
    #[doc = "0x11c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf11c(&self) -> &Spipf11c {
        &self.spipf11c
    }
    #[doc = "0x120 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf120(&self) -> &Spipf120 {
        &self.spipf120
    }
    #[doc = "0x124 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf124(&self) -> &Spipf124 {
        &self.spipf124
    }
    #[doc = "0x128 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf128(&self) -> &Spipf128 {
        &self.spipf128
    }
    #[doc = "0x12c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf12c(&self) -> &Spipf12c {
        &self.spipf12c
    }
    #[doc = "0x130 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf130(&self) -> &Spipf130 {
        &self.spipf130
    }
    #[doc = "0x134 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf134(&self) -> &Spipf134 {
        &self.spipf134
    }
    #[doc = "0x138 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf138(&self) -> &Spipf138 {
        &self.spipf138
    }
    #[doc = "0x13c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf13c(&self) -> &Spipf13c {
        &self.spipf13c
    }
    #[doc = "0x140 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf140(&self) -> &Spipf140 {
        &self.spipf140
    }
    #[doc = "0x144 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf144(&self) -> &Spipf144 {
        &self.spipf144
    }
    #[doc = "0x148 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf148(&self) -> &Spipf148 {
        &self.spipf148
    }
    #[doc = "0x14c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf14c(&self) -> &Spipf14c {
        &self.spipf14c
    }
    #[doc = "0x150 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf150(&self) -> &Spipf150 {
        &self.spipf150
    }
    #[doc = "0x154 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf154(&self) -> &Spipf154 {
        &self.spipf154
    }
    #[doc = "0x158 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf158(&self) -> &Spipf158 {
        &self.spipf158
    }
    #[doc = "0x15c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf15c(&self) -> &Spipf15c {
        &self.spipf15c
    }
    #[doc = "0x160 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf160(&self) -> &Spipf160 {
        &self.spipf160
    }
    #[doc = "0x164 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf164(&self) -> &Spipf164 {
        &self.spipf164
    }
    #[doc = "0x168 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf168(&self) -> &Spipf168 {
        &self.spipf168
    }
    #[doc = "0x16c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf16c(&self) -> &Spipf16c {
        &self.spipf16c
    }
    #[doc = "0x170 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf170(&self) -> &Spipf170 {
        &self.spipf170
    }
    #[doc = "0x174 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf174(&self) -> &Spipf174 {
        &self.spipf174
    }
    #[doc = "0x178 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf178(&self) -> &Spipf178 {
        &self.spipf178
    }
    #[doc = "0x17c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf17c(&self) -> &Spipf17c {
        &self.spipf17c
    }
    #[doc = "0x180 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf180(&self) -> &Spipf180 {
        &self.spipf180
    }
    #[doc = "0x184 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf184(&self) -> &Spipf184 {
        &self.spipf184
    }
    #[doc = "0x188 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf188(&self) -> &Spipf188 {
        &self.spipf188
    }
    #[doc = "0x18c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf18c(&self) -> &Spipf18c {
        &self.spipf18c
    }
    #[doc = "0x190 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf190(&self) -> &Spipf190 {
        &self.spipf190
    }
    #[doc = "0x194 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf194(&self) -> &Spipf194 {
        &self.spipf194
    }
    #[doc = "0x198 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf198(&self) -> &Spipf198 {
        &self.spipf198
    }
    #[doc = "0x19c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf19c(&self) -> &Spipf19c {
        &self.spipf19c
    }
    #[doc = "0x1a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1a0(&self) -> &Spipf1a0 {
        &self.spipf1a0
    }
    #[doc = "0x1a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1a4(&self) -> &Spipf1a4 {
        &self.spipf1a4
    }
    #[doc = "0x1a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1a8(&self) -> &Spipf1a8 {
        &self.spipf1a8
    }
    #[doc = "0x1ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1ac(&self) -> &Spipf1ac {
        &self.spipf1ac
    }
    #[doc = "0x1b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1b0(&self) -> &Spipf1b0 {
        &self.spipf1b0
    }
    #[doc = "0x1b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1b4(&self) -> &Spipf1b4 {
        &self.spipf1b4
    }
    #[doc = "0x1b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1b8(&self) -> &Spipf1b8 {
        &self.spipf1b8
    }
    #[doc = "0x1bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1bc(&self) -> &Spipf1bc {
        &self.spipf1bc
    }
    #[doc = "0x1c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1c0(&self) -> &Spipf1c0 {
        &self.spipf1c0
    }
    #[doc = "0x1c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1c4(&self) -> &Spipf1c4 {
        &self.spipf1c4
    }
    #[doc = "0x1c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1c8(&self) -> &Spipf1c8 {
        &self.spipf1c8
    }
    #[doc = "0x1cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1cc(&self) -> &Spipf1cc {
        &self.spipf1cc
    }
    #[doc = "0x1d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1d0(&self) -> &Spipf1d0 {
        &self.spipf1d0
    }
    #[doc = "0x1d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1d4(&self) -> &Spipf1d4 {
        &self.spipf1d4
    }
    #[doc = "0x1d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1d8(&self) -> &Spipf1d8 {
        &self.spipf1d8
    }
    #[doc = "0x1dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1dc(&self) -> &Spipf1dc {
        &self.spipf1dc
    }
    #[doc = "0x1e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1e0(&self) -> &Spipf1e0 {
        &self.spipf1e0
    }
    #[doc = "0x1e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1e4(&self) -> &Spipf1e4 {
        &self.spipf1e4
    }
    #[doc = "0x1e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1e8(&self) -> &Spipf1e8 {
        &self.spipf1e8
    }
    #[doc = "0x1ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1ec(&self) -> &Spipf1ec {
        &self.spipf1ec
    }
    #[doc = "0x1f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1f0(&self) -> &Spipf1f0 {
        &self.spipf1f0
    }
    #[doc = "0x1f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1f4(&self) -> &Spipf1f4 {
        &self.spipf1f4
    }
    #[doc = "0x1f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1f8(&self) -> &Spipf1f8 {
        &self.spipf1f8
    }
    #[doc = "0x1fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf1fc(&self) -> &Spipf1fc {
        &self.spipf1fc
    }
    #[doc = "0x200 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf200(&self) -> &Spipf200 {
        &self.spipf200
    }
    #[doc = "0x204 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf204(&self) -> &Spipf204 {
        &self.spipf204
    }
    #[doc = "0x208 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf208(&self) -> &Spipf208 {
        &self.spipf208
    }
    #[doc = "0x20c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf20c(&self) -> &Spipf20c {
        &self.spipf20c
    }
    #[doc = "0x210 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf210(&self) -> &Spipf210 {
        &self.spipf210
    }
    #[doc = "0x214 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf214(&self) -> &Spipf214 {
        &self.spipf214
    }
    #[doc = "0x218 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf218(&self) -> &Spipf218 {
        &self.spipf218
    }
    #[doc = "0x21c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf21c(&self) -> &Spipf21c {
        &self.spipf21c
    }
    #[doc = "0x220 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf220(&self) -> &Spipf220 {
        &self.spipf220
    }
    #[doc = "0x224 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf224(&self) -> &Spipf224 {
        &self.spipf224
    }
    #[doc = "0x228 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf228(&self) -> &Spipf228 {
        &self.spipf228
    }
    #[doc = "0x22c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf22c(&self) -> &Spipf22c {
        &self.spipf22c
    }
    #[doc = "0x230 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf230(&self) -> &Spipf230 {
        &self.spipf230
    }
    #[doc = "0x234 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf234(&self) -> &Spipf234 {
        &self.spipf234
    }
    #[doc = "0x238 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf238(&self) -> &Spipf238 {
        &self.spipf238
    }
    #[doc = "0x23c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf23c(&self) -> &Spipf23c {
        &self.spipf23c
    }
    #[doc = "0x240 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf240(&self) -> &Spipf240 {
        &self.spipf240
    }
    #[doc = "0x244 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf244(&self) -> &Spipf244 {
        &self.spipf244
    }
    #[doc = "0x248 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf248(&self) -> &Spipf248 {
        &self.spipf248
    }
    #[doc = "0x24c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf24c(&self) -> &Spipf24c {
        &self.spipf24c
    }
    #[doc = "0x250 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf250(&self) -> &Spipf250 {
        &self.spipf250
    }
    #[doc = "0x254 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf254(&self) -> &Spipf254 {
        &self.spipf254
    }
    #[doc = "0x258 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf258(&self) -> &Spipf258 {
        &self.spipf258
    }
    #[doc = "0x25c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf25c(&self) -> &Spipf25c {
        &self.spipf25c
    }
    #[doc = "0x260 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf260(&self) -> &Spipf260 {
        &self.spipf260
    }
    #[doc = "0x264 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf264(&self) -> &Spipf264 {
        &self.spipf264
    }
    #[doc = "0x268 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf268(&self) -> &Spipf268 {
        &self.spipf268
    }
    #[doc = "0x26c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf26c(&self) -> &Spipf26c {
        &self.spipf26c
    }
    #[doc = "0x270 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf270(&self) -> &Spipf270 {
        &self.spipf270
    }
    #[doc = "0x274 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf274(&self) -> &Spipf274 {
        &self.spipf274
    }
    #[doc = "0x278 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf278(&self) -> &Spipf278 {
        &self.spipf278
    }
    #[doc = "0x27c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf27c(&self) -> &Spipf27c {
        &self.spipf27c
    }
    #[doc = "0x280 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf280(&self) -> &Spipf280 {
        &self.spipf280
    }
    #[doc = "0x284 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf284(&self) -> &Spipf284 {
        &self.spipf284
    }
    #[doc = "0x288 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf288(&self) -> &Spipf288 {
        &self.spipf288
    }
    #[doc = "0x28c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf28c(&self) -> &Spipf28c {
        &self.spipf28c
    }
    #[doc = "0x290 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf290(&self) -> &Spipf290 {
        &self.spipf290
    }
    #[doc = "0x294 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf294(&self) -> &Spipf294 {
        &self.spipf294
    }
    #[doc = "0x298 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf298(&self) -> &Spipf298 {
        &self.spipf298
    }
    #[doc = "0x29c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf29c(&self) -> &Spipf29c {
        &self.spipf29c
    }
    #[doc = "0x2a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2a0(&self) -> &Spipf2a0 {
        &self.spipf2a0
    }
    #[doc = "0x2a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2a4(&self) -> &Spipf2a4 {
        &self.spipf2a4
    }
    #[doc = "0x2a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2a8(&self) -> &Spipf2a8 {
        &self.spipf2a8
    }
    #[doc = "0x2ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2ac(&self) -> &Spipf2ac {
        &self.spipf2ac
    }
    #[doc = "0x2b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2b0(&self) -> &Spipf2b0 {
        &self.spipf2b0
    }
    #[doc = "0x2b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2b4(&self) -> &Spipf2b4 {
        &self.spipf2b4
    }
    #[doc = "0x2b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2b8(&self) -> &Spipf2b8 {
        &self.spipf2b8
    }
    #[doc = "0x2bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2bc(&self) -> &Spipf2bc {
        &self.spipf2bc
    }
    #[doc = "0x2c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2c0(&self) -> &Spipf2c0 {
        &self.spipf2c0
    }
    #[doc = "0x2c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2c4(&self) -> &Spipf2c4 {
        &self.spipf2c4
    }
    #[doc = "0x2c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2c8(&self) -> &Spipf2c8 {
        &self.spipf2c8
    }
    #[doc = "0x2cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2cc(&self) -> &Spipf2cc {
        &self.spipf2cc
    }
    #[doc = "0x2d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2d0(&self) -> &Spipf2d0 {
        &self.spipf2d0
    }
    #[doc = "0x2d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2d4(&self) -> &Spipf2d4 {
        &self.spipf2d4
    }
    #[doc = "0x2d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2d8(&self) -> &Spipf2d8 {
        &self.spipf2d8
    }
    #[doc = "0x2dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2dc(&self) -> &Spipf2dc {
        &self.spipf2dc
    }
    #[doc = "0x2e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2e0(&self) -> &Spipf2e0 {
        &self.spipf2e0
    }
    #[doc = "0x2e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2e4(&self) -> &Spipf2e4 {
        &self.spipf2e4
    }
    #[doc = "0x2e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2e8(&self) -> &Spipf2e8 {
        &self.spipf2e8
    }
    #[doc = "0x2ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2ec(&self) -> &Spipf2ec {
        &self.spipf2ec
    }
    #[doc = "0x2f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2f0(&self) -> &Spipf2f0 {
        &self.spipf2f0
    }
    #[doc = "0x2f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2f4(&self) -> &Spipf2f4 {
        &self.spipf2f4
    }
    #[doc = "0x2f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2f8(&self) -> &Spipf2f8 {
        &self.spipf2f8
    }
    #[doc = "0x2fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf2fc(&self) -> &Spipf2fc {
        &self.spipf2fc
    }
    #[doc = "0x300 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf300(&self) -> &Spipf300 {
        &self.spipf300
    }
    #[doc = "0x304 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf304(&self) -> &Spipf304 {
        &self.spipf304
    }
    #[doc = "0x308 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf308(&self) -> &Spipf308 {
        &self.spipf308
    }
    #[doc = "0x30c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf30c(&self) -> &Spipf30c {
        &self.spipf30c
    }
    #[doc = "0x310 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf310(&self) -> &Spipf310 {
        &self.spipf310
    }
    #[doc = "0x314 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf314(&self) -> &Spipf314 {
        &self.spipf314
    }
    #[doc = "0x318 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf318(&self) -> &Spipf318 {
        &self.spipf318
    }
    #[doc = "0x31c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf31c(&self) -> &Spipf31c {
        &self.spipf31c
    }
    #[doc = "0x320 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf320(&self) -> &Spipf320 {
        &self.spipf320
    }
    #[doc = "0x324 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf324(&self) -> &Spipf324 {
        &self.spipf324
    }
    #[doc = "0x328 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf328(&self) -> &Spipf328 {
        &self.spipf328
    }
    #[doc = "0x32c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf32c(&self) -> &Spipf32c {
        &self.spipf32c
    }
    #[doc = "0x330 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf330(&self) -> &Spipf330 {
        &self.spipf330
    }
    #[doc = "0x334 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf334(&self) -> &Spipf334 {
        &self.spipf334
    }
    #[doc = "0x338 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf338(&self) -> &Spipf338 {
        &self.spipf338
    }
    #[doc = "0x33c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf33c(&self) -> &Spipf33c {
        &self.spipf33c
    }
    #[doc = "0x340 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf340(&self) -> &Spipf340 {
        &self.spipf340
    }
    #[doc = "0x344 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf344(&self) -> &Spipf344 {
        &self.spipf344
    }
    #[doc = "0x348 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf348(&self) -> &Spipf348 {
        &self.spipf348
    }
    #[doc = "0x34c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf34c(&self) -> &Spipf34c {
        &self.spipf34c
    }
    #[doc = "0x350 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf350(&self) -> &Spipf350 {
        &self.spipf350
    }
    #[doc = "0x354 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf354(&self) -> &Spipf354 {
        &self.spipf354
    }
    #[doc = "0x358 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf358(&self) -> &Spipf358 {
        &self.spipf358
    }
    #[doc = "0x35c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf35c(&self) -> &Spipf35c {
        &self.spipf35c
    }
    #[doc = "0x360 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf360(&self) -> &Spipf360 {
        &self.spipf360
    }
    #[doc = "0x364 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf364(&self) -> &Spipf364 {
        &self.spipf364
    }
    #[doc = "0x368 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf368(&self) -> &Spipf368 {
        &self.spipf368
    }
    #[doc = "0x36c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf36c(&self) -> &Spipf36c {
        &self.spipf36c
    }
    #[doc = "0x370 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf370(&self) -> &Spipf370 {
        &self.spipf370
    }
    #[doc = "0x374 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf374(&self) -> &Spipf374 {
        &self.spipf374
    }
    #[doc = "0x378 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf378(&self) -> &Spipf378 {
        &self.spipf378
    }
    #[doc = "0x37c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf37c(&self) -> &Spipf37c {
        &self.spipf37c
    }
    #[doc = "0x380 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf380(&self) -> &Spipf380 {
        &self.spipf380
    }
    #[doc = "0x384 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf384(&self) -> &Spipf384 {
        &self.spipf384
    }
    #[doc = "0x388 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf388(&self) -> &Spipf388 {
        &self.spipf388
    }
    #[doc = "0x38c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf38c(&self) -> &Spipf38c {
        &self.spipf38c
    }
    #[doc = "0x390 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf390(&self) -> &Spipf390 {
        &self.spipf390
    }
    #[doc = "0x394 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf394(&self) -> &Spipf394 {
        &self.spipf394
    }
    #[doc = "0x398 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf398(&self) -> &Spipf398 {
        &self.spipf398
    }
    #[doc = "0x39c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf39c(&self) -> &Spipf39c {
        &self.spipf39c
    }
    #[doc = "0x3a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3a0(&self) -> &Spipf3a0 {
        &self.spipf3a0
    }
    #[doc = "0x3a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3a4(&self) -> &Spipf3a4 {
        &self.spipf3a4
    }
    #[doc = "0x3a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3a8(&self) -> &Spipf3a8 {
        &self.spipf3a8
    }
    #[doc = "0x3ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3ac(&self) -> &Spipf3ac {
        &self.spipf3ac
    }
    #[doc = "0x3b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3b0(&self) -> &Spipf3b0 {
        &self.spipf3b0
    }
    #[doc = "0x3b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3b4(&self) -> &Spipf3b4 {
        &self.spipf3b4
    }
    #[doc = "0x3b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3b8(&self) -> &Spipf3b8 {
        &self.spipf3b8
    }
    #[doc = "0x3bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3bc(&self) -> &Spipf3bc {
        &self.spipf3bc
    }
    #[doc = "0x3c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3c0(&self) -> &Spipf3c0 {
        &self.spipf3c0
    }
    #[doc = "0x3c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3c4(&self) -> &Spipf3c4 {
        &self.spipf3c4
    }
    #[doc = "0x3c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3c8(&self) -> &Spipf3c8 {
        &self.spipf3c8
    }
    #[doc = "0x3cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3cc(&self) -> &Spipf3cc {
        &self.spipf3cc
    }
    #[doc = "0x3d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3d0(&self) -> &Spipf3d0 {
        &self.spipf3d0
    }
    #[doc = "0x3d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3d4(&self) -> &Spipf3d4 {
        &self.spipf3d4
    }
    #[doc = "0x3d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3d8(&self) -> &Spipf3d8 {
        &self.spipf3d8
    }
    #[doc = "0x3dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3dc(&self) -> &Spipf3dc {
        &self.spipf3dc
    }
    #[doc = "0x3e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3e0(&self) -> &Spipf3e0 {
        &self.spipf3e0
    }
    #[doc = "0x3e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3e4(&self) -> &Spipf3e4 {
        &self.spipf3e4
    }
    #[doc = "0x3e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3e8(&self) -> &Spipf3e8 {
        &self.spipf3e8
    }
    #[doc = "0x3ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3ec(&self) -> &Spipf3ec {
        &self.spipf3ec
    }
    #[doc = "0x3f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3f0(&self) -> &Spipf3f0 {
        &self.spipf3f0
    }
    #[doc = "0x3f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3f4(&self) -> &Spipf3f4 {
        &self.spipf3f4
    }
    #[doc = "0x3f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3f8(&self) -> &Spipf3f8 {
        &self.spipf3f8
    }
    #[doc = "0x3fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf3fc(&self) -> &Spipf3fc {
        &self.spipf3fc
    }
    #[doc = "0x400 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf400(&self) -> &Spipf400 {
        &self.spipf400
    }
    #[doc = "0x404 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf404(&self) -> &Spipf404 {
        &self.spipf404
    }
    #[doc = "0x408 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf408(&self) -> &Spipf408 {
        &self.spipf408
    }
    #[doc = "0x40c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf40c(&self) -> &Spipf40c {
        &self.spipf40c
    }
    #[doc = "0x410 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf410(&self) -> &Spipf410 {
        &self.spipf410
    }
    #[doc = "0x414 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf414(&self) -> &Spipf414 {
        &self.spipf414
    }
    #[doc = "0x418 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf418(&self) -> &Spipf418 {
        &self.spipf418
    }
    #[doc = "0x41c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf41c(&self) -> &Spipf41c {
        &self.spipf41c
    }
    #[doc = "0x420 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf420(&self) -> &Spipf420 {
        &self.spipf420
    }
    #[doc = "0x424 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf424(&self) -> &Spipf424 {
        &self.spipf424
    }
    #[doc = "0x428 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf428(&self) -> &Spipf428 {
        &self.spipf428
    }
    #[doc = "0x42c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf42c(&self) -> &Spipf42c {
        &self.spipf42c
    }
    #[doc = "0x430 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf430(&self) -> &Spipf430 {
        &self.spipf430
    }
    #[doc = "0x434 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf434(&self) -> &Spipf434 {
        &self.spipf434
    }
    #[doc = "0x438 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf438(&self) -> &Spipf438 {
        &self.spipf438
    }
    #[doc = "0x43c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf43c(&self) -> &Spipf43c {
        &self.spipf43c
    }
    #[doc = "0x440 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf440(&self) -> &Spipf440 {
        &self.spipf440
    }
    #[doc = "0x444 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf444(&self) -> &Spipf444 {
        &self.spipf444
    }
    #[doc = "0x448 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf448(&self) -> &Spipf448 {
        &self.spipf448
    }
    #[doc = "0x44c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf44c(&self) -> &Spipf44c {
        &self.spipf44c
    }
    #[doc = "0x450 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf450(&self) -> &Spipf450 {
        &self.spipf450
    }
    #[doc = "0x454 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf454(&self) -> &Spipf454 {
        &self.spipf454
    }
    #[doc = "0x458 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf458(&self) -> &Spipf458 {
        &self.spipf458
    }
    #[doc = "0x45c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf45c(&self) -> &Spipf45c {
        &self.spipf45c
    }
    #[doc = "0x460 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf460(&self) -> &Spipf460 {
        &self.spipf460
    }
    #[doc = "0x464 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf464(&self) -> &Spipf464 {
        &self.spipf464
    }
    #[doc = "0x468 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf468(&self) -> &Spipf468 {
        &self.spipf468
    }
    #[doc = "0x46c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf46c(&self) -> &Spipf46c {
        &self.spipf46c
    }
    #[doc = "0x470 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf470(&self) -> &Spipf470 {
        &self.spipf470
    }
    #[doc = "0x474 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf474(&self) -> &Spipf474 {
        &self.spipf474
    }
    #[doc = "0x478 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf478(&self) -> &Spipf478 {
        &self.spipf478
    }
    #[doc = "0x47c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf47c(&self) -> &Spipf47c {
        &self.spipf47c
    }
    #[doc = "0x480 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf480(&self) -> &Spipf480 {
        &self.spipf480
    }
    #[doc = "0x484 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf484(&self) -> &Spipf484 {
        &self.spipf484
    }
    #[doc = "0x488 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf488(&self) -> &Spipf488 {
        &self.spipf488
    }
    #[doc = "0x48c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf48c(&self) -> &Spipf48c {
        &self.spipf48c
    }
    #[doc = "0x490 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf490(&self) -> &Spipf490 {
        &self.spipf490
    }
    #[doc = "0x494 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf494(&self) -> &Spipf494 {
        &self.spipf494
    }
    #[doc = "0x498 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf498(&self) -> &Spipf498 {
        &self.spipf498
    }
    #[doc = "0x49c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf49c(&self) -> &Spipf49c {
        &self.spipf49c
    }
    #[doc = "0x4a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4a0(&self) -> &Spipf4a0 {
        &self.spipf4a0
    }
    #[doc = "0x4a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4a4(&self) -> &Spipf4a4 {
        &self.spipf4a4
    }
    #[doc = "0x4a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4a8(&self) -> &Spipf4a8 {
        &self.spipf4a8
    }
    #[doc = "0x4ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4ac(&self) -> &Spipf4ac {
        &self.spipf4ac
    }
    #[doc = "0x4b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4b0(&self) -> &Spipf4b0 {
        &self.spipf4b0
    }
    #[doc = "0x4b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4b4(&self) -> &Spipf4b4 {
        &self.spipf4b4
    }
    #[doc = "0x4b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4b8(&self) -> &Spipf4b8 {
        &self.spipf4b8
    }
    #[doc = "0x4bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4bc(&self) -> &Spipf4bc {
        &self.spipf4bc
    }
    #[doc = "0x4c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4c0(&self) -> &Spipf4c0 {
        &self.spipf4c0
    }
    #[doc = "0x4c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4c4(&self) -> &Spipf4c4 {
        &self.spipf4c4
    }
    #[doc = "0x4c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4c8(&self) -> &Spipf4c8 {
        &self.spipf4c8
    }
    #[doc = "0x4cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4cc(&self) -> &Spipf4cc {
        &self.spipf4cc
    }
    #[doc = "0x4d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4d0(&self) -> &Spipf4d0 {
        &self.spipf4d0
    }
    #[doc = "0x4d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4d4(&self) -> &Spipf4d4 {
        &self.spipf4d4
    }
    #[doc = "0x4d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4d8(&self) -> &Spipf4d8 {
        &self.spipf4d8
    }
    #[doc = "0x4dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4dc(&self) -> &Spipf4dc {
        &self.spipf4dc
    }
    #[doc = "0x4e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4e0(&self) -> &Spipf4e0 {
        &self.spipf4e0
    }
    #[doc = "0x4e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4e4(&self) -> &Spipf4e4 {
        &self.spipf4e4
    }
    #[doc = "0x4e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4e8(&self) -> &Spipf4e8 {
        &self.spipf4e8
    }
    #[doc = "0x4ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4ec(&self) -> &Spipf4ec {
        &self.spipf4ec
    }
    #[doc = "0x4f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4f0(&self) -> &Spipf4f0 {
        &self.spipf4f0
    }
    #[doc = "0x4f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4f4(&self) -> &Spipf4f4 {
        &self.spipf4f4
    }
    #[doc = "0x4f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4f8(&self) -> &Spipf4f8 {
        &self.spipf4f8
    }
    #[doc = "0x4fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf4fc(&self) -> &Spipf4fc {
        &self.spipf4fc
    }
    #[doc = "0x500 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf500(&self) -> &Spipf500 {
        &self.spipf500
    }
    #[doc = "0x504 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf504(&self) -> &Spipf504 {
        &self.spipf504
    }
    #[doc = "0x508 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf508(&self) -> &Spipf508 {
        &self.spipf508
    }
    #[doc = "0x50c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf50c(&self) -> &Spipf50c {
        &self.spipf50c
    }
    #[doc = "0x510 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf510(&self) -> &Spipf510 {
        &self.spipf510
    }
    #[doc = "0x514 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf514(&self) -> &Spipf514 {
        &self.spipf514
    }
    #[doc = "0x518 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf518(&self) -> &Spipf518 {
        &self.spipf518
    }
    #[doc = "0x51c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf51c(&self) -> &Spipf51c {
        &self.spipf51c
    }
    #[doc = "0x520 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf520(&self) -> &Spipf520 {
        &self.spipf520
    }
    #[doc = "0x524 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf524(&self) -> &Spipf524 {
        &self.spipf524
    }
    #[doc = "0x528 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf528(&self) -> &Spipf528 {
        &self.spipf528
    }
    #[doc = "0x52c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf52c(&self) -> &Spipf52c {
        &self.spipf52c
    }
    #[doc = "0x530 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf530(&self) -> &Spipf530 {
        &self.spipf530
    }
    #[doc = "0x534 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf534(&self) -> &Spipf534 {
        &self.spipf534
    }
    #[doc = "0x538 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf538(&self) -> &Spipf538 {
        &self.spipf538
    }
    #[doc = "0x53c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf53c(&self) -> &Spipf53c {
        &self.spipf53c
    }
    #[doc = "0x540 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf540(&self) -> &Spipf540 {
        &self.spipf540
    }
    #[doc = "0x544 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf544(&self) -> &Spipf544 {
        &self.spipf544
    }
    #[doc = "0x548 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf548(&self) -> &Spipf548 {
        &self.spipf548
    }
    #[doc = "0x54c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf54c(&self) -> &Spipf54c {
        &self.spipf54c
    }
    #[doc = "0x550 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf550(&self) -> &Spipf550 {
        &self.spipf550
    }
    #[doc = "0x554 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf554(&self) -> &Spipf554 {
        &self.spipf554
    }
    #[doc = "0x558 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf558(&self) -> &Spipf558 {
        &self.spipf558
    }
    #[doc = "0x55c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf55c(&self) -> &Spipf55c {
        &self.spipf55c
    }
    #[doc = "0x560 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf560(&self) -> &Spipf560 {
        &self.spipf560
    }
    #[doc = "0x564 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf564(&self) -> &Spipf564 {
        &self.spipf564
    }
    #[doc = "0x568 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf568(&self) -> &Spipf568 {
        &self.spipf568
    }
    #[doc = "0x56c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf56c(&self) -> &Spipf56c {
        &self.spipf56c
    }
    #[doc = "0x570 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf570(&self) -> &Spipf570 {
        &self.spipf570
    }
    #[doc = "0x574 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf574(&self) -> &Spipf574 {
        &self.spipf574
    }
    #[doc = "0x578 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf578(&self) -> &Spipf578 {
        &self.spipf578
    }
    #[doc = "0x57c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf57c(&self) -> &Spipf57c {
        &self.spipf57c
    }
    #[doc = "0x580 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf580(&self) -> &Spipf580 {
        &self.spipf580
    }
    #[doc = "0x584 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf584(&self) -> &Spipf584 {
        &self.spipf584
    }
    #[doc = "0x588 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf588(&self) -> &Spipf588 {
        &self.spipf588
    }
    #[doc = "0x58c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf58c(&self) -> &Spipf58c {
        &self.spipf58c
    }
    #[doc = "0x590 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf590(&self) -> &Spipf590 {
        &self.spipf590
    }
    #[doc = "0x594 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf594(&self) -> &Spipf594 {
        &self.spipf594
    }
    #[doc = "0x598 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf598(&self) -> &Spipf598 {
        &self.spipf598
    }
    #[doc = "0x59c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf59c(&self) -> &Spipf59c {
        &self.spipf59c
    }
    #[doc = "0x5a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5a0(&self) -> &Spipf5a0 {
        &self.spipf5a0
    }
    #[doc = "0x5a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5a4(&self) -> &Spipf5a4 {
        &self.spipf5a4
    }
    #[doc = "0x5a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5a8(&self) -> &Spipf5a8 {
        &self.spipf5a8
    }
    #[doc = "0x5ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5ac(&self) -> &Spipf5ac {
        &self.spipf5ac
    }
    #[doc = "0x5b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5b0(&self) -> &Spipf5b0 {
        &self.spipf5b0
    }
    #[doc = "0x5b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5b4(&self) -> &Spipf5b4 {
        &self.spipf5b4
    }
    #[doc = "0x5b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5b8(&self) -> &Spipf5b8 {
        &self.spipf5b8
    }
    #[doc = "0x5bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5bc(&self) -> &Spipf5bc {
        &self.spipf5bc
    }
    #[doc = "0x5c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5c0(&self) -> &Spipf5c0 {
        &self.spipf5c0
    }
    #[doc = "0x5c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5c4(&self) -> &Spipf5c4 {
        &self.spipf5c4
    }
    #[doc = "0x5c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5c8(&self) -> &Spipf5c8 {
        &self.spipf5c8
    }
    #[doc = "0x5cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5cc(&self) -> &Spipf5cc {
        &self.spipf5cc
    }
    #[doc = "0x5d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5d0(&self) -> &Spipf5d0 {
        &self.spipf5d0
    }
    #[doc = "0x5d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5d4(&self) -> &Spipf5d4 {
        &self.spipf5d4
    }
    #[doc = "0x5d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5d8(&self) -> &Spipf5d8 {
        &self.spipf5d8
    }
    #[doc = "0x5dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5dc(&self) -> &Spipf5dc {
        &self.spipf5dc
    }
    #[doc = "0x5e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5e0(&self) -> &Spipf5e0 {
        &self.spipf5e0
    }
    #[doc = "0x5e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5e4(&self) -> &Spipf5e4 {
        &self.spipf5e4
    }
    #[doc = "0x5e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5e8(&self) -> &Spipf5e8 {
        &self.spipf5e8
    }
    #[doc = "0x5ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5ec(&self) -> &Spipf5ec {
        &self.spipf5ec
    }
    #[doc = "0x5f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5f0(&self) -> &Spipf5f0 {
        &self.spipf5f0
    }
    #[doc = "0x5f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5f4(&self) -> &Spipf5f4 {
        &self.spipf5f4
    }
    #[doc = "0x5f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5f8(&self) -> &Spipf5f8 {
        &self.spipf5f8
    }
    #[doc = "0x5fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf5fc(&self) -> &Spipf5fc {
        &self.spipf5fc
    }
    #[doc = "0x600 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf600(&self) -> &Spipf600 {
        &self.spipf600
    }
    #[doc = "0x604 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf604(&self) -> &Spipf604 {
        &self.spipf604
    }
    #[doc = "0x608 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf608(&self) -> &Spipf608 {
        &self.spipf608
    }
    #[doc = "0x60c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf60c(&self) -> &Spipf60c {
        &self.spipf60c
    }
    #[doc = "0x610 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf610(&self) -> &Spipf610 {
        &self.spipf610
    }
    #[doc = "0x614 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf614(&self) -> &Spipf614 {
        &self.spipf614
    }
    #[doc = "0x618 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf618(&self) -> &Spipf618 {
        &self.spipf618
    }
    #[doc = "0x61c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf61c(&self) -> &Spipf61c {
        &self.spipf61c
    }
    #[doc = "0x620 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf620(&self) -> &Spipf620 {
        &self.spipf620
    }
    #[doc = "0x624 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf624(&self) -> &Spipf624 {
        &self.spipf624
    }
    #[doc = "0x628 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf628(&self) -> &Spipf628 {
        &self.spipf628
    }
    #[doc = "0x62c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf62c(&self) -> &Spipf62c {
        &self.spipf62c
    }
    #[doc = "0x630 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf630(&self) -> &Spipf630 {
        &self.spipf630
    }
    #[doc = "0x634 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf634(&self) -> &Spipf634 {
        &self.spipf634
    }
    #[doc = "0x638 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf638(&self) -> &Spipf638 {
        &self.spipf638
    }
    #[doc = "0x63c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf63c(&self) -> &Spipf63c {
        &self.spipf63c
    }
    #[doc = "0x640 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf640(&self) -> &Spipf640 {
        &self.spipf640
    }
    #[doc = "0x644 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf644(&self) -> &Spipf644 {
        &self.spipf644
    }
    #[doc = "0x648 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf648(&self) -> &Spipf648 {
        &self.spipf648
    }
    #[doc = "0x64c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf64c(&self) -> &Spipf64c {
        &self.spipf64c
    }
    #[doc = "0x650 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf650(&self) -> &Spipf650 {
        &self.spipf650
    }
    #[doc = "0x654 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf654(&self) -> &Spipf654 {
        &self.spipf654
    }
    #[doc = "0x658 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf658(&self) -> &Spipf658 {
        &self.spipf658
    }
    #[doc = "0x65c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf65c(&self) -> &Spipf65c {
        &self.spipf65c
    }
    #[doc = "0x660 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf660(&self) -> &Spipf660 {
        &self.spipf660
    }
    #[doc = "0x664 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf664(&self) -> &Spipf664 {
        &self.spipf664
    }
    #[doc = "0x668 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf668(&self) -> &Spipf668 {
        &self.spipf668
    }
    #[doc = "0x66c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf66c(&self) -> &Spipf66c {
        &self.spipf66c
    }
    #[doc = "0x670 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf670(&self) -> &Spipf670 {
        &self.spipf670
    }
    #[doc = "0x674 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf674(&self) -> &Spipf674 {
        &self.spipf674
    }
    #[doc = "0x678 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf678(&self) -> &Spipf678 {
        &self.spipf678
    }
    #[doc = "0x67c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf67c(&self) -> &Spipf67c {
        &self.spipf67c
    }
    #[doc = "0x680 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf680(&self) -> &Spipf680 {
        &self.spipf680
    }
    #[doc = "0x684 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf684(&self) -> &Spipf684 {
        &self.spipf684
    }
    #[doc = "0x688 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf688(&self) -> &Spipf688 {
        &self.spipf688
    }
    #[doc = "0x68c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf68c(&self) -> &Spipf68c {
        &self.spipf68c
    }
    #[doc = "0x690 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf690(&self) -> &Spipf690 {
        &self.spipf690
    }
    #[doc = "0x694 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf694(&self) -> &Spipf694 {
        &self.spipf694
    }
    #[doc = "0x698 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf698(&self) -> &Spipf698 {
        &self.spipf698
    }
    #[doc = "0x69c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf69c(&self) -> &Spipf69c {
        &self.spipf69c
    }
    #[doc = "0x6a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6a0(&self) -> &Spipf6a0 {
        &self.spipf6a0
    }
    #[doc = "0x6a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6a4(&self) -> &Spipf6a4 {
        &self.spipf6a4
    }
    #[doc = "0x6a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6a8(&self) -> &Spipf6a8 {
        &self.spipf6a8
    }
    #[doc = "0x6ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6ac(&self) -> &Spipf6ac {
        &self.spipf6ac
    }
    #[doc = "0x6b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6b0(&self) -> &Spipf6b0 {
        &self.spipf6b0
    }
    #[doc = "0x6b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6b4(&self) -> &Spipf6b4 {
        &self.spipf6b4
    }
    #[doc = "0x6b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6b8(&self) -> &Spipf6b8 {
        &self.spipf6b8
    }
    #[doc = "0x6bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6bc(&self) -> &Spipf6bc {
        &self.spipf6bc
    }
    #[doc = "0x6c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6c0(&self) -> &Spipf6c0 {
        &self.spipf6c0
    }
    #[doc = "0x6c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6c4(&self) -> &Spipf6c4 {
        &self.spipf6c4
    }
    #[doc = "0x6c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6c8(&self) -> &Spipf6c8 {
        &self.spipf6c8
    }
    #[doc = "0x6cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6cc(&self) -> &Spipf6cc {
        &self.spipf6cc
    }
    #[doc = "0x6d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6d0(&self) -> &Spipf6d0 {
        &self.spipf6d0
    }
    #[doc = "0x6d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6d4(&self) -> &Spipf6d4 {
        &self.spipf6d4
    }
    #[doc = "0x6d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6d8(&self) -> &Spipf6d8 {
        &self.spipf6d8
    }
    #[doc = "0x6dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6dc(&self) -> &Spipf6dc {
        &self.spipf6dc
    }
    #[doc = "0x6e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6e0(&self) -> &Spipf6e0 {
        &self.spipf6e0
    }
    #[doc = "0x6e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6e4(&self) -> &Spipf6e4 {
        &self.spipf6e4
    }
    #[doc = "0x6e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6e8(&self) -> &Spipf6e8 {
        &self.spipf6e8
    }
    #[doc = "0x6ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6ec(&self) -> &Spipf6ec {
        &self.spipf6ec
    }
    #[doc = "0x6f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6f0(&self) -> &Spipf6f0 {
        &self.spipf6f0
    }
    #[doc = "0x6f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6f4(&self) -> &Spipf6f4 {
        &self.spipf6f4
    }
    #[doc = "0x6f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6f8(&self) -> &Spipf6f8 {
        &self.spipf6f8
    }
    #[doc = "0x6fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf6fc(&self) -> &Spipf6fc {
        &self.spipf6fc
    }
    #[doc = "0x700 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf700(&self) -> &Spipf700 {
        &self.spipf700
    }
    #[doc = "0x704 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf704(&self) -> &Spipf704 {
        &self.spipf704
    }
    #[doc = "0x708 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf708(&self) -> &Spipf708 {
        &self.spipf708
    }
    #[doc = "0x70c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf70c(&self) -> &Spipf70c {
        &self.spipf70c
    }
    #[doc = "0x710 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf710(&self) -> &Spipf710 {
        &self.spipf710
    }
    #[doc = "0x714 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf714(&self) -> &Spipf714 {
        &self.spipf714
    }
    #[doc = "0x718 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf718(&self) -> &Spipf718 {
        &self.spipf718
    }
    #[doc = "0x71c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf71c(&self) -> &Spipf71c {
        &self.spipf71c
    }
    #[doc = "0x720 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf720(&self) -> &Spipf720 {
        &self.spipf720
    }
    #[doc = "0x724 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf724(&self) -> &Spipf724 {
        &self.spipf724
    }
    #[doc = "0x728 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf728(&self) -> &Spipf728 {
        &self.spipf728
    }
    #[doc = "0x72c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf72c(&self) -> &Spipf72c {
        &self.spipf72c
    }
    #[doc = "0x730 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf730(&self) -> &Spipf730 {
        &self.spipf730
    }
    #[doc = "0x734 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf734(&self) -> &Spipf734 {
        &self.spipf734
    }
    #[doc = "0x738 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf738(&self) -> &Spipf738 {
        &self.spipf738
    }
    #[doc = "0x73c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf73c(&self) -> &Spipf73c {
        &self.spipf73c
    }
    #[doc = "0x740 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf740(&self) -> &Spipf740 {
        &self.spipf740
    }
    #[doc = "0x744 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf744(&self) -> &Spipf744 {
        &self.spipf744
    }
    #[doc = "0x748 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf748(&self) -> &Spipf748 {
        &self.spipf748
    }
    #[doc = "0x74c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf74c(&self) -> &Spipf74c {
        &self.spipf74c
    }
    #[doc = "0x750 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf750(&self) -> &Spipf750 {
        &self.spipf750
    }
    #[doc = "0x754 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf754(&self) -> &Spipf754 {
        &self.spipf754
    }
    #[doc = "0x758 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf758(&self) -> &Spipf758 {
        &self.spipf758
    }
    #[doc = "0x75c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf75c(&self) -> &Spipf75c {
        &self.spipf75c
    }
    #[doc = "0x760 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf760(&self) -> &Spipf760 {
        &self.spipf760
    }
    #[doc = "0x764 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf764(&self) -> &Spipf764 {
        &self.spipf764
    }
    #[doc = "0x768 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf768(&self) -> &Spipf768 {
        &self.spipf768
    }
    #[doc = "0x76c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf76c(&self) -> &Spipf76c {
        &self.spipf76c
    }
    #[doc = "0x770 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf770(&self) -> &Spipf770 {
        &self.spipf770
    }
    #[doc = "0x774 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf774(&self) -> &Spipf774 {
        &self.spipf774
    }
    #[doc = "0x778 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf778(&self) -> &Spipf778 {
        &self.spipf778
    }
    #[doc = "0x77c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf77c(&self) -> &Spipf77c {
        &self.spipf77c
    }
    #[doc = "0x780 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf780(&self) -> &Spipf780 {
        &self.spipf780
    }
    #[doc = "0x784 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf784(&self) -> &Spipf784 {
        &self.spipf784
    }
    #[doc = "0x788 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf788(&self) -> &Spipf788 {
        &self.spipf788
    }
    #[doc = "0x78c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf78c(&self) -> &Spipf78c {
        &self.spipf78c
    }
    #[doc = "0x790 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf790(&self) -> &Spipf790 {
        &self.spipf790
    }
    #[doc = "0x794 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf794(&self) -> &Spipf794 {
        &self.spipf794
    }
    #[doc = "0x798 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf798(&self) -> &Spipf798 {
        &self.spipf798
    }
    #[doc = "0x79c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf79c(&self) -> &Spipf79c {
        &self.spipf79c
    }
    #[doc = "0x7a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7a0(&self) -> &Spipf7a0 {
        &self.spipf7a0
    }
    #[doc = "0x7a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7a4(&self) -> &Spipf7a4 {
        &self.spipf7a4
    }
    #[doc = "0x7a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7a8(&self) -> &Spipf7a8 {
        &self.spipf7a8
    }
    #[doc = "0x7ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7ac(&self) -> &Spipf7ac {
        &self.spipf7ac
    }
    #[doc = "0x7b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7b0(&self) -> &Spipf7b0 {
        &self.spipf7b0
    }
    #[doc = "0x7b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7b4(&self) -> &Spipf7b4 {
        &self.spipf7b4
    }
    #[doc = "0x7b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7b8(&self) -> &Spipf7b8 {
        &self.spipf7b8
    }
    #[doc = "0x7bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7bc(&self) -> &Spipf7bc {
        &self.spipf7bc
    }
    #[doc = "0x7c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7c0(&self) -> &Spipf7c0 {
        &self.spipf7c0
    }
    #[doc = "0x7c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7c4(&self) -> &Spipf7c4 {
        &self.spipf7c4
    }
    #[doc = "0x7c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7c8(&self) -> &Spipf7c8 {
        &self.spipf7c8
    }
    #[doc = "0x7cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7cc(&self) -> &Spipf7cc {
        &self.spipf7cc
    }
    #[doc = "0x7d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7d0(&self) -> &Spipf7d0 {
        &self.spipf7d0
    }
    #[doc = "0x7d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7d4(&self) -> &Spipf7d4 {
        &self.spipf7d4
    }
    #[doc = "0x7d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7d8(&self) -> &Spipf7d8 {
        &self.spipf7d8
    }
    #[doc = "0x7dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7dc(&self) -> &Spipf7dc {
        &self.spipf7dc
    }
    #[doc = "0x7e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7e0(&self) -> &Spipf7e0 {
        &self.spipf7e0
    }
    #[doc = "0x7e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7e4(&self) -> &Spipf7e4 {
        &self.spipf7e4
    }
    #[doc = "0x7e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7e8(&self) -> &Spipf7e8 {
        &self.spipf7e8
    }
    #[doc = "0x7ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7ec(&self) -> &Spipf7ec {
        &self.spipf7ec
    }
    #[doc = "0x7f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7f0(&self) -> &Spipf7f0 {
        &self.spipf7f0
    }
    #[doc = "0x7f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7f4(&self) -> &Spipf7f4 {
        &self.spipf7f4
    }
    #[doc = "0x7f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7f8(&self) -> &Spipf7f8 {
        &self.spipf7f8
    }
    #[doc = "0x7fc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf7fc(&self) -> &Spipf7fc {
        &self.spipf7fc
    }
    #[doc = "0x800 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf800(&self) -> &Spipf800 {
        &self.spipf800
    }
    #[doc = "0x804 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf804(&self) -> &Spipf804 {
        &self.spipf804
    }
    #[doc = "0x808 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf808(&self) -> &Spipf808 {
        &self.spipf808
    }
    #[doc = "0x80c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf80c(&self) -> &Spipf80c {
        &self.spipf80c
    }
    #[doc = "0x810 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf810(&self) -> &Spipf810 {
        &self.spipf810
    }
    #[doc = "0x814 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf814(&self) -> &Spipf814 {
        &self.spipf814
    }
    #[doc = "0x818 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf818(&self) -> &Spipf818 {
        &self.spipf818
    }
    #[doc = "0x81c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf81c(&self) -> &Spipf81c {
        &self.spipf81c
    }
    #[doc = "0x820 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf820(&self) -> &Spipf820 {
        &self.spipf820
    }
    #[doc = "0x824 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf824(&self) -> &Spipf824 {
        &self.spipf824
    }
    #[doc = "0x828 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf828(&self) -> &Spipf828 {
        &self.spipf828
    }
    #[doc = "0x82c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf82c(&self) -> &Spipf82c {
        &self.spipf82c
    }
    #[doc = "0x830 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf830(&self) -> &Spipf830 {
        &self.spipf830
    }
    #[doc = "0x834 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf834(&self) -> &Spipf834 {
        &self.spipf834
    }
    #[doc = "0x838 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf838(&self) -> &Spipf838 {
        &self.spipf838
    }
    #[doc = "0x83c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf83c(&self) -> &Spipf83c {
        &self.spipf83c
    }
    #[doc = "0x840 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf840(&self) -> &Spipf840 {
        &self.spipf840
    }
    #[doc = "0x844 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf844(&self) -> &Spipf844 {
        &self.spipf844
    }
    #[doc = "0x848 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf848(&self) -> &Spipf848 {
        &self.spipf848
    }
    #[doc = "0x84c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf84c(&self) -> &Spipf84c {
        &self.spipf84c
    }
    #[doc = "0x850 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf850(&self) -> &Spipf850 {
        &self.spipf850
    }
    #[doc = "0x854 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf854(&self) -> &Spipf854 {
        &self.spipf854
    }
    #[doc = "0x858 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf858(&self) -> &Spipf858 {
        &self.spipf858
    }
    #[doc = "0x85c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf85c(&self) -> &Spipf85c {
        &self.spipf85c
    }
    #[doc = "0x860 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf860(&self) -> &Spipf860 {
        &self.spipf860
    }
    #[doc = "0x864 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf864(&self) -> &Spipf864 {
        &self.spipf864
    }
    #[doc = "0x868 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf868(&self) -> &Spipf868 {
        &self.spipf868
    }
    #[doc = "0x86c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf86c(&self) -> &Spipf86c {
        &self.spipf86c
    }
    #[doc = "0x870 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf870(&self) -> &Spipf870 {
        &self.spipf870
    }
    #[doc = "0x874 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf874(&self) -> &Spipf874 {
        &self.spipf874
    }
    #[doc = "0x878 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf878(&self) -> &Spipf878 {
        &self.spipf878
    }
    #[doc = "0x87c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf87c(&self) -> &Spipf87c {
        &self.spipf87c
    }
    #[doc = "0x880 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf880(&self) -> &Spipf880 {
        &self.spipf880
    }
    #[doc = "0x884 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf884(&self) -> &Spipf884 {
        &self.spipf884
    }
    #[doc = "0x888 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf888(&self) -> &Spipf888 {
        &self.spipf888
    }
    #[doc = "0x88c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf88c(&self) -> &Spipf88c {
        &self.spipf88c
    }
    #[doc = "0x890 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf890(&self) -> &Spipf890 {
        &self.spipf890
    }
    #[doc = "0x894 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf894(&self) -> &Spipf894 {
        &self.spipf894
    }
    #[doc = "0x898 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf898(&self) -> &Spipf898 {
        &self.spipf898
    }
    #[doc = "0x89c - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf89c(&self) -> &Spipf89c {
        &self.spipf89c
    }
    #[doc = "0x8a0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8a0(&self) -> &Spipf8a0 {
        &self.spipf8a0
    }
    #[doc = "0x8a4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8a4(&self) -> &Spipf8a4 {
        &self.spipf8a4
    }
    #[doc = "0x8a8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8a8(&self) -> &Spipf8a8 {
        &self.spipf8a8
    }
    #[doc = "0x8ac - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8ac(&self) -> &Spipf8ac {
        &self.spipf8ac
    }
    #[doc = "0x8b0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8b0(&self) -> &Spipf8b0 {
        &self.spipf8b0
    }
    #[doc = "0x8b4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8b4(&self) -> &Spipf8b4 {
        &self.spipf8b4
    }
    #[doc = "0x8b8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8b8(&self) -> &Spipf8b8 {
        &self.spipf8b8
    }
    #[doc = "0x8bc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8bc(&self) -> &Spipf8bc {
        &self.spipf8bc
    }
    #[doc = "0x8c0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8c0(&self) -> &Spipf8c0 {
        &self.spipf8c0
    }
    #[doc = "0x8c4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8c4(&self) -> &Spipf8c4 {
        &self.spipf8c4
    }
    #[doc = "0x8c8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8c8(&self) -> &Spipf8c8 {
        &self.spipf8c8
    }
    #[doc = "0x8cc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8cc(&self) -> &Spipf8cc {
        &self.spipf8cc
    }
    #[doc = "0x8d0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8d0(&self) -> &Spipf8d0 {
        &self.spipf8d0
    }
    #[doc = "0x8d4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8d4(&self) -> &Spipf8d4 {
        &self.spipf8d4
    }
    #[doc = "0x8d8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8d8(&self) -> &Spipf8d8 {
        &self.spipf8d8
    }
    #[doc = "0x8dc - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8dc(&self) -> &Spipf8dc {
        &self.spipf8dc
    }
    #[doc = "0x8e0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8e0(&self) -> &Spipf8e0 {
        &self.spipf8e0
    }
    #[doc = "0x8e4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8e4(&self) -> &Spipf8e4 {
        &self.spipf8e4
    }
    #[doc = "0x8e8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8e8(&self) -> &Spipf8e8 {
        &self.spipf8e8
    }
    #[doc = "0x8ec - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8ec(&self) -> &Spipf8ec {
        &self.spipf8ec
    }
    #[doc = "0x8f0 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8f0(&self) -> &Spipf8f0 {
        &self.spipf8f0
    }
    #[doc = "0x8f4 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8f4(&self) -> &Spipf8f4 {
        &self.spipf8f4
    }
    #[doc = "0x8f8 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipf8f8(&self) -> &Spipf8f8 {
        &self.spipf8f8
    }
}
#[doc = "SPIPF000 (rw) register accessor: Engine Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf000`] module"]
#[doc(alias = "SPIPF000")]
pub type Spipf000 = crate::Reg<spipf000::Spipf000Spec>;
#[doc = "Engine Control Register"]
pub mod spipf000;
#[doc = "SPIPF004 (rw) register accessor: Interrupt Enable and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf004`] module"]
#[doc(alias = "SPIPF004")]
pub type Spipf004 = crate::Reg<spipf004::Spipf004Spec>;
#[doc = "Interrupt Enable and Status Register"]
pub mod spipf004;
#[doc = "SPIPF008 (rw) register accessor: EAR and Over Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf008`] module"]
#[doc(alias = "SPIPF008")]
pub type Spipf008 = crate::Reg<spipf008::Spipf008Spec>;
#[doc = "EAR and Over Speed Register"]
pub mod spipf008;
#[doc = "SPIPF00C (rw) register accessor: Block FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf00c`] module"]
#[doc(alias = "SPIPF00C")]
pub type Spipf00c = crate::Reg<spipf00c::Spipf00cSpec>;
#[doc = "Block FIFO Data Register"]
pub mod spipf00c;
#[doc = "SPIPF010 (rw) register accessor: Block Log DMA Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf010`] module"]
#[doc(alias = "SPIPF010")]
pub type Spipf010 = crate::Reg<spipf010::Spipf010Spec>;
#[doc = "Block Log DMA Base Address"]
pub mod spipf010;
#[doc = "SPIPF014 (rw) register accessor: Block Log DMA Size\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf014`] module"]
#[doc(alias = "SPIPF014")]
pub type Spipf014 = crate::Reg<spipf014::Spipf014Spec>;
#[doc = "Block Log DMA Size"]
pub mod spipf014;
#[doc = "SPIPF018 (rw) register accessor: Block Log DMA Write Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf018`] module"]
#[doc(alias = "SPIPF018")]
pub type Spipf018 = crate::Reg<spipf018::Spipf018Spec>;
#[doc = "Block Log DMA Write Pointer"]
pub mod spipf018;
#[doc = "SPIPF07C (rw) register accessor: Write Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf07c`] module"]
#[doc(alias = "SPIPF07C")]
pub type Spipf07c = crate::Reg<spipf07c::Spipf07cSpec>;
#[doc = "Write Disable Register"]
pub mod spipf07c;
#[doc = "SPIPF080 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf080`] module"]
#[doc(alias = "SPIPF080")]
pub type Spipf080 = crate::Reg<spipf080::Spipf080Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf080;
#[doc = "SPIPF084 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf084`] module"]
#[doc(alias = "SPIPF084")]
pub type Spipf084 = crate::Reg<spipf084::Spipf084Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf084;
#[doc = "SPIPF088 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf088`] module"]
#[doc(alias = "SPIPF088")]
pub type Spipf088 = crate::Reg<spipf088::Spipf088Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf088;
#[doc = "SPIPF08C (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf08c`] module"]
#[doc(alias = "SPIPF08C")]
pub type Spipf08c = crate::Reg<spipf08c::Spipf08cSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf08c;
#[doc = "SPIPF090 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf090`] module"]
#[doc(alias = "SPIPF090")]
pub type Spipf090 = crate::Reg<spipf090::Spipf090Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf090;
#[doc = "SPIPF094 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf094`] module"]
#[doc(alias = "SPIPF094")]
pub type Spipf094 = crate::Reg<spipf094::Spipf094Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf094;
#[doc = "SPIPF098 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf098`] module"]
#[doc(alias = "SPIPF098")]
pub type Spipf098 = crate::Reg<spipf098::Spipf098Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf098;
#[doc = "SPIPF09C (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf09c`] module"]
#[doc(alias = "SPIPF09C")]
pub type Spipf09c = crate::Reg<spipf09c::Spipf09cSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf09c;
#[doc = "SPIPF0A0 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0a0`] module"]
#[doc(alias = "SPIPF0A0")]
pub type Spipf0a0 = crate::Reg<spipf0a0::Spipf0a0Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0a0;
#[doc = "SPIPF0A4 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0a4`] module"]
#[doc(alias = "SPIPF0A4")]
pub type Spipf0a4 = crate::Reg<spipf0a4::Spipf0a4Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0a4;
#[doc = "SPIPF0A8 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0a8`] module"]
#[doc(alias = "SPIPF0A8")]
pub type Spipf0a8 = crate::Reg<spipf0a8::Spipf0a8Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0a8;
#[doc = "SPIPF0AC (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0ac`] module"]
#[doc(alias = "SPIPF0AC")]
pub type Spipf0ac = crate::Reg<spipf0ac::Spipf0acSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0ac;
#[doc = "SPIPF0B0 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0b0`] module"]
#[doc(alias = "SPIPF0B0")]
pub type Spipf0b0 = crate::Reg<spipf0b0::Spipf0b0Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0b0;
#[doc = "SPIPF0B4 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0b4`] module"]
#[doc(alias = "SPIPF0B4")]
pub type Spipf0b4 = crate::Reg<spipf0b4::Spipf0b4Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0b4;
#[doc = "SPIPF0B8 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0b8`] module"]
#[doc(alias = "SPIPF0B8")]
pub type Spipf0b8 = crate::Reg<spipf0b8::Spipf0b8Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0b8;
#[doc = "SPIPF0BC (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0bc`] module"]
#[doc(alias = "SPIPF0BC")]
pub type Spipf0bc = crate::Reg<spipf0bc::Spipf0bcSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0bc;
#[doc = "SPIPF0C0 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0c0`] module"]
#[doc(alias = "SPIPF0C0")]
pub type Spipf0c0 = crate::Reg<spipf0c0::Spipf0c0Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0c0;
#[doc = "SPIPF0C4 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0c4`] module"]
#[doc(alias = "SPIPF0C4")]
pub type Spipf0c4 = crate::Reg<spipf0c4::Spipf0c4Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0c4;
#[doc = "SPIPF0C8 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0c8`] module"]
#[doc(alias = "SPIPF0C8")]
pub type Spipf0c8 = crate::Reg<spipf0c8::Spipf0c8Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0c8;
#[doc = "SPIPF0CC (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0cc`] module"]
#[doc(alias = "SPIPF0CC")]
pub type Spipf0cc = crate::Reg<spipf0cc::Spipf0ccSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0cc;
#[doc = "SPIPF0D0 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0d0`] module"]
#[doc(alias = "SPIPF0D0")]
pub type Spipf0d0 = crate::Reg<spipf0d0::Spipf0d0Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0d0;
#[doc = "SPIPF0D4 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0d4`] module"]
#[doc(alias = "SPIPF0D4")]
pub type Spipf0d4 = crate::Reg<spipf0d4::Spipf0d4Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0d4;
#[doc = "SPIPF0D8 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0d8`] module"]
#[doc(alias = "SPIPF0D8")]
pub type Spipf0d8 = crate::Reg<spipf0d8::Spipf0d8Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0d8;
#[doc = "SPIPF0DC (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0dc`] module"]
#[doc(alias = "SPIPF0DC")]
pub type Spipf0dc = crate::Reg<spipf0dc::Spipf0dcSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0dc;
#[doc = "SPIPF0E0 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0e0`] module"]
#[doc(alias = "SPIPF0E0")]
pub type Spipf0e0 = crate::Reg<spipf0e0::Spipf0e0Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0e0;
#[doc = "SPIPF0E4 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0e4`] module"]
#[doc(alias = "SPIPF0E4")]
pub type Spipf0e4 = crate::Reg<spipf0e4::Spipf0e4Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0e4;
#[doc = "SPIPF0E8 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0e8`] module"]
#[doc(alias = "SPIPF0E8")]
pub type Spipf0e8 = crate::Reg<spipf0e8::Spipf0e8Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0e8;
#[doc = "SPIPF0EC (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0ec`] module"]
#[doc(alias = "SPIPF0EC")]
pub type Spipf0ec = crate::Reg<spipf0ec::Spipf0ecSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0ec;
#[doc = "SPIPF0F0 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0f0`] module"]
#[doc(alias = "SPIPF0F0")]
pub type Spipf0f0 = crate::Reg<spipf0f0::Spipf0f0Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0f0;
#[doc = "SPIPF0F4 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0f4`] module"]
#[doc(alias = "SPIPF0F4")]
pub type Spipf0f4 = crate::Reg<spipf0f4::Spipf0f4Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0f4;
#[doc = "SPIPF0F8 (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf0f8`] module"]
#[doc(alias = "SPIPF0F8")]
pub type Spipf0f8 = crate::Reg<spipf0f8::Spipf0f8Spec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipf0f8;
#[doc = "SPIPF100 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf100`] module"]
#[doc(alias = "SPIPF100")]
pub type Spipf100 = crate::Reg<spipf100::Spipf100Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf100;
#[doc = "SPIPF104 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf104`] module"]
#[doc(alias = "SPIPF104")]
pub type Spipf104 = crate::Reg<spipf104::Spipf104Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf104;
#[doc = "SPIPF108 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf108`] module"]
#[doc(alias = "SPIPF108")]
pub type Spipf108 = crate::Reg<spipf108::Spipf108Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf108;
#[doc = "SPIPF10C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf10c`] module"]
#[doc(alias = "SPIPF10C")]
pub type Spipf10c = crate::Reg<spipf10c::Spipf10cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf10c;
#[doc = "SPIPF110 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf110`] module"]
#[doc(alias = "SPIPF110")]
pub type Spipf110 = crate::Reg<spipf110::Spipf110Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf110;
#[doc = "SPIPF114 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf114`] module"]
#[doc(alias = "SPIPF114")]
pub type Spipf114 = crate::Reg<spipf114::Spipf114Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf114;
#[doc = "SPIPF118 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf118`] module"]
#[doc(alias = "SPIPF118")]
pub type Spipf118 = crate::Reg<spipf118::Spipf118Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf118;
#[doc = "SPIPF11C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf11c`] module"]
#[doc(alias = "SPIPF11C")]
pub type Spipf11c = crate::Reg<spipf11c::Spipf11cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf11c;
#[doc = "SPIPF120 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf120`] module"]
#[doc(alias = "SPIPF120")]
pub type Spipf120 = crate::Reg<spipf120::Spipf120Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf120;
#[doc = "SPIPF124 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf124`] module"]
#[doc(alias = "SPIPF124")]
pub type Spipf124 = crate::Reg<spipf124::Spipf124Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf124;
#[doc = "SPIPF128 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf128`] module"]
#[doc(alias = "SPIPF128")]
pub type Spipf128 = crate::Reg<spipf128::Spipf128Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf128;
#[doc = "SPIPF12C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf12c`] module"]
#[doc(alias = "SPIPF12C")]
pub type Spipf12c = crate::Reg<spipf12c::Spipf12cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf12c;
#[doc = "SPIPF130 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf130`] module"]
#[doc(alias = "SPIPF130")]
pub type Spipf130 = crate::Reg<spipf130::Spipf130Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf130;
#[doc = "SPIPF134 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf134`] module"]
#[doc(alias = "SPIPF134")]
pub type Spipf134 = crate::Reg<spipf134::Spipf134Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf134;
#[doc = "SPIPF138 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf138`] module"]
#[doc(alias = "SPIPF138")]
pub type Spipf138 = crate::Reg<spipf138::Spipf138Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf138;
#[doc = "SPIPF13C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf13c`] module"]
#[doc(alias = "SPIPF13C")]
pub type Spipf13c = crate::Reg<spipf13c::Spipf13cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf13c;
#[doc = "SPIPF140 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf140`] module"]
#[doc(alias = "SPIPF140")]
pub type Spipf140 = crate::Reg<spipf140::Spipf140Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf140;
#[doc = "SPIPF144 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf144`] module"]
#[doc(alias = "SPIPF144")]
pub type Spipf144 = crate::Reg<spipf144::Spipf144Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf144;
#[doc = "SPIPF148 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf148`] module"]
#[doc(alias = "SPIPF148")]
pub type Spipf148 = crate::Reg<spipf148::Spipf148Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf148;
#[doc = "SPIPF14C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf14c`] module"]
#[doc(alias = "SPIPF14C")]
pub type Spipf14c = crate::Reg<spipf14c::Spipf14cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf14c;
#[doc = "SPIPF150 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf150`] module"]
#[doc(alias = "SPIPF150")]
pub type Spipf150 = crate::Reg<spipf150::Spipf150Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf150;
#[doc = "SPIPF154 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf154`] module"]
#[doc(alias = "SPIPF154")]
pub type Spipf154 = crate::Reg<spipf154::Spipf154Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf154;
#[doc = "SPIPF158 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf158`] module"]
#[doc(alias = "SPIPF158")]
pub type Spipf158 = crate::Reg<spipf158::Spipf158Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf158;
#[doc = "SPIPF15C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf15c`] module"]
#[doc(alias = "SPIPF15C")]
pub type Spipf15c = crate::Reg<spipf15c::Spipf15cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf15c;
#[doc = "SPIPF160 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf160`] module"]
#[doc(alias = "SPIPF160")]
pub type Spipf160 = crate::Reg<spipf160::Spipf160Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf160;
#[doc = "SPIPF164 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf164`] module"]
#[doc(alias = "SPIPF164")]
pub type Spipf164 = crate::Reg<spipf164::Spipf164Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf164;
#[doc = "SPIPF168 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf168`] module"]
#[doc(alias = "SPIPF168")]
pub type Spipf168 = crate::Reg<spipf168::Spipf168Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf168;
#[doc = "SPIPF16C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf16c`] module"]
#[doc(alias = "SPIPF16C")]
pub type Spipf16c = crate::Reg<spipf16c::Spipf16cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf16c;
#[doc = "SPIPF170 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf170`] module"]
#[doc(alias = "SPIPF170")]
pub type Spipf170 = crate::Reg<spipf170::Spipf170Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf170;
#[doc = "SPIPF174 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf174`] module"]
#[doc(alias = "SPIPF174")]
pub type Spipf174 = crate::Reg<spipf174::Spipf174Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf174;
#[doc = "SPIPF178 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf178`] module"]
#[doc(alias = "SPIPF178")]
pub type Spipf178 = crate::Reg<spipf178::Spipf178Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf178;
#[doc = "SPIPF17C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf17c`] module"]
#[doc(alias = "SPIPF17C")]
pub type Spipf17c = crate::Reg<spipf17c::Spipf17cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf17c;
#[doc = "SPIPF180 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf180`] module"]
#[doc(alias = "SPIPF180")]
pub type Spipf180 = crate::Reg<spipf180::Spipf180Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf180;
#[doc = "SPIPF184 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf184`] module"]
#[doc(alias = "SPIPF184")]
pub type Spipf184 = crate::Reg<spipf184::Spipf184Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf184;
#[doc = "SPIPF188 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf188`] module"]
#[doc(alias = "SPIPF188")]
pub type Spipf188 = crate::Reg<spipf188::Spipf188Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf188;
#[doc = "SPIPF18C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf18c`] module"]
#[doc(alias = "SPIPF18C")]
pub type Spipf18c = crate::Reg<spipf18c::Spipf18cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf18c;
#[doc = "SPIPF190 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf190`] module"]
#[doc(alias = "SPIPF190")]
pub type Spipf190 = crate::Reg<spipf190::Spipf190Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf190;
#[doc = "SPIPF194 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf194`] module"]
#[doc(alias = "SPIPF194")]
pub type Spipf194 = crate::Reg<spipf194::Spipf194Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf194;
#[doc = "SPIPF198 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf198`] module"]
#[doc(alias = "SPIPF198")]
pub type Spipf198 = crate::Reg<spipf198::Spipf198Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf198;
#[doc = "SPIPF19C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf19c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf19c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf19c`] module"]
#[doc(alias = "SPIPF19C")]
pub type Spipf19c = crate::Reg<spipf19c::Spipf19cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf19c;
#[doc = "SPIPF1A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1a0`] module"]
#[doc(alias = "SPIPF1A0")]
pub type Spipf1a0 = crate::Reg<spipf1a0::Spipf1a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1a0;
#[doc = "SPIPF1A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1a4`] module"]
#[doc(alias = "SPIPF1A4")]
pub type Spipf1a4 = crate::Reg<spipf1a4::Spipf1a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1a4;
#[doc = "SPIPF1A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1a8`] module"]
#[doc(alias = "SPIPF1A8")]
pub type Spipf1a8 = crate::Reg<spipf1a8::Spipf1a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1a8;
#[doc = "SPIPF1AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1ac`] module"]
#[doc(alias = "SPIPF1AC")]
pub type Spipf1ac = crate::Reg<spipf1ac::Spipf1acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1ac;
#[doc = "SPIPF1B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1b0`] module"]
#[doc(alias = "SPIPF1B0")]
pub type Spipf1b0 = crate::Reg<spipf1b0::Spipf1b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1b0;
#[doc = "SPIPF1B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1b4`] module"]
#[doc(alias = "SPIPF1B4")]
pub type Spipf1b4 = crate::Reg<spipf1b4::Spipf1b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1b4;
#[doc = "SPIPF1B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1b8`] module"]
#[doc(alias = "SPIPF1B8")]
pub type Spipf1b8 = crate::Reg<spipf1b8::Spipf1b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1b8;
#[doc = "SPIPF1BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1bc`] module"]
#[doc(alias = "SPIPF1BC")]
pub type Spipf1bc = crate::Reg<spipf1bc::Spipf1bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1bc;
#[doc = "SPIPF1C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1c0`] module"]
#[doc(alias = "SPIPF1C0")]
pub type Spipf1c0 = crate::Reg<spipf1c0::Spipf1c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1c0;
#[doc = "SPIPF1C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1c4`] module"]
#[doc(alias = "SPIPF1C4")]
pub type Spipf1c4 = crate::Reg<spipf1c4::Spipf1c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1c4;
#[doc = "SPIPF1C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1c8`] module"]
#[doc(alias = "SPIPF1C8")]
pub type Spipf1c8 = crate::Reg<spipf1c8::Spipf1c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1c8;
#[doc = "SPIPF1CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1cc`] module"]
#[doc(alias = "SPIPF1CC")]
pub type Spipf1cc = crate::Reg<spipf1cc::Spipf1ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1cc;
#[doc = "SPIPF1D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1d0`] module"]
#[doc(alias = "SPIPF1D0")]
pub type Spipf1d0 = crate::Reg<spipf1d0::Spipf1d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1d0;
#[doc = "SPIPF1D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1d4`] module"]
#[doc(alias = "SPIPF1D4")]
pub type Spipf1d4 = crate::Reg<spipf1d4::Spipf1d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1d4;
#[doc = "SPIPF1D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1d8`] module"]
#[doc(alias = "SPIPF1D8")]
pub type Spipf1d8 = crate::Reg<spipf1d8::Spipf1d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1d8;
#[doc = "SPIPF1DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1dc`] module"]
#[doc(alias = "SPIPF1DC")]
pub type Spipf1dc = crate::Reg<spipf1dc::Spipf1dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1dc;
#[doc = "SPIPF1E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1e0`] module"]
#[doc(alias = "SPIPF1E0")]
pub type Spipf1e0 = crate::Reg<spipf1e0::Spipf1e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1e0;
#[doc = "SPIPF1E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1e4`] module"]
#[doc(alias = "SPIPF1E4")]
pub type Spipf1e4 = crate::Reg<spipf1e4::Spipf1e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1e4;
#[doc = "SPIPF1E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1e8`] module"]
#[doc(alias = "SPIPF1E8")]
pub type Spipf1e8 = crate::Reg<spipf1e8::Spipf1e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1e8;
#[doc = "SPIPF1EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1ec`] module"]
#[doc(alias = "SPIPF1EC")]
pub type Spipf1ec = crate::Reg<spipf1ec::Spipf1ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1ec;
#[doc = "SPIPF1F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1f0`] module"]
#[doc(alias = "SPIPF1F0")]
pub type Spipf1f0 = crate::Reg<spipf1f0::Spipf1f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1f0;
#[doc = "SPIPF1F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1f4`] module"]
#[doc(alias = "SPIPF1F4")]
pub type Spipf1f4 = crate::Reg<spipf1f4::Spipf1f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1f4;
#[doc = "SPIPF1F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1f8`] module"]
#[doc(alias = "SPIPF1F8")]
pub type Spipf1f8 = crate::Reg<spipf1f8::Spipf1f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1f8;
#[doc = "SPIPF1FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1fc`] module"]
#[doc(alias = "SPIPF1FC")]
pub type Spipf1fc = crate::Reg<spipf1fc::Spipf1fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf1fc;
#[doc = "SPIPF200 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf200`] module"]
#[doc(alias = "SPIPF200")]
pub type Spipf200 = crate::Reg<spipf200::Spipf200Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf200;
#[doc = "SPIPF204 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf204`] module"]
#[doc(alias = "SPIPF204")]
pub type Spipf204 = crate::Reg<spipf204::Spipf204Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf204;
#[doc = "SPIPF208 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf208`] module"]
#[doc(alias = "SPIPF208")]
pub type Spipf208 = crate::Reg<spipf208::Spipf208Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf208;
#[doc = "SPIPF20C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf20c`] module"]
#[doc(alias = "SPIPF20C")]
pub type Spipf20c = crate::Reg<spipf20c::Spipf20cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf20c;
#[doc = "SPIPF210 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf210`] module"]
#[doc(alias = "SPIPF210")]
pub type Spipf210 = crate::Reg<spipf210::Spipf210Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf210;
#[doc = "SPIPF214 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf214`] module"]
#[doc(alias = "SPIPF214")]
pub type Spipf214 = crate::Reg<spipf214::Spipf214Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf214;
#[doc = "SPIPF218 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf218`] module"]
#[doc(alias = "SPIPF218")]
pub type Spipf218 = crate::Reg<spipf218::Spipf218Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf218;
#[doc = "SPIPF21C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf21c`] module"]
#[doc(alias = "SPIPF21C")]
pub type Spipf21c = crate::Reg<spipf21c::Spipf21cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf21c;
#[doc = "SPIPF220 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf220`] module"]
#[doc(alias = "SPIPF220")]
pub type Spipf220 = crate::Reg<spipf220::Spipf220Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf220;
#[doc = "SPIPF224 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf224`] module"]
#[doc(alias = "SPIPF224")]
pub type Spipf224 = crate::Reg<spipf224::Spipf224Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf224;
#[doc = "SPIPF228 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf228`] module"]
#[doc(alias = "SPIPF228")]
pub type Spipf228 = crate::Reg<spipf228::Spipf228Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf228;
#[doc = "SPIPF22C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf22c`] module"]
#[doc(alias = "SPIPF22C")]
pub type Spipf22c = crate::Reg<spipf22c::Spipf22cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf22c;
#[doc = "SPIPF230 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf230`] module"]
#[doc(alias = "SPIPF230")]
pub type Spipf230 = crate::Reg<spipf230::Spipf230Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf230;
#[doc = "SPIPF234 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf234`] module"]
#[doc(alias = "SPIPF234")]
pub type Spipf234 = crate::Reg<spipf234::Spipf234Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf234;
#[doc = "SPIPF238 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf238`] module"]
#[doc(alias = "SPIPF238")]
pub type Spipf238 = crate::Reg<spipf238::Spipf238Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf238;
#[doc = "SPIPF23C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf23c`] module"]
#[doc(alias = "SPIPF23C")]
pub type Spipf23c = crate::Reg<spipf23c::Spipf23cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf23c;
#[doc = "SPIPF240 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf240`] module"]
#[doc(alias = "SPIPF240")]
pub type Spipf240 = crate::Reg<spipf240::Spipf240Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf240;
#[doc = "SPIPF244 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf244`] module"]
#[doc(alias = "SPIPF244")]
pub type Spipf244 = crate::Reg<spipf244::Spipf244Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf244;
#[doc = "SPIPF248 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf248`] module"]
#[doc(alias = "SPIPF248")]
pub type Spipf248 = crate::Reg<spipf248::Spipf248Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf248;
#[doc = "SPIPF24C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf24c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf24c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf24c`] module"]
#[doc(alias = "SPIPF24C")]
pub type Spipf24c = crate::Reg<spipf24c::Spipf24cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf24c;
#[doc = "SPIPF250 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf250`] module"]
#[doc(alias = "SPIPF250")]
pub type Spipf250 = crate::Reg<spipf250::Spipf250Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf250;
#[doc = "SPIPF254 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf254`] module"]
#[doc(alias = "SPIPF254")]
pub type Spipf254 = crate::Reg<spipf254::Spipf254Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf254;
#[doc = "SPIPF258 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf258`] module"]
#[doc(alias = "SPIPF258")]
pub type Spipf258 = crate::Reg<spipf258::Spipf258Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf258;
#[doc = "SPIPF25C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf25c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf25c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf25c`] module"]
#[doc(alias = "SPIPF25C")]
pub type Spipf25c = crate::Reg<spipf25c::Spipf25cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf25c;
#[doc = "SPIPF260 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf260`] module"]
#[doc(alias = "SPIPF260")]
pub type Spipf260 = crate::Reg<spipf260::Spipf260Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf260;
#[doc = "SPIPF264 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf264`] module"]
#[doc(alias = "SPIPF264")]
pub type Spipf264 = crate::Reg<spipf264::Spipf264Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf264;
#[doc = "SPIPF268 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf268`] module"]
#[doc(alias = "SPIPF268")]
pub type Spipf268 = crate::Reg<spipf268::Spipf268Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf268;
#[doc = "SPIPF26C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf26c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf26c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf26c`] module"]
#[doc(alias = "SPIPF26C")]
pub type Spipf26c = crate::Reg<spipf26c::Spipf26cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf26c;
#[doc = "SPIPF270 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf270`] module"]
#[doc(alias = "SPIPF270")]
pub type Spipf270 = crate::Reg<spipf270::Spipf270Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf270;
#[doc = "SPIPF274 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf274`] module"]
#[doc(alias = "SPIPF274")]
pub type Spipf274 = crate::Reg<spipf274::Spipf274Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf274;
#[doc = "SPIPF278 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf278`] module"]
#[doc(alias = "SPIPF278")]
pub type Spipf278 = crate::Reg<spipf278::Spipf278Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf278;
#[doc = "SPIPF27C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf27c`] module"]
#[doc(alias = "SPIPF27C")]
pub type Spipf27c = crate::Reg<spipf27c::Spipf27cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf27c;
#[doc = "SPIPF280 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf280`] module"]
#[doc(alias = "SPIPF280")]
pub type Spipf280 = crate::Reg<spipf280::Spipf280Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf280;
#[doc = "SPIPF284 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf284`] module"]
#[doc(alias = "SPIPF284")]
pub type Spipf284 = crate::Reg<spipf284::Spipf284Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf284;
#[doc = "SPIPF288 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf288`] module"]
#[doc(alias = "SPIPF288")]
pub type Spipf288 = crate::Reg<spipf288::Spipf288Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf288;
#[doc = "SPIPF28C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf28c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf28c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf28c`] module"]
#[doc(alias = "SPIPF28C")]
pub type Spipf28c = crate::Reg<spipf28c::Spipf28cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf28c;
#[doc = "SPIPF290 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf290`] module"]
#[doc(alias = "SPIPF290")]
pub type Spipf290 = crate::Reg<spipf290::Spipf290Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf290;
#[doc = "SPIPF294 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf294::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf294::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf294`] module"]
#[doc(alias = "SPIPF294")]
pub type Spipf294 = crate::Reg<spipf294::Spipf294Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf294;
#[doc = "SPIPF298 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf298`] module"]
#[doc(alias = "SPIPF298")]
pub type Spipf298 = crate::Reg<spipf298::Spipf298Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf298;
#[doc = "SPIPF29C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf29c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf29c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf29c`] module"]
#[doc(alias = "SPIPF29C")]
pub type Spipf29c = crate::Reg<spipf29c::Spipf29cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf29c;
#[doc = "SPIPF2A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2a0`] module"]
#[doc(alias = "SPIPF2A0")]
pub type Spipf2a0 = crate::Reg<spipf2a0::Spipf2a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2a0;
#[doc = "SPIPF2A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2a4`] module"]
#[doc(alias = "SPIPF2A4")]
pub type Spipf2a4 = crate::Reg<spipf2a4::Spipf2a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2a4;
#[doc = "SPIPF2A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2a8`] module"]
#[doc(alias = "SPIPF2A8")]
pub type Spipf2a8 = crate::Reg<spipf2a8::Spipf2a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2a8;
#[doc = "SPIPF2AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2ac`] module"]
#[doc(alias = "SPIPF2AC")]
pub type Spipf2ac = crate::Reg<spipf2ac::Spipf2acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2ac;
#[doc = "SPIPF2B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2b0`] module"]
#[doc(alias = "SPIPF2B0")]
pub type Spipf2b0 = crate::Reg<spipf2b0::Spipf2b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2b0;
#[doc = "SPIPF2B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2b4`] module"]
#[doc(alias = "SPIPF2B4")]
pub type Spipf2b4 = crate::Reg<spipf2b4::Spipf2b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2b4;
#[doc = "SPIPF2B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2b8`] module"]
#[doc(alias = "SPIPF2B8")]
pub type Spipf2b8 = crate::Reg<spipf2b8::Spipf2b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2b8;
#[doc = "SPIPF2BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2bc`] module"]
#[doc(alias = "SPIPF2BC")]
pub type Spipf2bc = crate::Reg<spipf2bc::Spipf2bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2bc;
#[doc = "SPIPF2C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2c0`] module"]
#[doc(alias = "SPIPF2C0")]
pub type Spipf2c0 = crate::Reg<spipf2c0::Spipf2c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2c0;
#[doc = "SPIPF2C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2c4`] module"]
#[doc(alias = "SPIPF2C4")]
pub type Spipf2c4 = crate::Reg<spipf2c4::Spipf2c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2c4;
#[doc = "SPIPF2C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2c8`] module"]
#[doc(alias = "SPIPF2C8")]
pub type Spipf2c8 = crate::Reg<spipf2c8::Spipf2c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2c8;
#[doc = "SPIPF2CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2cc`] module"]
#[doc(alias = "SPIPF2CC")]
pub type Spipf2cc = crate::Reg<spipf2cc::Spipf2ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2cc;
#[doc = "SPIPF2D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2d0`] module"]
#[doc(alias = "SPIPF2D0")]
pub type Spipf2d0 = crate::Reg<spipf2d0::Spipf2d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2d0;
#[doc = "SPIPF2D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2d4`] module"]
#[doc(alias = "SPIPF2D4")]
pub type Spipf2d4 = crate::Reg<spipf2d4::Spipf2d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2d4;
#[doc = "SPIPF2D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2d8`] module"]
#[doc(alias = "SPIPF2D8")]
pub type Spipf2d8 = crate::Reg<spipf2d8::Spipf2d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2d8;
#[doc = "SPIPF2DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2dc`] module"]
#[doc(alias = "SPIPF2DC")]
pub type Spipf2dc = crate::Reg<spipf2dc::Spipf2dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2dc;
#[doc = "SPIPF2E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2e0`] module"]
#[doc(alias = "SPIPF2E0")]
pub type Spipf2e0 = crate::Reg<spipf2e0::Spipf2e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2e0;
#[doc = "SPIPF2E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2e4`] module"]
#[doc(alias = "SPIPF2E4")]
pub type Spipf2e4 = crate::Reg<spipf2e4::Spipf2e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2e4;
#[doc = "SPIPF2E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2e8`] module"]
#[doc(alias = "SPIPF2E8")]
pub type Spipf2e8 = crate::Reg<spipf2e8::Spipf2e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2e8;
#[doc = "SPIPF2EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2ec`] module"]
#[doc(alias = "SPIPF2EC")]
pub type Spipf2ec = crate::Reg<spipf2ec::Spipf2ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2ec;
#[doc = "SPIPF2F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2f0`] module"]
#[doc(alias = "SPIPF2F0")]
pub type Spipf2f0 = crate::Reg<spipf2f0::Spipf2f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2f0;
#[doc = "SPIPF2F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2f4`] module"]
#[doc(alias = "SPIPF2F4")]
pub type Spipf2f4 = crate::Reg<spipf2f4::Spipf2f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2f4;
#[doc = "SPIPF2F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2f8`] module"]
#[doc(alias = "SPIPF2F8")]
pub type Spipf2f8 = crate::Reg<spipf2f8::Spipf2f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2f8;
#[doc = "SPIPF2FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2fc`] module"]
#[doc(alias = "SPIPF2FC")]
pub type Spipf2fc = crate::Reg<spipf2fc::Spipf2fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf2fc;
#[doc = "SPIPF300 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf300`] module"]
#[doc(alias = "SPIPF300")]
pub type Spipf300 = crate::Reg<spipf300::Spipf300Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf300;
#[doc = "SPIPF304 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf304`] module"]
#[doc(alias = "SPIPF304")]
pub type Spipf304 = crate::Reg<spipf304::Spipf304Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf304;
#[doc = "SPIPF308 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf308::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf308::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf308`] module"]
#[doc(alias = "SPIPF308")]
pub type Spipf308 = crate::Reg<spipf308::Spipf308Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf308;
#[doc = "SPIPF30C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf30c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf30c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf30c`] module"]
#[doc(alias = "SPIPF30C")]
pub type Spipf30c = crate::Reg<spipf30c::Spipf30cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf30c;
#[doc = "SPIPF310 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf310`] module"]
#[doc(alias = "SPIPF310")]
pub type Spipf310 = crate::Reg<spipf310::Spipf310Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf310;
#[doc = "SPIPF314 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf314`] module"]
#[doc(alias = "SPIPF314")]
pub type Spipf314 = crate::Reg<spipf314::Spipf314Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf314;
#[doc = "SPIPF318 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf318`] module"]
#[doc(alias = "SPIPF318")]
pub type Spipf318 = crate::Reg<spipf318::Spipf318Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf318;
#[doc = "SPIPF31C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf31c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf31c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf31c`] module"]
#[doc(alias = "SPIPF31C")]
pub type Spipf31c = crate::Reg<spipf31c::Spipf31cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf31c;
#[doc = "SPIPF320 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf320::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf320::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf320`] module"]
#[doc(alias = "SPIPF320")]
pub type Spipf320 = crate::Reg<spipf320::Spipf320Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf320;
#[doc = "SPIPF324 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf324::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf324::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf324`] module"]
#[doc(alias = "SPIPF324")]
pub type Spipf324 = crate::Reg<spipf324::Spipf324Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf324;
#[doc = "SPIPF328 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf328::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf328::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf328`] module"]
#[doc(alias = "SPIPF328")]
pub type Spipf328 = crate::Reg<spipf328::Spipf328Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf328;
#[doc = "SPIPF32C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf32c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf32c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf32c`] module"]
#[doc(alias = "SPIPF32C")]
pub type Spipf32c = crate::Reg<spipf32c::Spipf32cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf32c;
#[doc = "SPIPF330 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf330::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf330::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf330`] module"]
#[doc(alias = "SPIPF330")]
pub type Spipf330 = crate::Reg<spipf330::Spipf330Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf330;
#[doc = "SPIPF334 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf334::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf334::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf334`] module"]
#[doc(alias = "SPIPF334")]
pub type Spipf334 = crate::Reg<spipf334::Spipf334Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf334;
#[doc = "SPIPF338 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf338::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf338::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf338`] module"]
#[doc(alias = "SPIPF338")]
pub type Spipf338 = crate::Reg<spipf338::Spipf338Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf338;
#[doc = "SPIPF33C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf33c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf33c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf33c`] module"]
#[doc(alias = "SPIPF33C")]
pub type Spipf33c = crate::Reg<spipf33c::Spipf33cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf33c;
#[doc = "SPIPF340 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf340::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf340::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf340`] module"]
#[doc(alias = "SPIPF340")]
pub type Spipf340 = crate::Reg<spipf340::Spipf340Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf340;
#[doc = "SPIPF344 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf344::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf344::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf344`] module"]
#[doc(alias = "SPIPF344")]
pub type Spipf344 = crate::Reg<spipf344::Spipf344Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf344;
#[doc = "SPIPF348 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf348::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf348::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf348`] module"]
#[doc(alias = "SPIPF348")]
pub type Spipf348 = crate::Reg<spipf348::Spipf348Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf348;
#[doc = "SPIPF34C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf34c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf34c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf34c`] module"]
#[doc(alias = "SPIPF34C")]
pub type Spipf34c = crate::Reg<spipf34c::Spipf34cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf34c;
#[doc = "SPIPF350 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf350::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf350::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf350`] module"]
#[doc(alias = "SPIPF350")]
pub type Spipf350 = crate::Reg<spipf350::Spipf350Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf350;
#[doc = "SPIPF354 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf354::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf354::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf354`] module"]
#[doc(alias = "SPIPF354")]
pub type Spipf354 = crate::Reg<spipf354::Spipf354Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf354;
#[doc = "SPIPF358 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf358::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf358::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf358`] module"]
#[doc(alias = "SPIPF358")]
pub type Spipf358 = crate::Reg<spipf358::Spipf358Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf358;
#[doc = "SPIPF35C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf35c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf35c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf35c`] module"]
#[doc(alias = "SPIPF35C")]
pub type Spipf35c = crate::Reg<spipf35c::Spipf35cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf35c;
#[doc = "SPIPF360 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf360::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf360::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf360`] module"]
#[doc(alias = "SPIPF360")]
pub type Spipf360 = crate::Reg<spipf360::Spipf360Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf360;
#[doc = "SPIPF364 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf364::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf364::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf364`] module"]
#[doc(alias = "SPIPF364")]
pub type Spipf364 = crate::Reg<spipf364::Spipf364Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf364;
#[doc = "SPIPF368 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf368::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf368::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf368`] module"]
#[doc(alias = "SPIPF368")]
pub type Spipf368 = crate::Reg<spipf368::Spipf368Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf368;
#[doc = "SPIPF36C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf36c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf36c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf36c`] module"]
#[doc(alias = "SPIPF36C")]
pub type Spipf36c = crate::Reg<spipf36c::Spipf36cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf36c;
#[doc = "SPIPF370 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf370::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf370::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf370`] module"]
#[doc(alias = "SPIPF370")]
pub type Spipf370 = crate::Reg<spipf370::Spipf370Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf370;
#[doc = "SPIPF374 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf374::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf374::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf374`] module"]
#[doc(alias = "SPIPF374")]
pub type Spipf374 = crate::Reg<spipf374::Spipf374Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf374;
#[doc = "SPIPF378 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf378::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf378::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf378`] module"]
#[doc(alias = "SPIPF378")]
pub type Spipf378 = crate::Reg<spipf378::Spipf378Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf378;
#[doc = "SPIPF37C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf37c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf37c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf37c`] module"]
#[doc(alias = "SPIPF37C")]
pub type Spipf37c = crate::Reg<spipf37c::Spipf37cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf37c;
#[doc = "SPIPF380 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf380::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf380::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf380`] module"]
#[doc(alias = "SPIPF380")]
pub type Spipf380 = crate::Reg<spipf380::Spipf380Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf380;
#[doc = "SPIPF384 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf384::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf384::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf384`] module"]
#[doc(alias = "SPIPF384")]
pub type Spipf384 = crate::Reg<spipf384::Spipf384Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf384;
#[doc = "SPIPF388 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf388::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf388::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf388`] module"]
#[doc(alias = "SPIPF388")]
pub type Spipf388 = crate::Reg<spipf388::Spipf388Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf388;
#[doc = "SPIPF38C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf38c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf38c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf38c`] module"]
#[doc(alias = "SPIPF38C")]
pub type Spipf38c = crate::Reg<spipf38c::Spipf38cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf38c;
#[doc = "SPIPF390 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf390::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf390::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf390`] module"]
#[doc(alias = "SPIPF390")]
pub type Spipf390 = crate::Reg<spipf390::Spipf390Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf390;
#[doc = "SPIPF394 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf394::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf394::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf394`] module"]
#[doc(alias = "SPIPF394")]
pub type Spipf394 = crate::Reg<spipf394::Spipf394Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf394;
#[doc = "SPIPF398 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf398::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf398::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf398`] module"]
#[doc(alias = "SPIPF398")]
pub type Spipf398 = crate::Reg<spipf398::Spipf398Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf398;
#[doc = "SPIPF39C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf39c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf39c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf39c`] module"]
#[doc(alias = "SPIPF39C")]
pub type Spipf39c = crate::Reg<spipf39c::Spipf39cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf39c;
#[doc = "SPIPF3A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3a0`] module"]
#[doc(alias = "SPIPF3A0")]
pub type Spipf3a0 = crate::Reg<spipf3a0::Spipf3a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3a0;
#[doc = "SPIPF3A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3a4`] module"]
#[doc(alias = "SPIPF3A4")]
pub type Spipf3a4 = crate::Reg<spipf3a4::Spipf3a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3a4;
#[doc = "SPIPF3A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3a8`] module"]
#[doc(alias = "SPIPF3A8")]
pub type Spipf3a8 = crate::Reg<spipf3a8::Spipf3a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3a8;
#[doc = "SPIPF3AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3ac`] module"]
#[doc(alias = "SPIPF3AC")]
pub type Spipf3ac = crate::Reg<spipf3ac::Spipf3acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3ac;
#[doc = "SPIPF3B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3b0`] module"]
#[doc(alias = "SPIPF3B0")]
pub type Spipf3b0 = crate::Reg<spipf3b0::Spipf3b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3b0;
#[doc = "SPIPF3B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3b4`] module"]
#[doc(alias = "SPIPF3B4")]
pub type Spipf3b4 = crate::Reg<spipf3b4::Spipf3b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3b4;
#[doc = "SPIPF3B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3b8`] module"]
#[doc(alias = "SPIPF3B8")]
pub type Spipf3b8 = crate::Reg<spipf3b8::Spipf3b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3b8;
#[doc = "SPIPF3BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3bc`] module"]
#[doc(alias = "SPIPF3BC")]
pub type Spipf3bc = crate::Reg<spipf3bc::Spipf3bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3bc;
#[doc = "SPIPF3C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3c0`] module"]
#[doc(alias = "SPIPF3C0")]
pub type Spipf3c0 = crate::Reg<spipf3c0::Spipf3c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3c0;
#[doc = "SPIPF3C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3c4`] module"]
#[doc(alias = "SPIPF3C4")]
pub type Spipf3c4 = crate::Reg<spipf3c4::Spipf3c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3c4;
#[doc = "SPIPF3C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3c8`] module"]
#[doc(alias = "SPIPF3C8")]
pub type Spipf3c8 = crate::Reg<spipf3c8::Spipf3c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3c8;
#[doc = "SPIPF3CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3cc`] module"]
#[doc(alias = "SPIPF3CC")]
pub type Spipf3cc = crate::Reg<spipf3cc::Spipf3ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3cc;
#[doc = "SPIPF3D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3d0`] module"]
#[doc(alias = "SPIPF3D0")]
pub type Spipf3d0 = crate::Reg<spipf3d0::Spipf3d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3d0;
#[doc = "SPIPF3D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3d4`] module"]
#[doc(alias = "SPIPF3D4")]
pub type Spipf3d4 = crate::Reg<spipf3d4::Spipf3d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3d4;
#[doc = "SPIPF3D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3d8`] module"]
#[doc(alias = "SPIPF3D8")]
pub type Spipf3d8 = crate::Reg<spipf3d8::Spipf3d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3d8;
#[doc = "SPIPF3DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3dc`] module"]
#[doc(alias = "SPIPF3DC")]
pub type Spipf3dc = crate::Reg<spipf3dc::Spipf3dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3dc;
#[doc = "SPIPF3E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3e0`] module"]
#[doc(alias = "SPIPF3E0")]
pub type Spipf3e0 = crate::Reg<spipf3e0::Spipf3e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3e0;
#[doc = "SPIPF3E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3e4`] module"]
#[doc(alias = "SPIPF3E4")]
pub type Spipf3e4 = crate::Reg<spipf3e4::Spipf3e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3e4;
#[doc = "SPIPF3E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3e8`] module"]
#[doc(alias = "SPIPF3E8")]
pub type Spipf3e8 = crate::Reg<spipf3e8::Spipf3e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3e8;
#[doc = "SPIPF3EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3ec`] module"]
#[doc(alias = "SPIPF3EC")]
pub type Spipf3ec = crate::Reg<spipf3ec::Spipf3ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3ec;
#[doc = "SPIPF3F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3f0`] module"]
#[doc(alias = "SPIPF3F0")]
pub type Spipf3f0 = crate::Reg<spipf3f0::Spipf3f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3f0;
#[doc = "SPIPF3F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3f4`] module"]
#[doc(alias = "SPIPF3F4")]
pub type Spipf3f4 = crate::Reg<spipf3f4::Spipf3f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3f4;
#[doc = "SPIPF3F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3f8`] module"]
#[doc(alias = "SPIPF3F8")]
pub type Spipf3f8 = crate::Reg<spipf3f8::Spipf3f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3f8;
#[doc = "SPIPF3FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf3fc`] module"]
#[doc(alias = "SPIPF3FC")]
pub type Spipf3fc = crate::Reg<spipf3fc::Spipf3fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf3fc;
#[doc = "SPIPF400 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf400::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf400::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf400`] module"]
#[doc(alias = "SPIPF400")]
pub type Spipf400 = crate::Reg<spipf400::Spipf400Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf400;
#[doc = "SPIPF404 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf404::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf404::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf404`] module"]
#[doc(alias = "SPIPF404")]
pub type Spipf404 = crate::Reg<spipf404::Spipf404Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf404;
#[doc = "SPIPF408 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf408::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf408::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf408`] module"]
#[doc(alias = "SPIPF408")]
pub type Spipf408 = crate::Reg<spipf408::Spipf408Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf408;
#[doc = "SPIPF40C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf40c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf40c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf40c`] module"]
#[doc(alias = "SPIPF40C")]
pub type Spipf40c = crate::Reg<spipf40c::Spipf40cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf40c;
#[doc = "SPIPF410 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf410`] module"]
#[doc(alias = "SPIPF410")]
pub type Spipf410 = crate::Reg<spipf410::Spipf410Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf410;
#[doc = "SPIPF414 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf414`] module"]
#[doc(alias = "SPIPF414")]
pub type Spipf414 = crate::Reg<spipf414::Spipf414Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf414;
#[doc = "SPIPF418 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf418`] module"]
#[doc(alias = "SPIPF418")]
pub type Spipf418 = crate::Reg<spipf418::Spipf418Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf418;
#[doc = "SPIPF41C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf41c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf41c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf41c`] module"]
#[doc(alias = "SPIPF41C")]
pub type Spipf41c = crate::Reg<spipf41c::Spipf41cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf41c;
#[doc = "SPIPF420 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf420::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf420::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf420`] module"]
#[doc(alias = "SPIPF420")]
pub type Spipf420 = crate::Reg<spipf420::Spipf420Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf420;
#[doc = "SPIPF424 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf424::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf424::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf424`] module"]
#[doc(alias = "SPIPF424")]
pub type Spipf424 = crate::Reg<spipf424::Spipf424Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf424;
#[doc = "SPIPF428 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf428::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf428::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf428`] module"]
#[doc(alias = "SPIPF428")]
pub type Spipf428 = crate::Reg<spipf428::Spipf428Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf428;
#[doc = "SPIPF42C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf42c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf42c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf42c`] module"]
#[doc(alias = "SPIPF42C")]
pub type Spipf42c = crate::Reg<spipf42c::Spipf42cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf42c;
#[doc = "SPIPF430 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf430::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf430::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf430`] module"]
#[doc(alias = "SPIPF430")]
pub type Spipf430 = crate::Reg<spipf430::Spipf430Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf430;
#[doc = "SPIPF434 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf434::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf434::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf434`] module"]
#[doc(alias = "SPIPF434")]
pub type Spipf434 = crate::Reg<spipf434::Spipf434Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf434;
#[doc = "SPIPF438 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf438::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf438::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf438`] module"]
#[doc(alias = "SPIPF438")]
pub type Spipf438 = crate::Reg<spipf438::Spipf438Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf438;
#[doc = "SPIPF43C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf43c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf43c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf43c`] module"]
#[doc(alias = "SPIPF43C")]
pub type Spipf43c = crate::Reg<spipf43c::Spipf43cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf43c;
#[doc = "SPIPF440 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf440::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf440::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf440`] module"]
#[doc(alias = "SPIPF440")]
pub type Spipf440 = crate::Reg<spipf440::Spipf440Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf440;
#[doc = "SPIPF444 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf444`] module"]
#[doc(alias = "SPIPF444")]
pub type Spipf444 = crate::Reg<spipf444::Spipf444Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf444;
#[doc = "SPIPF448 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf448::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf448::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf448`] module"]
#[doc(alias = "SPIPF448")]
pub type Spipf448 = crate::Reg<spipf448::Spipf448Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf448;
#[doc = "SPIPF44C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf44c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf44c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf44c`] module"]
#[doc(alias = "SPIPF44C")]
pub type Spipf44c = crate::Reg<spipf44c::Spipf44cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf44c;
#[doc = "SPIPF450 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf450::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf450::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf450`] module"]
#[doc(alias = "SPIPF450")]
pub type Spipf450 = crate::Reg<spipf450::Spipf450Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf450;
#[doc = "SPIPF454 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf454::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf454::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf454`] module"]
#[doc(alias = "SPIPF454")]
pub type Spipf454 = crate::Reg<spipf454::Spipf454Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf454;
#[doc = "SPIPF458 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf458::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf458::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf458`] module"]
#[doc(alias = "SPIPF458")]
pub type Spipf458 = crate::Reg<spipf458::Spipf458Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf458;
#[doc = "SPIPF45C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf45c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf45c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf45c`] module"]
#[doc(alias = "SPIPF45C")]
pub type Spipf45c = crate::Reg<spipf45c::Spipf45cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf45c;
#[doc = "SPIPF460 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf460::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf460::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf460`] module"]
#[doc(alias = "SPIPF460")]
pub type Spipf460 = crate::Reg<spipf460::Spipf460Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf460;
#[doc = "SPIPF464 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf464::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf464::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf464`] module"]
#[doc(alias = "SPIPF464")]
pub type Spipf464 = crate::Reg<spipf464::Spipf464Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf464;
#[doc = "SPIPF468 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf468::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf468::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf468`] module"]
#[doc(alias = "SPIPF468")]
pub type Spipf468 = crate::Reg<spipf468::Spipf468Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf468;
#[doc = "SPIPF46C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf46c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf46c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf46c`] module"]
#[doc(alias = "SPIPF46C")]
pub type Spipf46c = crate::Reg<spipf46c::Spipf46cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf46c;
#[doc = "SPIPF470 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf470::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf470::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf470`] module"]
#[doc(alias = "SPIPF470")]
pub type Spipf470 = crate::Reg<spipf470::Spipf470Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf470;
#[doc = "SPIPF474 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf474::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf474::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf474`] module"]
#[doc(alias = "SPIPF474")]
pub type Spipf474 = crate::Reg<spipf474::Spipf474Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf474;
#[doc = "SPIPF478 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf478::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf478::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf478`] module"]
#[doc(alias = "SPIPF478")]
pub type Spipf478 = crate::Reg<spipf478::Spipf478Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf478;
#[doc = "SPIPF47C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf47c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf47c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf47c`] module"]
#[doc(alias = "SPIPF47C")]
pub type Spipf47c = crate::Reg<spipf47c::Spipf47cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf47c;
#[doc = "SPIPF480 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf480::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf480::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf480`] module"]
#[doc(alias = "SPIPF480")]
pub type Spipf480 = crate::Reg<spipf480::Spipf480Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf480;
#[doc = "SPIPF484 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf484::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf484::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf484`] module"]
#[doc(alias = "SPIPF484")]
pub type Spipf484 = crate::Reg<spipf484::Spipf484Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf484;
#[doc = "SPIPF488 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf488::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf488::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf488`] module"]
#[doc(alias = "SPIPF488")]
pub type Spipf488 = crate::Reg<spipf488::Spipf488Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf488;
#[doc = "SPIPF48C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf48c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf48c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf48c`] module"]
#[doc(alias = "SPIPF48C")]
pub type Spipf48c = crate::Reg<spipf48c::Spipf48cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf48c;
#[doc = "SPIPF490 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf490::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf490::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf490`] module"]
#[doc(alias = "SPIPF490")]
pub type Spipf490 = crate::Reg<spipf490::Spipf490Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf490;
#[doc = "SPIPF494 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf494::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf494::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf494`] module"]
#[doc(alias = "SPIPF494")]
pub type Spipf494 = crate::Reg<spipf494::Spipf494Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf494;
#[doc = "SPIPF498 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf498::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf498::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf498`] module"]
#[doc(alias = "SPIPF498")]
pub type Spipf498 = crate::Reg<spipf498::Spipf498Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf498;
#[doc = "SPIPF49C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf49c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf49c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf49c`] module"]
#[doc(alias = "SPIPF49C")]
pub type Spipf49c = crate::Reg<spipf49c::Spipf49cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf49c;
#[doc = "SPIPF4A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4a0`] module"]
#[doc(alias = "SPIPF4A0")]
pub type Spipf4a0 = crate::Reg<spipf4a0::Spipf4a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4a0;
#[doc = "SPIPF4A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4a4`] module"]
#[doc(alias = "SPIPF4A4")]
pub type Spipf4a4 = crate::Reg<spipf4a4::Spipf4a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4a4;
#[doc = "SPIPF4A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4a8`] module"]
#[doc(alias = "SPIPF4A8")]
pub type Spipf4a8 = crate::Reg<spipf4a8::Spipf4a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4a8;
#[doc = "SPIPF4AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4ac`] module"]
#[doc(alias = "SPIPF4AC")]
pub type Spipf4ac = crate::Reg<spipf4ac::Spipf4acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4ac;
#[doc = "SPIPF4B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4b0`] module"]
#[doc(alias = "SPIPF4B0")]
pub type Spipf4b0 = crate::Reg<spipf4b0::Spipf4b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4b0;
#[doc = "SPIPF4B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4b4`] module"]
#[doc(alias = "SPIPF4B4")]
pub type Spipf4b4 = crate::Reg<spipf4b4::Spipf4b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4b4;
#[doc = "SPIPF4B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4b8`] module"]
#[doc(alias = "SPIPF4B8")]
pub type Spipf4b8 = crate::Reg<spipf4b8::Spipf4b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4b8;
#[doc = "SPIPF4BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4bc`] module"]
#[doc(alias = "SPIPF4BC")]
pub type Spipf4bc = crate::Reg<spipf4bc::Spipf4bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4bc;
#[doc = "SPIPF4C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4c0`] module"]
#[doc(alias = "SPIPF4C0")]
pub type Spipf4c0 = crate::Reg<spipf4c0::Spipf4c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4c0;
#[doc = "SPIPF4C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4c4`] module"]
#[doc(alias = "SPIPF4C4")]
pub type Spipf4c4 = crate::Reg<spipf4c4::Spipf4c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4c4;
#[doc = "SPIPF4C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4c8`] module"]
#[doc(alias = "SPIPF4C8")]
pub type Spipf4c8 = crate::Reg<spipf4c8::Spipf4c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4c8;
#[doc = "SPIPF4CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4cc`] module"]
#[doc(alias = "SPIPF4CC")]
pub type Spipf4cc = crate::Reg<spipf4cc::Spipf4ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4cc;
#[doc = "SPIPF4D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4d0`] module"]
#[doc(alias = "SPIPF4D0")]
pub type Spipf4d0 = crate::Reg<spipf4d0::Spipf4d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4d0;
#[doc = "SPIPF4D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4d4`] module"]
#[doc(alias = "SPIPF4D4")]
pub type Spipf4d4 = crate::Reg<spipf4d4::Spipf4d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4d4;
#[doc = "SPIPF4D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4d8`] module"]
#[doc(alias = "SPIPF4D8")]
pub type Spipf4d8 = crate::Reg<spipf4d8::Spipf4d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4d8;
#[doc = "SPIPF4DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4dc`] module"]
#[doc(alias = "SPIPF4DC")]
pub type Spipf4dc = crate::Reg<spipf4dc::Spipf4dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4dc;
#[doc = "SPIPF4E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4e0`] module"]
#[doc(alias = "SPIPF4E0")]
pub type Spipf4e0 = crate::Reg<spipf4e0::Spipf4e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4e0;
#[doc = "SPIPF4E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4e4`] module"]
#[doc(alias = "SPIPF4E4")]
pub type Spipf4e4 = crate::Reg<spipf4e4::Spipf4e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4e4;
#[doc = "SPIPF4E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4e8`] module"]
#[doc(alias = "SPIPF4E8")]
pub type Spipf4e8 = crate::Reg<spipf4e8::Spipf4e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4e8;
#[doc = "SPIPF4EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4ec`] module"]
#[doc(alias = "SPIPF4EC")]
pub type Spipf4ec = crate::Reg<spipf4ec::Spipf4ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4ec;
#[doc = "SPIPF4F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4f0`] module"]
#[doc(alias = "SPIPF4F0")]
pub type Spipf4f0 = crate::Reg<spipf4f0::Spipf4f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4f0;
#[doc = "SPIPF4F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4f4`] module"]
#[doc(alias = "SPIPF4F4")]
pub type Spipf4f4 = crate::Reg<spipf4f4::Spipf4f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4f4;
#[doc = "SPIPF4F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4f8`] module"]
#[doc(alias = "SPIPF4F8")]
pub type Spipf4f8 = crate::Reg<spipf4f8::Spipf4f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4f8;
#[doc = "SPIPF4FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf4fc`] module"]
#[doc(alias = "SPIPF4FC")]
pub type Spipf4fc = crate::Reg<spipf4fc::Spipf4fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf4fc;
#[doc = "SPIPF500 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf500::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf500::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf500`] module"]
#[doc(alias = "SPIPF500")]
pub type Spipf500 = crate::Reg<spipf500::Spipf500Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf500;
#[doc = "SPIPF504 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf504::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf504::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf504`] module"]
#[doc(alias = "SPIPF504")]
pub type Spipf504 = crate::Reg<spipf504::Spipf504Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf504;
#[doc = "SPIPF508 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf508::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf508::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf508`] module"]
#[doc(alias = "SPIPF508")]
pub type Spipf508 = crate::Reg<spipf508::Spipf508Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf508;
#[doc = "SPIPF50C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf50c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf50c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf50c`] module"]
#[doc(alias = "SPIPF50C")]
pub type Spipf50c = crate::Reg<spipf50c::Spipf50cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf50c;
#[doc = "SPIPF510 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf510`] module"]
#[doc(alias = "SPIPF510")]
pub type Spipf510 = crate::Reg<spipf510::Spipf510Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf510;
#[doc = "SPIPF514 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf514`] module"]
#[doc(alias = "SPIPF514")]
pub type Spipf514 = crate::Reg<spipf514::Spipf514Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf514;
#[doc = "SPIPF518 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf518::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf518::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf518`] module"]
#[doc(alias = "SPIPF518")]
pub type Spipf518 = crate::Reg<spipf518::Spipf518Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf518;
#[doc = "SPIPF51C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf51c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf51c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf51c`] module"]
#[doc(alias = "SPIPF51C")]
pub type Spipf51c = crate::Reg<spipf51c::Spipf51cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf51c;
#[doc = "SPIPF520 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf520::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf520::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf520`] module"]
#[doc(alias = "SPIPF520")]
pub type Spipf520 = crate::Reg<spipf520::Spipf520Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf520;
#[doc = "SPIPF524 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf524::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf524::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf524`] module"]
#[doc(alias = "SPIPF524")]
pub type Spipf524 = crate::Reg<spipf524::Spipf524Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf524;
#[doc = "SPIPF528 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf528::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf528::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf528`] module"]
#[doc(alias = "SPIPF528")]
pub type Spipf528 = crate::Reg<spipf528::Spipf528Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf528;
#[doc = "SPIPF52C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf52c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf52c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf52c`] module"]
#[doc(alias = "SPIPF52C")]
pub type Spipf52c = crate::Reg<spipf52c::Spipf52cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf52c;
#[doc = "SPIPF530 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf530::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf530::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf530`] module"]
#[doc(alias = "SPIPF530")]
pub type Spipf530 = crate::Reg<spipf530::Spipf530Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf530;
#[doc = "SPIPF534 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf534::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf534::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf534`] module"]
#[doc(alias = "SPIPF534")]
pub type Spipf534 = crate::Reg<spipf534::Spipf534Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf534;
#[doc = "SPIPF538 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf538::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf538::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf538`] module"]
#[doc(alias = "SPIPF538")]
pub type Spipf538 = crate::Reg<spipf538::Spipf538Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf538;
#[doc = "SPIPF53C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf53c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf53c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf53c`] module"]
#[doc(alias = "SPIPF53C")]
pub type Spipf53c = crate::Reg<spipf53c::Spipf53cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf53c;
#[doc = "SPIPF540 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf540::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf540::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf540`] module"]
#[doc(alias = "SPIPF540")]
pub type Spipf540 = crate::Reg<spipf540::Spipf540Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf540;
#[doc = "SPIPF544 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf544::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf544::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf544`] module"]
#[doc(alias = "SPIPF544")]
pub type Spipf544 = crate::Reg<spipf544::Spipf544Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf544;
#[doc = "SPIPF548 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf548::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf548::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf548`] module"]
#[doc(alias = "SPIPF548")]
pub type Spipf548 = crate::Reg<spipf548::Spipf548Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf548;
#[doc = "SPIPF54C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf54c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf54c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf54c`] module"]
#[doc(alias = "SPIPF54C")]
pub type Spipf54c = crate::Reg<spipf54c::Spipf54cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf54c;
#[doc = "SPIPF550 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf550::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf550::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf550`] module"]
#[doc(alias = "SPIPF550")]
pub type Spipf550 = crate::Reg<spipf550::Spipf550Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf550;
#[doc = "SPIPF554 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf554::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf554::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf554`] module"]
#[doc(alias = "SPIPF554")]
pub type Spipf554 = crate::Reg<spipf554::Spipf554Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf554;
#[doc = "SPIPF558 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf558::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf558::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf558`] module"]
#[doc(alias = "SPIPF558")]
pub type Spipf558 = crate::Reg<spipf558::Spipf558Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf558;
#[doc = "SPIPF55C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf55c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf55c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf55c`] module"]
#[doc(alias = "SPIPF55C")]
pub type Spipf55c = crate::Reg<spipf55c::Spipf55cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf55c;
#[doc = "SPIPF560 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf560::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf560::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf560`] module"]
#[doc(alias = "SPIPF560")]
pub type Spipf560 = crate::Reg<spipf560::Spipf560Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf560;
#[doc = "SPIPF564 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf564::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf564::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf564`] module"]
#[doc(alias = "SPIPF564")]
pub type Spipf564 = crate::Reg<spipf564::Spipf564Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf564;
#[doc = "SPIPF568 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf568::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf568::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf568`] module"]
#[doc(alias = "SPIPF568")]
pub type Spipf568 = crate::Reg<spipf568::Spipf568Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf568;
#[doc = "SPIPF56C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf56c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf56c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf56c`] module"]
#[doc(alias = "SPIPF56C")]
pub type Spipf56c = crate::Reg<spipf56c::Spipf56cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf56c;
#[doc = "SPIPF570 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf570::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf570::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf570`] module"]
#[doc(alias = "SPIPF570")]
pub type Spipf570 = crate::Reg<spipf570::Spipf570Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf570;
#[doc = "SPIPF574 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf574::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf574::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf574`] module"]
#[doc(alias = "SPIPF574")]
pub type Spipf574 = crate::Reg<spipf574::Spipf574Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf574;
#[doc = "SPIPF578 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf578::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf578::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf578`] module"]
#[doc(alias = "SPIPF578")]
pub type Spipf578 = crate::Reg<spipf578::Spipf578Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf578;
#[doc = "SPIPF57C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf57c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf57c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf57c`] module"]
#[doc(alias = "SPIPF57C")]
pub type Spipf57c = crate::Reg<spipf57c::Spipf57cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf57c;
#[doc = "SPIPF580 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf580::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf580::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf580`] module"]
#[doc(alias = "SPIPF580")]
pub type Spipf580 = crate::Reg<spipf580::Spipf580Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf580;
#[doc = "SPIPF584 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf584::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf584::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf584`] module"]
#[doc(alias = "SPIPF584")]
pub type Spipf584 = crate::Reg<spipf584::Spipf584Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf584;
#[doc = "SPIPF588 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf588::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf588::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf588`] module"]
#[doc(alias = "SPIPF588")]
pub type Spipf588 = crate::Reg<spipf588::Spipf588Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf588;
#[doc = "SPIPF58C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf58c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf58c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf58c`] module"]
#[doc(alias = "SPIPF58C")]
pub type Spipf58c = crate::Reg<spipf58c::Spipf58cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf58c;
#[doc = "SPIPF590 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf590::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf590::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf590`] module"]
#[doc(alias = "SPIPF590")]
pub type Spipf590 = crate::Reg<spipf590::Spipf590Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf590;
#[doc = "SPIPF594 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf594::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf594::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf594`] module"]
#[doc(alias = "SPIPF594")]
pub type Spipf594 = crate::Reg<spipf594::Spipf594Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf594;
#[doc = "SPIPF598 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf598::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf598::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf598`] module"]
#[doc(alias = "SPIPF598")]
pub type Spipf598 = crate::Reg<spipf598::Spipf598Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf598;
#[doc = "SPIPF59C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf59c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf59c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf59c`] module"]
#[doc(alias = "SPIPF59C")]
pub type Spipf59c = crate::Reg<spipf59c::Spipf59cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf59c;
#[doc = "SPIPF5A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5a0`] module"]
#[doc(alias = "SPIPF5A0")]
pub type Spipf5a0 = crate::Reg<spipf5a0::Spipf5a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5a0;
#[doc = "SPIPF5A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5a4`] module"]
#[doc(alias = "SPIPF5A4")]
pub type Spipf5a4 = crate::Reg<spipf5a4::Spipf5a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5a4;
#[doc = "SPIPF5A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5a8`] module"]
#[doc(alias = "SPIPF5A8")]
pub type Spipf5a8 = crate::Reg<spipf5a8::Spipf5a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5a8;
#[doc = "SPIPF5AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5ac`] module"]
#[doc(alias = "SPIPF5AC")]
pub type Spipf5ac = crate::Reg<spipf5ac::Spipf5acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5ac;
#[doc = "SPIPF5B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5b0`] module"]
#[doc(alias = "SPIPF5B0")]
pub type Spipf5b0 = crate::Reg<spipf5b0::Spipf5b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5b0;
#[doc = "SPIPF5B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5b4`] module"]
#[doc(alias = "SPIPF5B4")]
pub type Spipf5b4 = crate::Reg<spipf5b4::Spipf5b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5b4;
#[doc = "SPIPF5B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5b8`] module"]
#[doc(alias = "SPIPF5B8")]
pub type Spipf5b8 = crate::Reg<spipf5b8::Spipf5b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5b8;
#[doc = "SPIPF5BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5bc`] module"]
#[doc(alias = "SPIPF5BC")]
pub type Spipf5bc = crate::Reg<spipf5bc::Spipf5bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5bc;
#[doc = "SPIPF5C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5c0`] module"]
#[doc(alias = "SPIPF5C0")]
pub type Spipf5c0 = crate::Reg<spipf5c0::Spipf5c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5c0;
#[doc = "SPIPF5C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5c4`] module"]
#[doc(alias = "SPIPF5C4")]
pub type Spipf5c4 = crate::Reg<spipf5c4::Spipf5c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5c4;
#[doc = "SPIPF5C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5c8`] module"]
#[doc(alias = "SPIPF5C8")]
pub type Spipf5c8 = crate::Reg<spipf5c8::Spipf5c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5c8;
#[doc = "SPIPF5CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5cc`] module"]
#[doc(alias = "SPIPF5CC")]
pub type Spipf5cc = crate::Reg<spipf5cc::Spipf5ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5cc;
#[doc = "SPIPF5D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5d0`] module"]
#[doc(alias = "SPIPF5D0")]
pub type Spipf5d0 = crate::Reg<spipf5d0::Spipf5d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5d0;
#[doc = "SPIPF5D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5d4`] module"]
#[doc(alias = "SPIPF5D4")]
pub type Spipf5d4 = crate::Reg<spipf5d4::Spipf5d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5d4;
#[doc = "SPIPF5D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5d8`] module"]
#[doc(alias = "SPIPF5D8")]
pub type Spipf5d8 = crate::Reg<spipf5d8::Spipf5d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5d8;
#[doc = "SPIPF5DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5dc`] module"]
#[doc(alias = "SPIPF5DC")]
pub type Spipf5dc = crate::Reg<spipf5dc::Spipf5dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5dc;
#[doc = "SPIPF5E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5e0`] module"]
#[doc(alias = "SPIPF5E0")]
pub type Spipf5e0 = crate::Reg<spipf5e0::Spipf5e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5e0;
#[doc = "SPIPF5E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5e4`] module"]
#[doc(alias = "SPIPF5E4")]
pub type Spipf5e4 = crate::Reg<spipf5e4::Spipf5e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5e4;
#[doc = "SPIPF5E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5e8`] module"]
#[doc(alias = "SPIPF5E8")]
pub type Spipf5e8 = crate::Reg<spipf5e8::Spipf5e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5e8;
#[doc = "SPIPF5EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5ec`] module"]
#[doc(alias = "SPIPF5EC")]
pub type Spipf5ec = crate::Reg<spipf5ec::Spipf5ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5ec;
#[doc = "SPIPF5F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5f0`] module"]
#[doc(alias = "SPIPF5F0")]
pub type Spipf5f0 = crate::Reg<spipf5f0::Spipf5f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5f0;
#[doc = "SPIPF5F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5f4`] module"]
#[doc(alias = "SPIPF5F4")]
pub type Spipf5f4 = crate::Reg<spipf5f4::Spipf5f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5f4;
#[doc = "SPIPF5F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5f8`] module"]
#[doc(alias = "SPIPF5F8")]
pub type Spipf5f8 = crate::Reg<spipf5f8::Spipf5f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5f8;
#[doc = "SPIPF5FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf5fc`] module"]
#[doc(alias = "SPIPF5FC")]
pub type Spipf5fc = crate::Reg<spipf5fc::Spipf5fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf5fc;
#[doc = "SPIPF600 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf600::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf600::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf600`] module"]
#[doc(alias = "SPIPF600")]
pub type Spipf600 = crate::Reg<spipf600::Spipf600Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf600;
#[doc = "SPIPF604 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf604::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf604::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf604`] module"]
#[doc(alias = "SPIPF604")]
pub type Spipf604 = crate::Reg<spipf604::Spipf604Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf604;
#[doc = "SPIPF608 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf608::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf608::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf608`] module"]
#[doc(alias = "SPIPF608")]
pub type Spipf608 = crate::Reg<spipf608::Spipf608Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf608;
#[doc = "SPIPF60C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf60c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf60c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf60c`] module"]
#[doc(alias = "SPIPF60C")]
pub type Spipf60c = crate::Reg<spipf60c::Spipf60cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf60c;
#[doc = "SPIPF610 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf610::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf610::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf610`] module"]
#[doc(alias = "SPIPF610")]
pub type Spipf610 = crate::Reg<spipf610::Spipf610Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf610;
#[doc = "SPIPF614 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf614::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf614::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf614`] module"]
#[doc(alias = "SPIPF614")]
pub type Spipf614 = crate::Reg<spipf614::Spipf614Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf614;
#[doc = "SPIPF618 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf618::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf618::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf618`] module"]
#[doc(alias = "SPIPF618")]
pub type Spipf618 = crate::Reg<spipf618::Spipf618Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf618;
#[doc = "SPIPF61C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf61c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf61c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf61c`] module"]
#[doc(alias = "SPIPF61C")]
pub type Spipf61c = crate::Reg<spipf61c::Spipf61cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf61c;
#[doc = "SPIPF620 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf620::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf620::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf620`] module"]
#[doc(alias = "SPIPF620")]
pub type Spipf620 = crate::Reg<spipf620::Spipf620Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf620;
#[doc = "SPIPF624 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf624::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf624::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf624`] module"]
#[doc(alias = "SPIPF624")]
pub type Spipf624 = crate::Reg<spipf624::Spipf624Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf624;
#[doc = "SPIPF628 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf628::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf628::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf628`] module"]
#[doc(alias = "SPIPF628")]
pub type Spipf628 = crate::Reg<spipf628::Spipf628Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf628;
#[doc = "SPIPF62C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf62c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf62c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf62c`] module"]
#[doc(alias = "SPIPF62C")]
pub type Spipf62c = crate::Reg<spipf62c::Spipf62cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf62c;
#[doc = "SPIPF630 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf630::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf630::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf630`] module"]
#[doc(alias = "SPIPF630")]
pub type Spipf630 = crate::Reg<spipf630::Spipf630Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf630;
#[doc = "SPIPF634 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf634::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf634::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf634`] module"]
#[doc(alias = "SPIPF634")]
pub type Spipf634 = crate::Reg<spipf634::Spipf634Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf634;
#[doc = "SPIPF638 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf638::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf638::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf638`] module"]
#[doc(alias = "SPIPF638")]
pub type Spipf638 = crate::Reg<spipf638::Spipf638Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf638;
#[doc = "SPIPF63C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf63c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf63c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf63c`] module"]
#[doc(alias = "SPIPF63C")]
pub type Spipf63c = crate::Reg<spipf63c::Spipf63cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf63c;
#[doc = "SPIPF640 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf640::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf640::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf640`] module"]
#[doc(alias = "SPIPF640")]
pub type Spipf640 = crate::Reg<spipf640::Spipf640Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf640;
#[doc = "SPIPF644 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf644::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf644::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf644`] module"]
#[doc(alias = "SPIPF644")]
pub type Spipf644 = crate::Reg<spipf644::Spipf644Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf644;
#[doc = "SPIPF648 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf648::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf648::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf648`] module"]
#[doc(alias = "SPIPF648")]
pub type Spipf648 = crate::Reg<spipf648::Spipf648Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf648;
#[doc = "SPIPF64C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf64c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf64c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf64c`] module"]
#[doc(alias = "SPIPF64C")]
pub type Spipf64c = crate::Reg<spipf64c::Spipf64cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf64c;
#[doc = "SPIPF650 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf650::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf650::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf650`] module"]
#[doc(alias = "SPIPF650")]
pub type Spipf650 = crate::Reg<spipf650::Spipf650Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf650;
#[doc = "SPIPF654 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf654::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf654::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf654`] module"]
#[doc(alias = "SPIPF654")]
pub type Spipf654 = crate::Reg<spipf654::Spipf654Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf654;
#[doc = "SPIPF658 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf658::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf658::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf658`] module"]
#[doc(alias = "SPIPF658")]
pub type Spipf658 = crate::Reg<spipf658::Spipf658Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf658;
#[doc = "SPIPF65C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf65c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf65c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf65c`] module"]
#[doc(alias = "SPIPF65C")]
pub type Spipf65c = crate::Reg<spipf65c::Spipf65cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf65c;
#[doc = "SPIPF660 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf660::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf660::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf660`] module"]
#[doc(alias = "SPIPF660")]
pub type Spipf660 = crate::Reg<spipf660::Spipf660Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf660;
#[doc = "SPIPF664 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf664::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf664::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf664`] module"]
#[doc(alias = "SPIPF664")]
pub type Spipf664 = crate::Reg<spipf664::Spipf664Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf664;
#[doc = "SPIPF668 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf668::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf668::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf668`] module"]
#[doc(alias = "SPIPF668")]
pub type Spipf668 = crate::Reg<spipf668::Spipf668Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf668;
#[doc = "SPIPF66C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf66c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf66c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf66c`] module"]
#[doc(alias = "SPIPF66C")]
pub type Spipf66c = crate::Reg<spipf66c::Spipf66cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf66c;
#[doc = "SPIPF670 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf670::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf670::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf670`] module"]
#[doc(alias = "SPIPF670")]
pub type Spipf670 = crate::Reg<spipf670::Spipf670Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf670;
#[doc = "SPIPF674 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf674::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf674::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf674`] module"]
#[doc(alias = "SPIPF674")]
pub type Spipf674 = crate::Reg<spipf674::Spipf674Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf674;
#[doc = "SPIPF678 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf678::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf678::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf678`] module"]
#[doc(alias = "SPIPF678")]
pub type Spipf678 = crate::Reg<spipf678::Spipf678Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf678;
#[doc = "SPIPF67C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf67c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf67c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf67c`] module"]
#[doc(alias = "SPIPF67C")]
pub type Spipf67c = crate::Reg<spipf67c::Spipf67cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf67c;
#[doc = "SPIPF680 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf680::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf680::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf680`] module"]
#[doc(alias = "SPIPF680")]
pub type Spipf680 = crate::Reg<spipf680::Spipf680Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf680;
#[doc = "SPIPF684 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf684::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf684::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf684`] module"]
#[doc(alias = "SPIPF684")]
pub type Spipf684 = crate::Reg<spipf684::Spipf684Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf684;
#[doc = "SPIPF688 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf688::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf688::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf688`] module"]
#[doc(alias = "SPIPF688")]
pub type Spipf688 = crate::Reg<spipf688::Spipf688Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf688;
#[doc = "SPIPF68C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf68c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf68c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf68c`] module"]
#[doc(alias = "SPIPF68C")]
pub type Spipf68c = crate::Reg<spipf68c::Spipf68cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf68c;
#[doc = "SPIPF690 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf690::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf690::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf690`] module"]
#[doc(alias = "SPIPF690")]
pub type Spipf690 = crate::Reg<spipf690::Spipf690Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf690;
#[doc = "SPIPF694 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf694::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf694::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf694`] module"]
#[doc(alias = "SPIPF694")]
pub type Spipf694 = crate::Reg<spipf694::Spipf694Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf694;
#[doc = "SPIPF698 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf698::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf698::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf698`] module"]
#[doc(alias = "SPIPF698")]
pub type Spipf698 = crate::Reg<spipf698::Spipf698Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf698;
#[doc = "SPIPF69C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf69c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf69c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf69c`] module"]
#[doc(alias = "SPIPF69C")]
pub type Spipf69c = crate::Reg<spipf69c::Spipf69cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf69c;
#[doc = "SPIPF6A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6a0`] module"]
#[doc(alias = "SPIPF6A0")]
pub type Spipf6a0 = crate::Reg<spipf6a0::Spipf6a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6a0;
#[doc = "SPIPF6A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6a4`] module"]
#[doc(alias = "SPIPF6A4")]
pub type Spipf6a4 = crate::Reg<spipf6a4::Spipf6a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6a4;
#[doc = "SPIPF6A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6a8`] module"]
#[doc(alias = "SPIPF6A8")]
pub type Spipf6a8 = crate::Reg<spipf6a8::Spipf6a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6a8;
#[doc = "SPIPF6AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6ac`] module"]
#[doc(alias = "SPIPF6AC")]
pub type Spipf6ac = crate::Reg<spipf6ac::Spipf6acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6ac;
#[doc = "SPIPF6B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6b0`] module"]
#[doc(alias = "SPIPF6B0")]
pub type Spipf6b0 = crate::Reg<spipf6b0::Spipf6b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6b0;
#[doc = "SPIPF6B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6b4`] module"]
#[doc(alias = "SPIPF6B4")]
pub type Spipf6b4 = crate::Reg<spipf6b4::Spipf6b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6b4;
#[doc = "SPIPF6B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6b8`] module"]
#[doc(alias = "SPIPF6B8")]
pub type Spipf6b8 = crate::Reg<spipf6b8::Spipf6b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6b8;
#[doc = "SPIPF6BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6bc`] module"]
#[doc(alias = "SPIPF6BC")]
pub type Spipf6bc = crate::Reg<spipf6bc::Spipf6bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6bc;
#[doc = "SPIPF6C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6c0`] module"]
#[doc(alias = "SPIPF6C0")]
pub type Spipf6c0 = crate::Reg<spipf6c0::Spipf6c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6c0;
#[doc = "SPIPF6C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6c4`] module"]
#[doc(alias = "SPIPF6C4")]
pub type Spipf6c4 = crate::Reg<spipf6c4::Spipf6c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6c4;
#[doc = "SPIPF6C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6c8`] module"]
#[doc(alias = "SPIPF6C8")]
pub type Spipf6c8 = crate::Reg<spipf6c8::Spipf6c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6c8;
#[doc = "SPIPF6CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6cc`] module"]
#[doc(alias = "SPIPF6CC")]
pub type Spipf6cc = crate::Reg<spipf6cc::Spipf6ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6cc;
#[doc = "SPIPF6D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6d0`] module"]
#[doc(alias = "SPIPF6D0")]
pub type Spipf6d0 = crate::Reg<spipf6d0::Spipf6d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6d0;
#[doc = "SPIPF6D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6d4`] module"]
#[doc(alias = "SPIPF6D4")]
pub type Spipf6d4 = crate::Reg<spipf6d4::Spipf6d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6d4;
#[doc = "SPIPF6D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6d8`] module"]
#[doc(alias = "SPIPF6D8")]
pub type Spipf6d8 = crate::Reg<spipf6d8::Spipf6d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6d8;
#[doc = "SPIPF6DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6dc`] module"]
#[doc(alias = "SPIPF6DC")]
pub type Spipf6dc = crate::Reg<spipf6dc::Spipf6dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6dc;
#[doc = "SPIPF6E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6e0`] module"]
#[doc(alias = "SPIPF6E0")]
pub type Spipf6e0 = crate::Reg<spipf6e0::Spipf6e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6e0;
#[doc = "SPIPF6E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6e4`] module"]
#[doc(alias = "SPIPF6E4")]
pub type Spipf6e4 = crate::Reg<spipf6e4::Spipf6e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6e4;
#[doc = "SPIPF6E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6e8`] module"]
#[doc(alias = "SPIPF6E8")]
pub type Spipf6e8 = crate::Reg<spipf6e8::Spipf6e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6e8;
#[doc = "SPIPF6EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6ec`] module"]
#[doc(alias = "SPIPF6EC")]
pub type Spipf6ec = crate::Reg<spipf6ec::Spipf6ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6ec;
#[doc = "SPIPF6F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6f0`] module"]
#[doc(alias = "SPIPF6F0")]
pub type Spipf6f0 = crate::Reg<spipf6f0::Spipf6f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6f0;
#[doc = "SPIPF6F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6f4`] module"]
#[doc(alias = "SPIPF6F4")]
pub type Spipf6f4 = crate::Reg<spipf6f4::Spipf6f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6f4;
#[doc = "SPIPF6F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6f8`] module"]
#[doc(alias = "SPIPF6F8")]
pub type Spipf6f8 = crate::Reg<spipf6f8::Spipf6f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6f8;
#[doc = "SPIPF6FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf6fc`] module"]
#[doc(alias = "SPIPF6FC")]
pub type Spipf6fc = crate::Reg<spipf6fc::Spipf6fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf6fc;
#[doc = "SPIPF700 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf700::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf700::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf700`] module"]
#[doc(alias = "SPIPF700")]
pub type Spipf700 = crate::Reg<spipf700::Spipf700Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf700;
#[doc = "SPIPF704 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf704::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf704::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf704`] module"]
#[doc(alias = "SPIPF704")]
pub type Spipf704 = crate::Reg<spipf704::Spipf704Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf704;
#[doc = "SPIPF708 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf708::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf708::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf708`] module"]
#[doc(alias = "SPIPF708")]
pub type Spipf708 = crate::Reg<spipf708::Spipf708Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf708;
#[doc = "SPIPF70C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf70c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf70c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf70c`] module"]
#[doc(alias = "SPIPF70C")]
pub type Spipf70c = crate::Reg<spipf70c::Spipf70cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf70c;
#[doc = "SPIPF710 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf710::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf710::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf710`] module"]
#[doc(alias = "SPIPF710")]
pub type Spipf710 = crate::Reg<spipf710::Spipf710Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf710;
#[doc = "SPIPF714 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf714::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf714::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf714`] module"]
#[doc(alias = "SPIPF714")]
pub type Spipf714 = crate::Reg<spipf714::Spipf714Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf714;
#[doc = "SPIPF718 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf718::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf718::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf718`] module"]
#[doc(alias = "SPIPF718")]
pub type Spipf718 = crate::Reg<spipf718::Spipf718Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf718;
#[doc = "SPIPF71C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf71c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf71c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf71c`] module"]
#[doc(alias = "SPIPF71C")]
pub type Spipf71c = crate::Reg<spipf71c::Spipf71cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf71c;
#[doc = "SPIPF720 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf720::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf720::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf720`] module"]
#[doc(alias = "SPIPF720")]
pub type Spipf720 = crate::Reg<spipf720::Spipf720Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf720;
#[doc = "SPIPF724 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf724::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf724::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf724`] module"]
#[doc(alias = "SPIPF724")]
pub type Spipf724 = crate::Reg<spipf724::Spipf724Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf724;
#[doc = "SPIPF728 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf728::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf728::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf728`] module"]
#[doc(alias = "SPIPF728")]
pub type Spipf728 = crate::Reg<spipf728::Spipf728Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf728;
#[doc = "SPIPF72C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf72c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf72c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf72c`] module"]
#[doc(alias = "SPIPF72C")]
pub type Spipf72c = crate::Reg<spipf72c::Spipf72cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf72c;
#[doc = "SPIPF730 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf730::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf730::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf730`] module"]
#[doc(alias = "SPIPF730")]
pub type Spipf730 = crate::Reg<spipf730::Spipf730Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf730;
#[doc = "SPIPF734 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf734::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf734::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf734`] module"]
#[doc(alias = "SPIPF734")]
pub type Spipf734 = crate::Reg<spipf734::Spipf734Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf734;
#[doc = "SPIPF738 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf738::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf738::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf738`] module"]
#[doc(alias = "SPIPF738")]
pub type Spipf738 = crate::Reg<spipf738::Spipf738Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf738;
#[doc = "SPIPF73C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf73c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf73c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf73c`] module"]
#[doc(alias = "SPIPF73C")]
pub type Spipf73c = crate::Reg<spipf73c::Spipf73cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf73c;
#[doc = "SPIPF740 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf740::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf740::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf740`] module"]
#[doc(alias = "SPIPF740")]
pub type Spipf740 = crate::Reg<spipf740::Spipf740Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf740;
#[doc = "SPIPF744 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf744::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf744::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf744`] module"]
#[doc(alias = "SPIPF744")]
pub type Spipf744 = crate::Reg<spipf744::Spipf744Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf744;
#[doc = "SPIPF748 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf748::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf748::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf748`] module"]
#[doc(alias = "SPIPF748")]
pub type Spipf748 = crate::Reg<spipf748::Spipf748Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf748;
#[doc = "SPIPF74C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf74c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf74c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf74c`] module"]
#[doc(alias = "SPIPF74C")]
pub type Spipf74c = crate::Reg<spipf74c::Spipf74cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf74c;
#[doc = "SPIPF750 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf750::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf750::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf750`] module"]
#[doc(alias = "SPIPF750")]
pub type Spipf750 = crate::Reg<spipf750::Spipf750Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf750;
#[doc = "SPIPF754 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf754::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf754::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf754`] module"]
#[doc(alias = "SPIPF754")]
pub type Spipf754 = crate::Reg<spipf754::Spipf754Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf754;
#[doc = "SPIPF758 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf758::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf758::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf758`] module"]
#[doc(alias = "SPIPF758")]
pub type Spipf758 = crate::Reg<spipf758::Spipf758Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf758;
#[doc = "SPIPF75C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf75c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf75c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf75c`] module"]
#[doc(alias = "SPIPF75C")]
pub type Spipf75c = crate::Reg<spipf75c::Spipf75cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf75c;
#[doc = "SPIPF760 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf760::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf760::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf760`] module"]
#[doc(alias = "SPIPF760")]
pub type Spipf760 = crate::Reg<spipf760::Spipf760Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf760;
#[doc = "SPIPF764 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf764::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf764::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf764`] module"]
#[doc(alias = "SPIPF764")]
pub type Spipf764 = crate::Reg<spipf764::Spipf764Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf764;
#[doc = "SPIPF768 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf768::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf768::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf768`] module"]
#[doc(alias = "SPIPF768")]
pub type Spipf768 = crate::Reg<spipf768::Spipf768Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf768;
#[doc = "SPIPF76C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf76c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf76c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf76c`] module"]
#[doc(alias = "SPIPF76C")]
pub type Spipf76c = crate::Reg<spipf76c::Spipf76cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf76c;
#[doc = "SPIPF770 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf770::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf770::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf770`] module"]
#[doc(alias = "SPIPF770")]
pub type Spipf770 = crate::Reg<spipf770::Spipf770Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf770;
#[doc = "SPIPF774 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf774::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf774::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf774`] module"]
#[doc(alias = "SPIPF774")]
pub type Spipf774 = crate::Reg<spipf774::Spipf774Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf774;
#[doc = "SPIPF778 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf778::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf778::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf778`] module"]
#[doc(alias = "SPIPF778")]
pub type Spipf778 = crate::Reg<spipf778::Spipf778Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf778;
#[doc = "SPIPF77C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf77c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf77c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf77c`] module"]
#[doc(alias = "SPIPF77C")]
pub type Spipf77c = crate::Reg<spipf77c::Spipf77cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf77c;
#[doc = "SPIPF780 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf780::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf780::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf780`] module"]
#[doc(alias = "SPIPF780")]
pub type Spipf780 = crate::Reg<spipf780::Spipf780Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf780;
#[doc = "SPIPF784 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf784::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf784::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf784`] module"]
#[doc(alias = "SPIPF784")]
pub type Spipf784 = crate::Reg<spipf784::Spipf784Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf784;
#[doc = "SPIPF788 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf788::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf788::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf788`] module"]
#[doc(alias = "SPIPF788")]
pub type Spipf788 = crate::Reg<spipf788::Spipf788Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf788;
#[doc = "SPIPF78C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf78c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf78c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf78c`] module"]
#[doc(alias = "SPIPF78C")]
pub type Spipf78c = crate::Reg<spipf78c::Spipf78cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf78c;
#[doc = "SPIPF790 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf790::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf790::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf790`] module"]
#[doc(alias = "SPIPF790")]
pub type Spipf790 = crate::Reg<spipf790::Spipf790Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf790;
#[doc = "SPIPF794 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf794::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf794::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf794`] module"]
#[doc(alias = "SPIPF794")]
pub type Spipf794 = crate::Reg<spipf794::Spipf794Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf794;
#[doc = "SPIPF798 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf798::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf798::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf798`] module"]
#[doc(alias = "SPIPF798")]
pub type Spipf798 = crate::Reg<spipf798::Spipf798Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf798;
#[doc = "SPIPF79C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf79c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf79c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf79c`] module"]
#[doc(alias = "SPIPF79C")]
pub type Spipf79c = crate::Reg<spipf79c::Spipf79cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf79c;
#[doc = "SPIPF7A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7a0`] module"]
#[doc(alias = "SPIPF7A0")]
pub type Spipf7a0 = crate::Reg<spipf7a0::Spipf7a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7a0;
#[doc = "SPIPF7A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7a4`] module"]
#[doc(alias = "SPIPF7A4")]
pub type Spipf7a4 = crate::Reg<spipf7a4::Spipf7a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7a4;
#[doc = "SPIPF7A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7a8`] module"]
#[doc(alias = "SPIPF7A8")]
pub type Spipf7a8 = crate::Reg<spipf7a8::Spipf7a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7a8;
#[doc = "SPIPF7AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7ac`] module"]
#[doc(alias = "SPIPF7AC")]
pub type Spipf7ac = crate::Reg<spipf7ac::Spipf7acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7ac;
#[doc = "SPIPF7B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7b0`] module"]
#[doc(alias = "SPIPF7B0")]
pub type Spipf7b0 = crate::Reg<spipf7b0::Spipf7b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7b0;
#[doc = "SPIPF7B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7b4`] module"]
#[doc(alias = "SPIPF7B4")]
pub type Spipf7b4 = crate::Reg<spipf7b4::Spipf7b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7b4;
#[doc = "SPIPF7B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7b8`] module"]
#[doc(alias = "SPIPF7B8")]
pub type Spipf7b8 = crate::Reg<spipf7b8::Spipf7b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7b8;
#[doc = "SPIPF7BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7bc`] module"]
#[doc(alias = "SPIPF7BC")]
pub type Spipf7bc = crate::Reg<spipf7bc::Spipf7bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7bc;
#[doc = "SPIPF7C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7c0`] module"]
#[doc(alias = "SPIPF7C0")]
pub type Spipf7c0 = crate::Reg<spipf7c0::Spipf7c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7c0;
#[doc = "SPIPF7C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7c4`] module"]
#[doc(alias = "SPIPF7C4")]
pub type Spipf7c4 = crate::Reg<spipf7c4::Spipf7c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7c4;
#[doc = "SPIPF7C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7c8`] module"]
#[doc(alias = "SPIPF7C8")]
pub type Spipf7c8 = crate::Reg<spipf7c8::Spipf7c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7c8;
#[doc = "SPIPF7CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7cc`] module"]
#[doc(alias = "SPIPF7CC")]
pub type Spipf7cc = crate::Reg<spipf7cc::Spipf7ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7cc;
#[doc = "SPIPF7D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7d0`] module"]
#[doc(alias = "SPIPF7D0")]
pub type Spipf7d0 = crate::Reg<spipf7d0::Spipf7d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7d0;
#[doc = "SPIPF7D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7d4`] module"]
#[doc(alias = "SPIPF7D4")]
pub type Spipf7d4 = crate::Reg<spipf7d4::Spipf7d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7d4;
#[doc = "SPIPF7D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7d8`] module"]
#[doc(alias = "SPIPF7D8")]
pub type Spipf7d8 = crate::Reg<spipf7d8::Spipf7d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7d8;
#[doc = "SPIPF7DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7dc`] module"]
#[doc(alias = "SPIPF7DC")]
pub type Spipf7dc = crate::Reg<spipf7dc::Spipf7dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7dc;
#[doc = "SPIPF7E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7e0`] module"]
#[doc(alias = "SPIPF7E0")]
pub type Spipf7e0 = crate::Reg<spipf7e0::Spipf7e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7e0;
#[doc = "SPIPF7E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7e4`] module"]
#[doc(alias = "SPIPF7E4")]
pub type Spipf7e4 = crate::Reg<spipf7e4::Spipf7e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7e4;
#[doc = "SPIPF7E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7e8`] module"]
#[doc(alias = "SPIPF7E8")]
pub type Spipf7e8 = crate::Reg<spipf7e8::Spipf7e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7e8;
#[doc = "SPIPF7EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7ec`] module"]
#[doc(alias = "SPIPF7EC")]
pub type Spipf7ec = crate::Reg<spipf7ec::Spipf7ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7ec;
#[doc = "SPIPF7F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7f0`] module"]
#[doc(alias = "SPIPF7F0")]
pub type Spipf7f0 = crate::Reg<spipf7f0::Spipf7f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7f0;
#[doc = "SPIPF7F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7f4`] module"]
#[doc(alias = "SPIPF7F4")]
pub type Spipf7f4 = crate::Reg<spipf7f4::Spipf7f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7f4;
#[doc = "SPIPF7F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7f8`] module"]
#[doc(alias = "SPIPF7F8")]
pub type Spipf7f8 = crate::Reg<spipf7f8::Spipf7f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7f8;
#[doc = "SPIPF7FC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf7fc`] module"]
#[doc(alias = "SPIPF7FC")]
pub type Spipf7fc = crate::Reg<spipf7fc::Spipf7fcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf7fc;
#[doc = "SPIPF800 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf800::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf800::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf800`] module"]
#[doc(alias = "SPIPF800")]
pub type Spipf800 = crate::Reg<spipf800::Spipf800Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf800;
#[doc = "SPIPF804 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf804::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf804::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf804`] module"]
#[doc(alias = "SPIPF804")]
pub type Spipf804 = crate::Reg<spipf804::Spipf804Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf804;
#[doc = "SPIPF808 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf808::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf808::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf808`] module"]
#[doc(alias = "SPIPF808")]
pub type Spipf808 = crate::Reg<spipf808::Spipf808Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf808;
#[doc = "SPIPF80C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf80c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf80c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf80c`] module"]
#[doc(alias = "SPIPF80C")]
pub type Spipf80c = crate::Reg<spipf80c::Spipf80cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf80c;
#[doc = "SPIPF810 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf810::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf810::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf810`] module"]
#[doc(alias = "SPIPF810")]
pub type Spipf810 = crate::Reg<spipf810::Spipf810Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf810;
#[doc = "SPIPF814 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf814::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf814::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf814`] module"]
#[doc(alias = "SPIPF814")]
pub type Spipf814 = crate::Reg<spipf814::Spipf814Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf814;
#[doc = "SPIPF818 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf818::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf818::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf818`] module"]
#[doc(alias = "SPIPF818")]
pub type Spipf818 = crate::Reg<spipf818::Spipf818Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf818;
#[doc = "SPIPF81C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf81c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf81c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf81c`] module"]
#[doc(alias = "SPIPF81C")]
pub type Spipf81c = crate::Reg<spipf81c::Spipf81cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf81c;
#[doc = "SPIPF820 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf820::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf820::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf820`] module"]
#[doc(alias = "SPIPF820")]
pub type Spipf820 = crate::Reg<spipf820::Spipf820Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf820;
#[doc = "SPIPF824 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf824::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf824::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf824`] module"]
#[doc(alias = "SPIPF824")]
pub type Spipf824 = crate::Reg<spipf824::Spipf824Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf824;
#[doc = "SPIPF828 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf828::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf828::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf828`] module"]
#[doc(alias = "SPIPF828")]
pub type Spipf828 = crate::Reg<spipf828::Spipf828Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf828;
#[doc = "SPIPF82C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf82c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf82c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf82c`] module"]
#[doc(alias = "SPIPF82C")]
pub type Spipf82c = crate::Reg<spipf82c::Spipf82cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf82c;
#[doc = "SPIPF830 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf830::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf830::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf830`] module"]
#[doc(alias = "SPIPF830")]
pub type Spipf830 = crate::Reg<spipf830::Spipf830Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf830;
#[doc = "SPIPF834 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf834::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf834::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf834`] module"]
#[doc(alias = "SPIPF834")]
pub type Spipf834 = crate::Reg<spipf834::Spipf834Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf834;
#[doc = "SPIPF838 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf838::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf838::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf838`] module"]
#[doc(alias = "SPIPF838")]
pub type Spipf838 = crate::Reg<spipf838::Spipf838Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf838;
#[doc = "SPIPF83C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf83c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf83c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf83c`] module"]
#[doc(alias = "SPIPF83C")]
pub type Spipf83c = crate::Reg<spipf83c::Spipf83cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf83c;
#[doc = "SPIPF840 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf840::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf840::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf840`] module"]
#[doc(alias = "SPIPF840")]
pub type Spipf840 = crate::Reg<spipf840::Spipf840Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf840;
#[doc = "SPIPF844 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf844::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf844::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf844`] module"]
#[doc(alias = "SPIPF844")]
pub type Spipf844 = crate::Reg<spipf844::Spipf844Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf844;
#[doc = "SPIPF848 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf848::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf848::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf848`] module"]
#[doc(alias = "SPIPF848")]
pub type Spipf848 = crate::Reg<spipf848::Spipf848Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf848;
#[doc = "SPIPF84C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf84c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf84c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf84c`] module"]
#[doc(alias = "SPIPF84C")]
pub type Spipf84c = crate::Reg<spipf84c::Spipf84cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf84c;
#[doc = "SPIPF850 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf850::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf850::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf850`] module"]
#[doc(alias = "SPIPF850")]
pub type Spipf850 = crate::Reg<spipf850::Spipf850Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf850;
#[doc = "SPIPF854 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf854::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf854::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf854`] module"]
#[doc(alias = "SPIPF854")]
pub type Spipf854 = crate::Reg<spipf854::Spipf854Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf854;
#[doc = "SPIPF858 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf858::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf858::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf858`] module"]
#[doc(alias = "SPIPF858")]
pub type Spipf858 = crate::Reg<spipf858::Spipf858Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf858;
#[doc = "SPIPF85C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf85c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf85c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf85c`] module"]
#[doc(alias = "SPIPF85C")]
pub type Spipf85c = crate::Reg<spipf85c::Spipf85cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf85c;
#[doc = "SPIPF860 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf860::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf860::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf860`] module"]
#[doc(alias = "SPIPF860")]
pub type Spipf860 = crate::Reg<spipf860::Spipf860Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf860;
#[doc = "SPIPF864 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf864::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf864::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf864`] module"]
#[doc(alias = "SPIPF864")]
pub type Spipf864 = crate::Reg<spipf864::Spipf864Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf864;
#[doc = "SPIPF868 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf868::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf868::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf868`] module"]
#[doc(alias = "SPIPF868")]
pub type Spipf868 = crate::Reg<spipf868::Spipf868Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf868;
#[doc = "SPIPF86C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf86c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf86c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf86c`] module"]
#[doc(alias = "SPIPF86C")]
pub type Spipf86c = crate::Reg<spipf86c::Spipf86cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf86c;
#[doc = "SPIPF870 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf870::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf870::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf870`] module"]
#[doc(alias = "SPIPF870")]
pub type Spipf870 = crate::Reg<spipf870::Spipf870Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf870;
#[doc = "SPIPF874 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf874::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf874::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf874`] module"]
#[doc(alias = "SPIPF874")]
pub type Spipf874 = crate::Reg<spipf874::Spipf874Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf874;
#[doc = "SPIPF878 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf878::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf878::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf878`] module"]
#[doc(alias = "SPIPF878")]
pub type Spipf878 = crate::Reg<spipf878::Spipf878Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf878;
#[doc = "SPIPF87C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf87c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf87c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf87c`] module"]
#[doc(alias = "SPIPF87C")]
pub type Spipf87c = crate::Reg<spipf87c::Spipf87cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf87c;
#[doc = "SPIPF880 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf880::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf880::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf880`] module"]
#[doc(alias = "SPIPF880")]
pub type Spipf880 = crate::Reg<spipf880::Spipf880Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf880;
#[doc = "SPIPF884 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf884::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf884::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf884`] module"]
#[doc(alias = "SPIPF884")]
pub type Spipf884 = crate::Reg<spipf884::Spipf884Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf884;
#[doc = "SPIPF888 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf888`] module"]
#[doc(alias = "SPIPF888")]
pub type Spipf888 = crate::Reg<spipf888::Spipf888Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf888;
#[doc = "SPIPF88C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf88c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf88c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf88c`] module"]
#[doc(alias = "SPIPF88C")]
pub type Spipf88c = crate::Reg<spipf88c::Spipf88cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf88c;
#[doc = "SPIPF890 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf890::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf890::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf890`] module"]
#[doc(alias = "SPIPF890")]
pub type Spipf890 = crate::Reg<spipf890::Spipf890Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf890;
#[doc = "SPIPF894 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf894::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf894::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf894`] module"]
#[doc(alias = "SPIPF894")]
pub type Spipf894 = crate::Reg<spipf894::Spipf894Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf894;
#[doc = "SPIPF898 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf898::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf898::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf898`] module"]
#[doc(alias = "SPIPF898")]
pub type Spipf898 = crate::Reg<spipf898::Spipf898Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf898;
#[doc = "SPIPF89C (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf89c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf89c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf89c`] module"]
#[doc(alias = "SPIPF89C")]
pub type Spipf89c = crate::Reg<spipf89c::Spipf89cSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf89c;
#[doc = "SPIPF8A0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8a0`] module"]
#[doc(alias = "SPIPF8A0")]
pub type Spipf8a0 = crate::Reg<spipf8a0::Spipf8a0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8a0;
#[doc = "SPIPF8A4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8a4`] module"]
#[doc(alias = "SPIPF8A4")]
pub type Spipf8a4 = crate::Reg<spipf8a4::Spipf8a4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8a4;
#[doc = "SPIPF8A8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8a8`] module"]
#[doc(alias = "SPIPF8A8")]
pub type Spipf8a8 = crate::Reg<spipf8a8::Spipf8a8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8a8;
#[doc = "SPIPF8AC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8ac`] module"]
#[doc(alias = "SPIPF8AC")]
pub type Spipf8ac = crate::Reg<spipf8ac::Spipf8acSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8ac;
#[doc = "SPIPF8B0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8b0`] module"]
#[doc(alias = "SPIPF8B0")]
pub type Spipf8b0 = crate::Reg<spipf8b0::Spipf8b0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8b0;
#[doc = "SPIPF8B4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8b4`] module"]
#[doc(alias = "SPIPF8B4")]
pub type Spipf8b4 = crate::Reg<spipf8b4::Spipf8b4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8b4;
#[doc = "SPIPF8B8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8b8`] module"]
#[doc(alias = "SPIPF8B8")]
pub type Spipf8b8 = crate::Reg<spipf8b8::Spipf8b8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8b8;
#[doc = "SPIPF8BC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8bc`] module"]
#[doc(alias = "SPIPF8BC")]
pub type Spipf8bc = crate::Reg<spipf8bc::Spipf8bcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8bc;
#[doc = "SPIPF8C0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8c0`] module"]
#[doc(alias = "SPIPF8C0")]
pub type Spipf8c0 = crate::Reg<spipf8c0::Spipf8c0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8c0;
#[doc = "SPIPF8C4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8c4`] module"]
#[doc(alias = "SPIPF8C4")]
pub type Spipf8c4 = crate::Reg<spipf8c4::Spipf8c4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8c4;
#[doc = "SPIPF8C8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8c8`] module"]
#[doc(alias = "SPIPF8C8")]
pub type Spipf8c8 = crate::Reg<spipf8c8::Spipf8c8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8c8;
#[doc = "SPIPF8CC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8cc`] module"]
#[doc(alias = "SPIPF8CC")]
pub type Spipf8cc = crate::Reg<spipf8cc::Spipf8ccSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8cc;
#[doc = "SPIPF8D0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8d0`] module"]
#[doc(alias = "SPIPF8D0")]
pub type Spipf8d0 = crate::Reg<spipf8d0::Spipf8d0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8d0;
#[doc = "SPIPF8D4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8d4`] module"]
#[doc(alias = "SPIPF8D4")]
pub type Spipf8d4 = crate::Reg<spipf8d4::Spipf8d4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8d4;
#[doc = "SPIPF8D8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8d8`] module"]
#[doc(alias = "SPIPF8D8")]
pub type Spipf8d8 = crate::Reg<spipf8d8::Spipf8d8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8d8;
#[doc = "SPIPF8DC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8dc`] module"]
#[doc(alias = "SPIPF8DC")]
pub type Spipf8dc = crate::Reg<spipf8dc::Spipf8dcSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8dc;
#[doc = "SPIPF8E0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8e0`] module"]
#[doc(alias = "SPIPF8E0")]
pub type Spipf8e0 = crate::Reg<spipf8e0::Spipf8e0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8e0;
#[doc = "SPIPF8E4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8e4`] module"]
#[doc(alias = "SPIPF8E4")]
pub type Spipf8e4 = crate::Reg<spipf8e4::Spipf8e4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8e4;
#[doc = "SPIPF8E8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8e8`] module"]
#[doc(alias = "SPIPF8E8")]
pub type Spipf8e8 = crate::Reg<spipf8e8::Spipf8e8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8e8;
#[doc = "SPIPF8EC (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8ec`] module"]
#[doc(alias = "SPIPF8EC")]
pub type Spipf8ec = crate::Reg<spipf8ec::Spipf8ecSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8ec;
#[doc = "SPIPF8F0 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8f0`] module"]
#[doc(alias = "SPIPF8F0")]
pub type Spipf8f0 = crate::Reg<spipf8f0::Spipf8f0Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8f0;
#[doc = "SPIPF8F4 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8f4`] module"]
#[doc(alias = "SPIPF8F4")]
pub type Spipf8f4 = crate::Reg<spipf8f4::Spipf8f4Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8f4;
#[doc = "SPIPF8F8 (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf8f8`] module"]
#[doc(alias = "SPIPF8F8")]
pub type Spipf8f8 = crate::Reg<spipf8f8::Spipf8f8Spec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipf8f8;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    secure000: Secure000,
    secure004: Secure004,
    secure008: Secure008,
    _reserved3: [u8; 0x04],
    secure010: Secure010,
    secure014: Secure014,
    secure018: Secure018,
    _reserved6: [u8; 0x04],
    secure020: Secure020,
    secure024: Secure024,
    secure028: Secure028,
    secure02c: Secure02c,
    secure030: Secure030,
    secure034: Secure034,
    secure038: Secure038,
    _reserved13: [u8; 0x04],
    secure040: Secure040,
    secure044: Secure044,
    secure048: Secure048,
    _reserved16: [u8; 0x04],
    secure050: Secure050,
    secure054: Secure054,
    secure058: Secure058,
    secure05c: Secure05c,
    secure060: Secure060,
    secure064: Secure064,
    secure068: Secure068,
    secure06c: Secure06c,
    secure070: Secure070,
    _reserved25: [u8; 0x04],
    secure078: Secure078,
    _reserved26: [u8; 0x04],
    secure080: Secure080,
    secure084: Secure084,
    secure088: Secure088,
    secure08c: Secure08c,
    secure090: Secure090,
    secure094: Secure094,
    secure098: Secure098,
    _reserved33: [u8; 0x14],
    secure0b0: Secure0b0,
    secure0b4: Secure0b4,
    _reserved35: [u8; 0x04],
    secure0bc: Secure0bc,
    secure0c0: Secure0c0,
    secure0c4: Secure0c4,
    _reserved38: [u8; 0x0738],
    secure800: Secure800,
    _reserved39: [u8; 0x04],
    secure808: Secure808,
    secure80c: Secure80c,
    secure810: Secure810,
    secure814: Secure814,
    _reserved43: [u8; 0x08],
    secure820: Secure820,
    _reserved44: [u8; 0x1c],
    secure840: Secure840,
    secure844: Secure844,
    secure848: Secure848,
    _reserved47: [u8; 0x08],
    secure854: Secure854,
    secure858: Secure858,
    secure85c: Secure85c,
    secure860: Secure860,
    secure864: Secure864,
    secure868: Secure868,
    _reserved53: [u8; 0x04],
    secure870: Secure870,
    secure874: Secure874,
    secure878: Secure878,
    secure87c: Secure87c,
    secure880: Secure880,
    secure884: Secure884,
    secure888: Secure888,
    secure88c: Secure88c,
    secure890: Secure890,
    _reserved62: [u8; 0x0c],
    secure8a0: Secure8a0,
    secure8a4: Secure8a4,
    secure8a8: Secure8a8,
    secure8ac: Secure8ac,
    secure8b0: Secure8b0,
    secure8b4: Secure8b4,
    secure8b8: Secure8b8,
    secure8bc: Secure8bc,
    _reserved70: [u8; 0x40],
    secure900: Secure900,
    secure904: Secure904,
    secure908: Secure908,
    secure90c: Secure90c,
    secure910: Secure910,
    secure914: Secure914,
    secure918: Secure918,
    secure91c: Secure91c,
    secure920: Secure920,
    secure924: Secure924,
    secure928: Secure928,
    secure92c: Secure92c,
    secure930: Secure930,
    secure934: Secure934,
    secure938: Secure938,
    secure93c: Secure93c,
    _reserved86: [u8; 0x40],
    secure980: Secure980,
    secure984: Secure984,
    secure988: Secure988,
    secure98c: Secure98c,
    secure990: Secure990,
    secure994: Secure994,
    secure998: Secure998,
    secure99c: Secure99c,
    secure9a0: Secure9a0,
    secure9a4: Secure9a4,
    secure9a8: Secure9a8,
    secure9ac: Secure9ac,
    secure9b0: Secure9b0,
    secure9b4: Secure9b4,
    secure9b8: Secure9b8,
    secure9bc: Secure9bc,
    _reserved102: [u8; 0x40],
    securea00: Securea00,
    securea04: Securea04,
    securea08: Securea08,
    securea0c: Securea0c,
    securea10: Securea10,
    securea14: Securea14,
    securea18: Securea18,
    securea1c: Securea1c,
    securea20: Securea20,
    securea24: Securea24,
    securea28: Securea28,
    securea2c: Securea2c,
    securea30: Securea30,
    securea34: Securea34,
    securea38: Securea38,
    securea3c: Securea3c,
    securea40: Securea40,
    securea44: Securea44,
    securea48: Securea48,
    securea4c: Securea4c,
    securea50: Securea50,
    securea54: Securea54,
    securea58: Securea58,
    securea5c: Securea5c,
    securea60: Securea60,
    securea64: Securea64,
    securea68: Securea68,
    securea6c: Securea6c,
    securea70: Securea70,
    securea74: Securea74,
    securea78: Securea78,
    securea7c: Securea7c,
    securea80: Securea80,
    securea84: Securea84,
    securea88: Securea88,
    securea8c: Securea8c,
    securea90: Securea90,
    securea94: Securea94,
    securea98: Securea98,
    securea9c: Securea9c,
    secureaa0: Secureaa0,
    secureaa4: Secureaa4,
    secureaa8: Secureaa8,
    secureaac: Secureaac,
    secureab0: Secureab0,
    secureab4: Secureab4,
    secureab8: Secureab8,
    secureabc: Secureabc,
    secureac0: Secureac0,
    secureac4: Secureac4,
    secureac8: Secureac8,
    secureacc: Secureacc,
    securead0: Securead0,
    securead4: Securead4,
    securead8: Securead8,
    secureadc: Secureadc,
    secureae0: Secureae0,
    secureae4: Secureae4,
    secureae8: Secureae8,
    secureaec: Secureaec,
}
impl RegisterBlock {
    #[doc = "0x00 - Protection Key Register"]
    #[inline(always)]
    pub const fn secure000(&self) -> &Secure000 {
        &self.secure000
    }
    #[doc = "0x04 - OTP Command Trigger Register"]
    #[inline(always)]
    pub const fn secure004(&self) -> &Secure004 {
        &self.secure004
    }
    #[doc = "0x08 - OTP Timing Register"]
    #[inline(always)]
    pub const fn secure008(&self) -> &Secure008 {
        &self.secure008
    }
    #[doc = "0x10 - OTP Address Register"]
    #[inline(always)]
    pub const fn secure010(&self) -> &Secure010 {
        &self.secure010
    }
    #[doc = "0x14 - Secure Engine Status Register"]
    #[inline(always)]
    pub const fn secure014(&self) -> &Secure014 {
        &self.secure014
    }
    #[doc = "0x18 - OTP Programming Status Register"]
    #[inline(always)]
    pub const fn secure018(&self) -> &Secure018 {
        &self.secure018
    }
    #[doc = "0x20 - OTP Data Compare Register 1"]
    #[inline(always)]
    pub const fn secure020(&self) -> &Secure020 {
        &self.secure020
    }
    #[doc = "0x24 - OTP Data Compare Register 2"]
    #[inline(always)]
    pub const fn secure024(&self) -> &Secure024 {
        &self.secure024
    }
    #[doc = "0x28 - OTP Data Compare Register 3"]
    #[inline(always)]
    pub const fn secure028(&self) -> &Secure028 {
        &self.secure028
    }
    #[doc = "0x2c - OTP Data Compare Register 4"]
    #[inline(always)]
    pub const fn secure02c(&self) -> &Secure02c {
        &self.secure02c
    }
    #[doc = "0x30 - OTPTRAP data read back 1"]
    #[inline(always)]
    pub const fn secure030(&self) -> &Secure030 {
        &self.secure030
    }
    #[doc = "0x34 - OTPTRAP data read back 2"]
    #[inline(always)]
    pub const fn secure034(&self) -> &Secure034 {
        &self.secure034
    }
    #[doc = "0x38 - OTP QRR data read back"]
    #[inline(always)]
    pub const fn secure038(&self) -> &Secure038 {
        &self.secure038
    }
    #[doc = "0x40 - OTP QSR data read back"]
    #[inline(always)]
    pub const fn secure040(&self) -> &Secure040 {
        &self.secure040
    }
    #[doc = "0x44 - OTP QMR data read back"]
    #[inline(always)]
    pub const fn secure044(&self) -> &Secure044 {
        &self.secure044
    }
    #[doc = "0x48 - OTP QMRA and QMRB data read back"]
    #[inline(always)]
    pub const fn secure048(&self) -> &Secure048 {
        &self.secure048
    }
    #[doc = "0x50 - Extra Programming Protection Range Register"]
    #[inline(always)]
    pub const fn secure050(&self) -> &Secure050 {
        &self.secure050
    }
    #[doc = "0x54 - Extra Read Protection Range Register"]
    #[inline(always)]
    pub const fn secure054(&self) -> &Secure054 {
        &self.secure054
    }
    #[doc = "0x58 - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub const fn secure058(&self) -> &Secure058 {
        &self.secure058
    }
    #[doc = "0x5c - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub const fn secure05c(&self) -> &Secure05c {
        &self.secure05c
    }
    #[doc = "0x60 - Secure Boot Hardware Revision Register 1"]
    #[inline(always)]
    pub const fn secure060(&self) -> &Secure060 {
        &self.secure060
    }
    #[doc = "0x64 - Secure Boot Hardware Revision Register 2"]
    #[inline(always)]
    pub const fn secure064(&self) -> &Secure064 {
        &self.secure064
    }
    #[doc = "0x68 - Secure Boot Software Revision Register 1"]
    #[inline(always)]
    pub const fn secure068(&self) -> &Secure068 {
        &self.secure068
    }
    #[doc = "0x6c - Secure Boot Software Revision Register 2"]
    #[inline(always)]
    pub const fn secure06c(&self) -> &Secure06c {
        &self.secure06c
    }
    #[doc = "0x70 - Secure Boot from SPI Status Register"]
    #[inline(always)]
    pub const fn secure070(&self) -> &Secure070 {
        &self.secure070
    }
    #[doc = "0x78 - Secure Boot Key Number Register"]
    #[inline(always)]
    pub const fn secure078(&self) -> &Secure078 {
        &self.secure078
    }
    #[doc = "0x80 - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub const fn secure080(&self) -> &Secure080 {
        &self.secure080
    }
    #[doc = "0x84 - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub const fn secure084(&self) -> &Secure084 {
        &self.secure084
    }
    #[doc = "0x88 - Software ECC Control Register"]
    #[inline(always)]
    pub const fn secure088(&self) -> &Secure088 {
        &self.secure088
    }
    #[doc = "0x8c - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub const fn secure08c(&self) -> &Secure08c {
        &self.secure08c
    }
    #[doc = "0x90 - Secure Boot Counter Register"]
    #[inline(always)]
    pub const fn secure090(&self) -> &Secure090 {
        &self.secure090
    }
    #[doc = "0x94 - Secure Boot Counter 2 Register"]
    #[inline(always)]
    pub const fn secure094(&self) -> &Secure094 {
        &self.secure094
    }
    #[doc = "0x98 - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub const fn secure098(&self) -> &Secure098 {
        &self.secure098
    }
    #[doc = "0xb0 - Secure Boot RSA Engine Controller Register"]
    #[inline(always)]
    pub const fn secure0b0(&self) -> &Secure0b0 {
        &self.secure0b0
    }
    #[doc = "0xb4 - Secure Boot ECC Engine Controller Register"]
    #[inline(always)]
    pub const fn secure0b4(&self) -> &Secure0b4 {
        &self.secure0b4
    }
    #[doc = "0xbc - Secure Boot Engine Trigger Register"]
    #[inline(always)]
    pub const fn secure0bc(&self) -> &Secure0bc {
        &self.secure0bc
    }
    #[doc = "0xc0 - Secure Boot Engine Interrupt Controller Register"]
    #[inline(always)]
    pub const fn secure0c0(&self) -> &Secure0c0 {
        &self.secure0c0
    }
    #[doc = "0xc4 - Secure Boot Engine Interrupt Status Register"]
    #[inline(always)]
    pub const fn secure0c4(&self) -> &Secure0c4 {
        &self.secure0c4
    }
    #[doc = "0x800 - Secure Boot DMA Enable Register"]
    #[inline(always)]
    pub const fn secure800(&self) -> &Secure800 {
        &self.secure800
    }
    #[doc = "0x808 - Secure Boot DMA Mode Register"]
    #[inline(always)]
    pub const fn secure808(&self) -> &Secure808 {
        &self.secure808
    }
    #[doc = "0x80c - Secure Boot Vault Key Control Register"]
    #[inline(always)]
    pub const fn secure80c(&self) -> &Secure80c {
        &self.secure80c
    }
    #[doc = "0x810 - Secure Boot Digest Status"]
    #[inline(always)]
    pub const fn secure810(&self) -> &Secure810 {
        &self.secure810
    }
    #[doc = "0x814 - Secure Boot Digest Check Status"]
    #[inline(always)]
    pub const fn secure814(&self) -> &Secure814 {
        &self.secure814
    }
    #[doc = "0x820 - Secure Crypto Engine Enable Register"]
    #[inline(always)]
    pub const fn secure820(&self) -> &Secure820 {
        &self.secure820
    }
    #[doc = "0x840 - Secure Boot DMA Source Address Register"]
    #[inline(always)]
    pub const fn secure840(&self) -> &Secure840 {
        &self.secure840
    }
    #[doc = "0x844 - Secure Boot DMA Destination Address Register"]
    #[inline(always)]
    pub const fn secure844(&self) -> &Secure844 {
        &self.secure844
    }
    #[doc = "0x848 - Secure Boot DMA Size Register"]
    #[inline(always)]
    pub const fn secure848(&self) -> &Secure848 {
        &self.secure848
    }
    #[doc = "0x854 - Secure Boot Hash Size Register"]
    #[inline(always)]
    pub const fn secure854(&self) -> &Secure854 {
        &self.secure854
    }
    #[doc = "0x858 - Secure Boot Hash Mode Register"]
    #[inline(always)]
    pub const fn secure858(&self) -> &Secure858 {
        &self.secure858
    }
    #[doc = "0x85c - Secure Boot Hash Engine Fire Register"]
    #[inline(always)]
    pub const fn secure85c(&self) -> &Secure85c {
        &self.secure85c
    }
    #[doc = "0x860 - Secure Boot Crypto Mode Register"]
    #[inline(always)]
    pub const fn secure860(&self) -> &Secure860 {
        &self.secure860
    }
    #[doc = "0x864 - Secure Boot Crypto Data Size Register"]
    #[inline(always)]
    pub const fn secure864(&self) -> &Secure864 {
        &self.secure864
    }
    #[doc = "0x868 - Secure Boot Crypto Data Total Size Register"]
    #[inline(always)]
    pub const fn secure868(&self) -> &Secure868 {
        &self.secure868
    }
    #[doc = "0x870 - Secure Boot Crypto Low Key Write Trigger Register"]
    #[inline(always)]
    pub const fn secure870(&self) -> &Secure870 {
        &self.secure870
    }
    #[doc = "0x874 - Secure Boot Crypto High Key Write Trigger Register"]
    #[inline(always)]
    pub const fn secure874(&self) -> &Secure874 {
        &self.secure874
    }
    #[doc = "0x878 - Secure Boot Crypto Vector Write Trigger Register"]
    #[inline(always)]
    pub const fn secure878(&self) -> &Secure878 {
        &self.secure878
    }
    #[doc = "0x87c - Secure Boot Crypto Engine Fire Register"]
    #[inline(always)]
    pub const fn secure87c(&self) -> &Secure87c {
        &self.secure87c
    }
    #[doc = "0x880 - Secure Boot Crypto Data Buffer 0 Register"]
    #[inline(always)]
    pub const fn secure880(&self) -> &Secure880 {
        &self.secure880
    }
    #[doc = "0x884 - Secure Boot Crypto Data Buffer 1 Register"]
    #[inline(always)]
    pub const fn secure884(&self) -> &Secure884 {
        &self.secure884
    }
    #[doc = "0x888 - Secure Boot Crypto Data Buffer 2 Register"]
    #[inline(always)]
    pub const fn secure888(&self) -> &Secure888 {
        &self.secure888
    }
    #[doc = "0x88c - Secure Boot Crypto Data Buffer 3 Register"]
    #[inline(always)]
    pub const fn secure88c(&self) -> &Secure88c {
        &self.secure88c
    }
    #[doc = "0x890 - Secure Boot Crypto AES-GCM GHash Key Write Trigger Register"]
    #[inline(always)]
    pub const fn secure890(&self) -> &Secure890 {
        &self.secure890
    }
    #[doc = "0x8a0 - Secure Boot Crypto Key Buffer 0 Register"]
    #[inline(always)]
    pub const fn secure8a0(&self) -> &Secure8a0 {
        &self.secure8a0
    }
    #[doc = "0x8a4 - Secure Boot Crypto Key Buffer 1 Register"]
    #[inline(always)]
    pub const fn secure8a4(&self) -> &Secure8a4 {
        &self.secure8a4
    }
    #[doc = "0x8a8 - Secure Boot Crypto Key Buffer 2 Register"]
    #[inline(always)]
    pub const fn secure8a8(&self) -> &Secure8a8 {
        &self.secure8a8
    }
    #[doc = "0x8ac - Secure Boot Crypto Key Buffer 3 Register"]
    #[inline(always)]
    pub const fn secure8ac(&self) -> &Secure8ac {
        &self.secure8ac
    }
    #[doc = "0x8b0 - Secure Boot Crypto Key Buffer 4 Register"]
    #[inline(always)]
    pub const fn secure8b0(&self) -> &Secure8b0 {
        &self.secure8b0
    }
    #[doc = "0x8b4 - Secure Boot Crypto Key Buffer 5 Register"]
    #[inline(always)]
    pub const fn secure8b4(&self) -> &Secure8b4 {
        &self.secure8b4
    }
    #[doc = "0x8b8 - Secure Boot Crypto Key Buffer 6 Register"]
    #[inline(always)]
    pub const fn secure8b8(&self) -> &Secure8b8 {
        &self.secure8b8
    }
    #[doc = "0x8bc - Secure Boot Crypto Key Buffer 7 Register"]
    #[inline(always)]
    pub const fn secure8bc(&self) -> &Secure8bc {
        &self.secure8bc
    }
    #[doc = "0x900 - Secure Boot First Vault Key 0 Register"]
    #[inline(always)]
    pub const fn secure900(&self) -> &Secure900 {
        &self.secure900
    }
    #[doc = "0x904 - Secure Boot First Vault Key 1 Register"]
    #[inline(always)]
    pub const fn secure904(&self) -> &Secure904 {
        &self.secure904
    }
    #[doc = "0x908 - Secure Boot First Vault Key 2 Register"]
    #[inline(always)]
    pub const fn secure908(&self) -> &Secure908 {
        &self.secure908
    }
    #[doc = "0x90c - Secure Boot First Vault Key 3 Register"]
    #[inline(always)]
    pub const fn secure90c(&self) -> &Secure90c {
        &self.secure90c
    }
    #[doc = "0x910 - Secure Boot First Vault Key 4 Register"]
    #[inline(always)]
    pub const fn secure910(&self) -> &Secure910 {
        &self.secure910
    }
    #[doc = "0x914 - Secure Boot First Vault Key 5 Register"]
    #[inline(always)]
    pub const fn secure914(&self) -> &Secure914 {
        &self.secure914
    }
    #[doc = "0x918 - Secure Boot First Vault Key 6 Register"]
    #[inline(always)]
    pub const fn secure918(&self) -> &Secure918 {
        &self.secure918
    }
    #[doc = "0x91c - Secure Boot First Vault Key 7 Register"]
    #[inline(always)]
    pub const fn secure91c(&self) -> &Secure91c {
        &self.secure91c
    }
    #[doc = "0x920 - Secure Boot Second Vault Key 0 Register"]
    #[inline(always)]
    pub const fn secure920(&self) -> &Secure920 {
        &self.secure920
    }
    #[doc = "0x924 - Secure Boot Second Vault Key 1 Register"]
    #[inline(always)]
    pub const fn secure924(&self) -> &Secure924 {
        &self.secure924
    }
    #[doc = "0x928 - Secure Boot Second Vault Key 2 Register"]
    #[inline(always)]
    pub const fn secure928(&self) -> &Secure928 {
        &self.secure928
    }
    #[doc = "0x92c - Secure Boot Second Vault Key 3 Register"]
    #[inline(always)]
    pub const fn secure92c(&self) -> &Secure92c {
        &self.secure92c
    }
    #[doc = "0x930 - Secure Boot Second Vault Key 4 Register"]
    #[inline(always)]
    pub const fn secure930(&self) -> &Secure930 {
        &self.secure930
    }
    #[doc = "0x934 - Secure Boot Second Vault Key 5 Register"]
    #[inline(always)]
    pub const fn secure934(&self) -> &Secure934 {
        &self.secure934
    }
    #[doc = "0x938 - Secure Boot Second Vault Key 6 Register"]
    #[inline(always)]
    pub const fn secure938(&self) -> &Secure938 {
        &self.secure938
    }
    #[doc = "0x93c - Secure Boot Second Vault Key 7 Register"]
    #[inline(always)]
    pub const fn secure93c(&self) -> &Secure93c {
        &self.secure93c
    }
    #[doc = "0x980 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure980(&self) -> &Secure980 {
        &self.secure980
    }
    #[doc = "0x984 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure984(&self) -> &Secure984 {
        &self.secure984
    }
    #[doc = "0x988 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure988(&self) -> &Secure988 {
        &self.secure988
    }
    #[doc = "0x98c - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure98c(&self) -> &Secure98c {
        &self.secure98c
    }
    #[doc = "0x990 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure990(&self) -> &Secure990 {
        &self.secure990
    }
    #[doc = "0x994 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure994(&self) -> &Secure994 {
        &self.secure994
    }
    #[doc = "0x998 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure998(&self) -> &Secure998 {
        &self.secure998
    }
    #[doc = "0x99c - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure99c(&self) -> &Secure99c {
        &self.secure99c
    }
    #[doc = "0x9a0 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9a0(&self) -> &Secure9a0 {
        &self.secure9a0
    }
    #[doc = "0x9a4 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9a4(&self) -> &Secure9a4 {
        &self.secure9a4
    }
    #[doc = "0x9a8 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9a8(&self) -> &Secure9a8 {
        &self.secure9a8
    }
    #[doc = "0x9ac - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9ac(&self) -> &Secure9ac {
        &self.secure9ac
    }
    #[doc = "0x9b0 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9b0(&self) -> &Secure9b0 {
        &self.secure9b0
    }
    #[doc = "0x9b4 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9b4(&self) -> &Secure9b4 {
        &self.secure9b4
    }
    #[doc = "0x9b8 - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9b8(&self) -> &Secure9b8 {
        &self.secure9b8
    }
    #[doc = "0x9bc - Secure Boot Image Digest Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secure9bc(&self) -> &Secure9bc {
        &self.secure9bc
    }
    #[doc = "0xa00 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea00(&self) -> &Securea00 {
        &self.securea00
    }
    #[doc = "0xa04 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea04(&self) -> &Securea04 {
        &self.securea04
    }
    #[doc = "0xa08 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea08(&self) -> &Securea08 {
        &self.securea08
    }
    #[doc = "0xa0c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea0c(&self) -> &Securea0c {
        &self.securea0c
    }
    #[doc = "0xa10 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea10(&self) -> &Securea10 {
        &self.securea10
    }
    #[doc = "0xa14 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea14(&self) -> &Securea14 {
        &self.securea14
    }
    #[doc = "0xa18 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea18(&self) -> &Securea18 {
        &self.securea18
    }
    #[doc = "0xa1c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea1c(&self) -> &Securea1c {
        &self.securea1c
    }
    #[doc = "0xa20 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea20(&self) -> &Securea20 {
        &self.securea20
    }
    #[doc = "0xa24 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea24(&self) -> &Securea24 {
        &self.securea24
    }
    #[doc = "0xa28 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea28(&self) -> &Securea28 {
        &self.securea28
    }
    #[doc = "0xa2c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea2c(&self) -> &Securea2c {
        &self.securea2c
    }
    #[doc = "0xa30 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea30(&self) -> &Securea30 {
        &self.securea30
    }
    #[doc = "0xa34 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea34(&self) -> &Securea34 {
        &self.securea34
    }
    #[doc = "0xa38 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea38(&self) -> &Securea38 {
        &self.securea38
    }
    #[doc = "0xa3c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea3c(&self) -> &Securea3c {
        &self.securea3c
    }
    #[doc = "0xa40 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea40(&self) -> &Securea40 {
        &self.securea40
    }
    #[doc = "0xa44 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea44(&self) -> &Securea44 {
        &self.securea44
    }
    #[doc = "0xa48 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea48(&self) -> &Securea48 {
        &self.securea48
    }
    #[doc = "0xa4c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea4c(&self) -> &Securea4c {
        &self.securea4c
    }
    #[doc = "0xa50 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea50(&self) -> &Securea50 {
        &self.securea50
    }
    #[doc = "0xa54 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea54(&self) -> &Securea54 {
        &self.securea54
    }
    #[doc = "0xa58 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea58(&self) -> &Securea58 {
        &self.securea58
    }
    #[doc = "0xa5c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea5c(&self) -> &Securea5c {
        &self.securea5c
    }
    #[doc = "0xa60 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea60(&self) -> &Securea60 {
        &self.securea60
    }
    #[doc = "0xa64 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea64(&self) -> &Securea64 {
        &self.securea64
    }
    #[doc = "0xa68 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea68(&self) -> &Securea68 {
        &self.securea68
    }
    #[doc = "0xa6c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea6c(&self) -> &Securea6c {
        &self.securea6c
    }
    #[doc = "0xa70 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea70(&self) -> &Securea70 {
        &self.securea70
    }
    #[doc = "0xa74 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea74(&self) -> &Securea74 {
        &self.securea74
    }
    #[doc = "0xa78 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea78(&self) -> &Securea78 {
        &self.securea78
    }
    #[doc = "0xa7c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea7c(&self) -> &Securea7c {
        &self.securea7c
    }
    #[doc = "0xa80 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea80(&self) -> &Securea80 {
        &self.securea80
    }
    #[doc = "0xa84 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea84(&self) -> &Securea84 {
        &self.securea84
    }
    #[doc = "0xa88 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea88(&self) -> &Securea88 {
        &self.securea88
    }
    #[doc = "0xa8c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea8c(&self) -> &Securea8c {
        &self.securea8c
    }
    #[doc = "0xa90 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea90(&self) -> &Securea90 {
        &self.securea90
    }
    #[doc = "0xa94 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea94(&self) -> &Securea94 {
        &self.securea94
    }
    #[doc = "0xa98 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea98(&self) -> &Securea98 {
        &self.securea98
    }
    #[doc = "0xa9c - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securea9c(&self) -> &Securea9c {
        &self.securea9c
    }
    #[doc = "0xaa0 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureaa0(&self) -> &Secureaa0 {
        &self.secureaa0
    }
    #[doc = "0xaa4 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureaa4(&self) -> &Secureaa4 {
        &self.secureaa4
    }
    #[doc = "0xaa8 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureaa8(&self) -> &Secureaa8 {
        &self.secureaa8
    }
    #[doc = "0xaac - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureaac(&self) -> &Secureaac {
        &self.secureaac
    }
    #[doc = "0xab0 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureab0(&self) -> &Secureab0 {
        &self.secureab0
    }
    #[doc = "0xab4 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureab4(&self) -> &Secureab4 {
        &self.secureab4
    }
    #[doc = "0xab8 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureab8(&self) -> &Secureab8 {
        &self.secureab8
    }
    #[doc = "0xabc - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureabc(&self) -> &Secureabc {
        &self.secureabc
    }
    #[doc = "0xac0 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureac0(&self) -> &Secureac0 {
        &self.secureac0
    }
    #[doc = "0xac4 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureac4(&self) -> &Secureac4 {
        &self.secureac4
    }
    #[doc = "0xac8 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureac8(&self) -> &Secureac8 {
        &self.secureac8
    }
    #[doc = "0xacc - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureacc(&self) -> &Secureacc {
        &self.secureacc
    }
    #[doc = "0xad0 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securead0(&self) -> &Securead0 {
        &self.securead0
    }
    #[doc = "0xad4 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securead4(&self) -> &Securead4 {
        &self.securead4
    }
    #[doc = "0xad8 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn securead8(&self) -> &Securead8 {
        &self.securead8
    }
    #[doc = "0xadc - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureadc(&self) -> &Secureadc {
        &self.secureadc
    }
    #[doc = "0xae0 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureae0(&self) -> &Secureae0 {
        &self.secureae0
    }
    #[doc = "0xae4 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureae4(&self) -> &Secureae4 {
        &self.secureae4
    }
    #[doc = "0xae8 - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureae8(&self) -> &Secureae8 {
        &self.secureae8
    }
    #[doc = "0xaec - Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
    #[inline(always)]
    pub const fn secureaec(&self) -> &Secureaec {
        &self.secureaec
    }
}
#[doc = "SECURE000 (rw) register accessor: Protection Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure000`] module"]
#[doc(alias = "SECURE000")]
pub type Secure000 = crate::Reg<secure000::Secure000Spec>;
#[doc = "Protection Key Register"]
pub mod secure000;
#[doc = "SECURE004 (rw) register accessor: OTP Command Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure004`] module"]
#[doc(alias = "SECURE004")]
pub type Secure004 = crate::Reg<secure004::Secure004Spec>;
#[doc = "OTP Command Trigger Register"]
pub mod secure004;
#[doc = "SECURE008 (rw) register accessor: OTP Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure008`] module"]
#[doc(alias = "SECURE008")]
pub type Secure008 = crate::Reg<secure008::Secure008Spec>;
#[doc = "OTP Timing Register"]
pub mod secure008;
#[doc = "SECURE010 (rw) register accessor: OTP Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure010`] module"]
#[doc(alias = "SECURE010")]
pub type Secure010 = crate::Reg<secure010::Secure010Spec>;
#[doc = "OTP Address Register"]
pub mod secure010;
#[doc = "SECURE014 (rw) register accessor: Secure Engine Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure014`] module"]
#[doc(alias = "SECURE014")]
pub type Secure014 = crate::Reg<secure014::Secure014Spec>;
#[doc = "Secure Engine Status Register"]
pub mod secure014;
#[doc = "SECURE018 (rw) register accessor: OTP Programming Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure018`] module"]
#[doc(alias = "SECURE018")]
pub type Secure018 = crate::Reg<secure018::Secure018Spec>;
#[doc = "OTP Programming Status Register"]
pub mod secure018;
#[doc = "SECURE020 (rw) register accessor: OTP Data Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure020`] module"]
#[doc(alias = "SECURE020")]
pub type Secure020 = crate::Reg<secure020::Secure020Spec>;
#[doc = "OTP Data Compare Register 1"]
pub mod secure020;
#[doc = "SECURE024 (rw) register accessor: OTP Data Compare Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure024`] module"]
#[doc(alias = "SECURE024")]
pub type Secure024 = crate::Reg<secure024::Secure024Spec>;
#[doc = "OTP Data Compare Register 2"]
pub mod secure024;
#[doc = "SECURE028 (rw) register accessor: OTP Data Compare Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`secure028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure028`] module"]
#[doc(alias = "SECURE028")]
pub type Secure028 = crate::Reg<secure028::Secure028Spec>;
#[doc = "OTP Data Compare Register 3"]
pub mod secure028;
#[doc = "SECURE02C (rw) register accessor: OTP Data Compare Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`secure02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure02c`] module"]
#[doc(alias = "SECURE02C")]
pub type Secure02c = crate::Reg<secure02c::Secure02cSpec>;
#[doc = "OTP Data Compare Register 4"]
pub mod secure02c;
#[doc = "SECURE030 (rw) register accessor: OTPTRAP data read back 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure030`] module"]
#[doc(alias = "SECURE030")]
pub type Secure030 = crate::Reg<secure030::Secure030Spec>;
#[doc = "OTPTRAP data read back 1"]
pub mod secure030;
#[doc = "SECURE034 (rw) register accessor: OTPTRAP data read back 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure034`] module"]
#[doc(alias = "SECURE034")]
pub type Secure034 = crate::Reg<secure034::Secure034Spec>;
#[doc = "OTPTRAP data read back 2"]
pub mod secure034;
#[doc = "SECURE038 (rw) register accessor: OTP QRR data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure038`] module"]
#[doc(alias = "SECURE038")]
pub type Secure038 = crate::Reg<secure038::Secure038Spec>;
#[doc = "OTP QRR data read back"]
pub mod secure038;
#[doc = "SECURE040 (rw) register accessor: OTP QSR data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure040`] module"]
#[doc(alias = "SECURE040")]
pub type Secure040 = crate::Reg<secure040::Secure040Spec>;
#[doc = "OTP QSR data read back"]
pub mod secure040;
#[doc = "SECURE044 (rw) register accessor: OTP QMR data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure044`] module"]
#[doc(alias = "SECURE044")]
pub type Secure044 = crate::Reg<secure044::Secure044Spec>;
#[doc = "OTP QMR data read back"]
pub mod secure044;
#[doc = "SECURE048 (rw) register accessor: OTP QMRA and QMRB data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure048`] module"]
#[doc(alias = "SECURE048")]
pub type Secure048 = crate::Reg<secure048::Secure048Spec>;
#[doc = "OTP QMRA and QMRB data read back"]
pub mod secure048;
#[doc = "SECURE050 (rw) register accessor: Extra Programming Protection Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure050`] module"]
#[doc(alias = "SECURE050")]
pub type Secure050 = crate::Reg<secure050::Secure050Spec>;
#[doc = "Extra Programming Protection Range Register"]
pub mod secure050;
#[doc = "SECURE054 (rw) register accessor: Extra Read Protection Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure054`] module"]
#[doc(alias = "SECURE054")]
pub type Secure054 = crate::Reg<secure054::Secure054Spec>;
#[doc = "Extra Read Protection Range Register"]
pub mod secure054;
#[doc = "SECURE058 (rw) register accessor: Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure058`] module"]
#[doc(alias = "SECURE058")]
pub type Secure058 = crate::Reg<secure058::Secure058Spec>;
#[doc = "Secure Boot Engine Internal Controller Register"]
pub mod secure058;
#[doc = "SECURE05C (rw) register accessor: Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure05c`] module"]
#[doc(alias = "SECURE05C")]
pub type Secure05c = crate::Reg<secure05c::Secure05cSpec>;
#[doc = "Secure Boot Engine Internal Controller Register"]
pub mod secure05c;
#[doc = "SECURE060 (rw) register accessor: Secure Boot Hardware Revision Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure060`] module"]
#[doc(alias = "SECURE060")]
pub type Secure060 = crate::Reg<secure060::Secure060Spec>;
#[doc = "Secure Boot Hardware Revision Register 1"]
pub mod secure060;
#[doc = "SECURE064 (rw) register accessor: Secure Boot Hardware Revision Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure064`] module"]
#[doc(alias = "SECURE064")]
pub type Secure064 = crate::Reg<secure064::Secure064Spec>;
#[doc = "Secure Boot Hardware Revision Register 2"]
pub mod secure064;
#[doc = "SECURE068 (rw) register accessor: Secure Boot Software Revision Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`secure068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure068`] module"]
#[doc(alias = "SECURE068")]
pub type Secure068 = crate::Reg<secure068::Secure068Spec>;
#[doc = "Secure Boot Software Revision Register 1"]
pub mod secure068;
#[doc = "SECURE06C (rw) register accessor: Secure Boot Software Revision Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure06c`] module"]
#[doc(alias = "SECURE06C")]
pub type Secure06c = crate::Reg<secure06c::Secure06cSpec>;
#[doc = "Secure Boot Software Revision Register 2"]
pub mod secure06c;
#[doc = "SECURE070 (rw) register accessor: Secure Boot from SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure070`] module"]
#[doc(alias = "SECURE070")]
pub type Secure070 = crate::Reg<secure070::Secure070Spec>;
#[doc = "Secure Boot from SPI Status Register"]
pub mod secure070;
#[doc = "SECURE078 (rw) register accessor: Secure Boot Key Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure078`] module"]
#[doc(alias = "SECURE078")]
pub type Secure078 = crate::Reg<secure078::Secure078Spec>;
#[doc = "Secure Boot Key Number Register"]
pub mod secure078;
#[doc = "SECURE080 (rw) register accessor: Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure080`] module"]
#[doc(alias = "SECURE080")]
pub type Secure080 = crate::Reg<secure080::Secure080Spec>;
#[doc = "Secure Boot Engine Internal Controller Register"]
pub mod secure080;
#[doc = "SECURE084 (rw) register accessor: Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure084`] module"]
#[doc(alias = "SECURE084")]
pub type Secure084 = crate::Reg<secure084::Secure084Spec>;
#[doc = "Secure Boot Engine Internal Controller Register"]
pub mod secure084;
#[doc = "SECURE088 (rw) register accessor: Software ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure088`] module"]
#[doc(alias = "SECURE088")]
pub type Secure088 = crate::Reg<secure088::Secure088Spec>;
#[doc = "Software ECC Control Register"]
pub mod secure088;
#[doc = "SECURE08C (rw) register accessor: Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure08c`] module"]
#[doc(alias = "SECURE08C")]
pub type Secure08c = crate::Reg<secure08c::Secure08cSpec>;
#[doc = "Secure Boot Engine Internal Controller Register"]
pub mod secure08c;
#[doc = "SECURE090 (rw) register accessor: Secure Boot Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure090`] module"]
#[doc(alias = "SECURE090")]
pub type Secure090 = crate::Reg<secure090::Secure090Spec>;
#[doc = "Secure Boot Counter Register"]
pub mod secure090;
#[doc = "SECURE094 (rw) register accessor: Secure Boot Counter 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure094`] module"]
#[doc(alias = "SECURE094")]
pub type Secure094 = crate::Reg<secure094::Secure094Spec>;
#[doc = "Secure Boot Counter 2 Register"]
pub mod secure094;
#[doc = "SECURE098 (rw) register accessor: Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure098`] module"]
#[doc(alias = "SECURE098")]
pub type Secure098 = crate::Reg<secure098::Secure098Spec>;
#[doc = "Secure Boot Engine Internal Controller Register"]
pub mod secure098;
#[doc = "SECURE0B0 (rw) register accessor: Secure Boot RSA Engine Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure0b0`] module"]
#[doc(alias = "SECURE0B0")]
pub type Secure0b0 = crate::Reg<secure0b0::Secure0b0Spec>;
#[doc = "Secure Boot RSA Engine Controller Register"]
pub mod secure0b0;
#[doc = "SECURE0B4 (rw) register accessor: Secure Boot ECC Engine Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure0b4`] module"]
#[doc(alias = "SECURE0B4")]
pub type Secure0b4 = crate::Reg<secure0b4::Secure0b4Spec>;
#[doc = "Secure Boot ECC Engine Controller Register"]
pub mod secure0b4;
#[doc = "SECURE0BC (rw) register accessor: Secure Boot Engine Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure0bc`] module"]
#[doc(alias = "SECURE0BC")]
pub type Secure0bc = crate::Reg<secure0bc::Secure0bcSpec>;
#[doc = "Secure Boot Engine Trigger Register"]
pub mod secure0bc;
#[doc = "SECURE0C0 (rw) register accessor: Secure Boot Engine Interrupt Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure0c0`] module"]
#[doc(alias = "SECURE0C0")]
pub type Secure0c0 = crate::Reg<secure0c0::Secure0c0Spec>;
#[doc = "Secure Boot Engine Interrupt Controller Register"]
pub mod secure0c0;
#[doc = "SECURE0C4 (rw) register accessor: Secure Boot Engine Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure0c4`] module"]
#[doc(alias = "SECURE0C4")]
pub type Secure0c4 = crate::Reg<secure0c4::Secure0c4Spec>;
#[doc = "Secure Boot Engine Interrupt Status Register"]
pub mod secure0c4;
#[doc = "SECURE800 (rw) register accessor: Secure Boot DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure800::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure800::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure800`] module"]
#[doc(alias = "SECURE800")]
pub type Secure800 = crate::Reg<secure800::Secure800Spec>;
#[doc = "Secure Boot DMA Enable Register"]
pub mod secure800;
#[doc = "SECURE808 (rw) register accessor: Secure Boot DMA Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure808::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure808::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure808`] module"]
#[doc(alias = "SECURE808")]
pub type Secure808 = crate::Reg<secure808::Secure808Spec>;
#[doc = "Secure Boot DMA Mode Register"]
pub mod secure808;
#[doc = "SECURE80C (rw) register accessor: Secure Boot Vault Key Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure80c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure80c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure80c`] module"]
#[doc(alias = "SECURE80C")]
pub type Secure80c = crate::Reg<secure80c::Secure80cSpec>;
#[doc = "Secure Boot Vault Key Control Register"]
pub mod secure80c;
#[doc = "SECURE810 (rw) register accessor: Secure Boot Digest Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secure810::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure810::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure810`] module"]
#[doc(alias = "SECURE810")]
pub type Secure810 = crate::Reg<secure810::Secure810Spec>;
#[doc = "Secure Boot Digest Status"]
pub mod secure810;
#[doc = "SECURE814 (rw) register accessor: Secure Boot Digest Check Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secure814::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure814::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure814`] module"]
#[doc(alias = "SECURE814")]
pub type Secure814 = crate::Reg<secure814::Secure814Spec>;
#[doc = "Secure Boot Digest Check Status"]
pub mod secure814;
#[doc = "SECURE820 (rw) register accessor: Secure Crypto Engine Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure820::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure820::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure820`] module"]
#[doc(alias = "SECURE820")]
pub type Secure820 = crate::Reg<secure820::Secure820Spec>;
#[doc = "Secure Crypto Engine Enable Register"]
pub mod secure820;
#[doc = "SECURE840 (rw) register accessor: Secure Boot DMA Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure840::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure840::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure840`] module"]
#[doc(alias = "SECURE840")]
pub type Secure840 = crate::Reg<secure840::Secure840Spec>;
#[doc = "Secure Boot DMA Source Address Register"]
pub mod secure840;
#[doc = "SECURE844 (rw) register accessor: Secure Boot DMA Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure844::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure844::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure844`] module"]
#[doc(alias = "SECURE844")]
pub type Secure844 = crate::Reg<secure844::Secure844Spec>;
#[doc = "Secure Boot DMA Destination Address Register"]
pub mod secure844;
#[doc = "SECURE848 (rw) register accessor: Secure Boot DMA Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure848::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure848::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure848`] module"]
#[doc(alias = "SECURE848")]
pub type Secure848 = crate::Reg<secure848::Secure848Spec>;
#[doc = "Secure Boot DMA Size Register"]
pub mod secure848;
#[doc = "SECURE854 (rw) register accessor: Secure Boot Hash Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure854::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure854::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure854`] module"]
#[doc(alias = "SECURE854")]
pub type Secure854 = crate::Reg<secure854::Secure854Spec>;
#[doc = "Secure Boot Hash Size Register"]
pub mod secure854;
#[doc = "SECURE858 (rw) register accessor: Secure Boot Hash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure858::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure858::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure858`] module"]
#[doc(alias = "SECURE858")]
pub type Secure858 = crate::Reg<secure858::Secure858Spec>;
#[doc = "Secure Boot Hash Mode Register"]
pub mod secure858;
#[doc = "SECURE85C (rw) register accessor: Secure Boot Hash Engine Fire Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure85c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure85c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure85c`] module"]
#[doc(alias = "SECURE85C")]
pub type Secure85c = crate::Reg<secure85c::Secure85cSpec>;
#[doc = "Secure Boot Hash Engine Fire Register"]
pub mod secure85c;
#[doc = "SECURE860 (rw) register accessor: Secure Boot Crypto Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure860::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure860::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure860`] module"]
#[doc(alias = "SECURE860")]
pub type Secure860 = crate::Reg<secure860::Secure860Spec>;
#[doc = "Secure Boot Crypto Mode Register"]
pub mod secure860;
#[doc = "SECURE864 (rw) register accessor: Secure Boot Crypto Data Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure864::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure864::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure864`] module"]
#[doc(alias = "SECURE864")]
pub type Secure864 = crate::Reg<secure864::Secure864Spec>;
#[doc = "Secure Boot Crypto Data Size Register"]
pub mod secure864;
#[doc = "SECURE868 (rw) register accessor: Secure Boot Crypto Data Total Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure868::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure868::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure868`] module"]
#[doc(alias = "SECURE868")]
pub type Secure868 = crate::Reg<secure868::Secure868Spec>;
#[doc = "Secure Boot Crypto Data Total Size Register"]
pub mod secure868;
#[doc = "SECURE870 (rw) register accessor: Secure Boot Crypto Low Key Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure870::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure870::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure870`] module"]
#[doc(alias = "SECURE870")]
pub type Secure870 = crate::Reg<secure870::Secure870Spec>;
#[doc = "Secure Boot Crypto Low Key Write Trigger Register"]
pub mod secure870;
#[doc = "SECURE874 (rw) register accessor: Secure Boot Crypto High Key Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure874::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure874::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure874`] module"]
#[doc(alias = "SECURE874")]
pub type Secure874 = crate::Reg<secure874::Secure874Spec>;
#[doc = "Secure Boot Crypto High Key Write Trigger Register"]
pub mod secure874;
#[doc = "SECURE878 (rw) register accessor: Secure Boot Crypto Vector Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure878::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure878::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure878`] module"]
#[doc(alias = "SECURE878")]
pub type Secure878 = crate::Reg<secure878::Secure878Spec>;
#[doc = "Secure Boot Crypto Vector Write Trigger Register"]
pub mod secure878;
#[doc = "SECURE87C (rw) register accessor: Secure Boot Crypto Engine Fire Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure87c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure87c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure87c`] module"]
#[doc(alias = "SECURE87C")]
pub type Secure87c = crate::Reg<secure87c::Secure87cSpec>;
#[doc = "Secure Boot Crypto Engine Fire Register"]
pub mod secure87c;
#[doc = "SECURE880 (rw) register accessor: Secure Boot Crypto Data Buffer 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure880::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure880::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure880`] module"]
#[doc(alias = "SECURE880")]
pub type Secure880 = crate::Reg<secure880::Secure880Spec>;
#[doc = "Secure Boot Crypto Data Buffer 0 Register"]
pub mod secure880;
#[doc = "SECURE884 (rw) register accessor: Secure Boot Crypto Data Buffer 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure884::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure884::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure884`] module"]
#[doc(alias = "SECURE884")]
pub type Secure884 = crate::Reg<secure884::Secure884Spec>;
#[doc = "Secure Boot Crypto Data Buffer 1 Register"]
pub mod secure884;
#[doc = "SECURE888 (rw) register accessor: Secure Boot Crypto Data Buffer 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure888`] module"]
#[doc(alias = "SECURE888")]
pub type Secure888 = crate::Reg<secure888::Secure888Spec>;
#[doc = "Secure Boot Crypto Data Buffer 2 Register"]
pub mod secure888;
#[doc = "SECURE88C (rw) register accessor: Secure Boot Crypto Data Buffer 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure88c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure88c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure88c`] module"]
#[doc(alias = "SECURE88C")]
pub type Secure88c = crate::Reg<secure88c::Secure88cSpec>;
#[doc = "Secure Boot Crypto Data Buffer 3 Register"]
pub mod secure88c;
#[doc = "SECURE890 (rw) register accessor: Secure Boot Crypto AES-GCM GHash Key Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure890::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure890::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure890`] module"]
#[doc(alias = "SECURE890")]
pub type Secure890 = crate::Reg<secure890::Secure890Spec>;
#[doc = "Secure Boot Crypto AES-GCM GHash Key Write Trigger Register"]
pub mod secure890;
#[doc = "SECURE8A0 (rw) register accessor: Secure Boot Crypto Key Buffer 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8a0`] module"]
#[doc(alias = "SECURE8A0")]
pub type Secure8a0 = crate::Reg<secure8a0::Secure8a0Spec>;
#[doc = "Secure Boot Crypto Key Buffer 0 Register"]
pub mod secure8a0;
#[doc = "SECURE8A4 (rw) register accessor: Secure Boot Crypto Key Buffer 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8a4`] module"]
#[doc(alias = "SECURE8A4")]
pub type Secure8a4 = crate::Reg<secure8a4::Secure8a4Spec>;
#[doc = "Secure Boot Crypto Key Buffer 1 Register"]
pub mod secure8a4;
#[doc = "SECURE8A8 (rw) register accessor: Secure Boot Crypto Key Buffer 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8a8`] module"]
#[doc(alias = "SECURE8A8")]
pub type Secure8a8 = crate::Reg<secure8a8::Secure8a8Spec>;
#[doc = "Secure Boot Crypto Key Buffer 2 Register"]
pub mod secure8a8;
#[doc = "SECURE8AC (rw) register accessor: Secure Boot Crypto Key Buffer 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8ac`] module"]
#[doc(alias = "SECURE8AC")]
pub type Secure8ac = crate::Reg<secure8ac::Secure8acSpec>;
#[doc = "Secure Boot Crypto Key Buffer 3 Register"]
pub mod secure8ac;
#[doc = "SECURE8B0 (rw) register accessor: Secure Boot Crypto Key Buffer 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8b0`] module"]
#[doc(alias = "SECURE8B0")]
pub type Secure8b0 = crate::Reg<secure8b0::Secure8b0Spec>;
#[doc = "Secure Boot Crypto Key Buffer 4 Register"]
pub mod secure8b0;
#[doc = "SECURE8B4 (rw) register accessor: Secure Boot Crypto Key Buffer 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8b4`] module"]
#[doc(alias = "SECURE8B4")]
pub type Secure8b4 = crate::Reg<secure8b4::Secure8b4Spec>;
#[doc = "Secure Boot Crypto Key Buffer 5 Register"]
pub mod secure8b4;
#[doc = "SECURE8B8 (rw) register accessor: Secure Boot Crypto Key Buffer 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8b8`] module"]
#[doc(alias = "SECURE8B8")]
pub type Secure8b8 = crate::Reg<secure8b8::Secure8b8Spec>;
#[doc = "Secure Boot Crypto Key Buffer 6 Register"]
pub mod secure8b8;
#[doc = "SECURE8BC (rw) register accessor: Secure Boot Crypto Key Buffer 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure8bc`] module"]
#[doc(alias = "SECURE8BC")]
pub type Secure8bc = crate::Reg<secure8bc::Secure8bcSpec>;
#[doc = "Secure Boot Crypto Key Buffer 7 Register"]
pub mod secure8bc;
#[doc = "SECURE900 (rw) register accessor: Secure Boot First Vault Key 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure900::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure900::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure900`] module"]
#[doc(alias = "SECURE900")]
pub type Secure900 = crate::Reg<secure900::Secure900Spec>;
#[doc = "Secure Boot First Vault Key 0 Register"]
pub mod secure900;
#[doc = "SECURE904 (rw) register accessor: Secure Boot First Vault Key 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure904::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure904::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure904`] module"]
#[doc(alias = "SECURE904")]
pub type Secure904 = crate::Reg<secure904::Secure904Spec>;
#[doc = "Secure Boot First Vault Key 1 Register"]
pub mod secure904;
#[doc = "SECURE908 (rw) register accessor: Secure Boot First Vault Key 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure908::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure908::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure908`] module"]
#[doc(alias = "SECURE908")]
pub type Secure908 = crate::Reg<secure908::Secure908Spec>;
#[doc = "Secure Boot First Vault Key 2 Register"]
pub mod secure908;
#[doc = "SECURE90C (rw) register accessor: Secure Boot First Vault Key 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure90c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure90c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure90c`] module"]
#[doc(alias = "SECURE90C")]
pub type Secure90c = crate::Reg<secure90c::Secure90cSpec>;
#[doc = "Secure Boot First Vault Key 3 Register"]
pub mod secure90c;
#[doc = "SECURE910 (rw) register accessor: Secure Boot First Vault Key 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure910::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure910::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure910`] module"]
#[doc(alias = "SECURE910")]
pub type Secure910 = crate::Reg<secure910::Secure910Spec>;
#[doc = "Secure Boot First Vault Key 4 Register"]
pub mod secure910;
#[doc = "SECURE914 (rw) register accessor: Secure Boot First Vault Key 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure914::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure914::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure914`] module"]
#[doc(alias = "SECURE914")]
pub type Secure914 = crate::Reg<secure914::Secure914Spec>;
#[doc = "Secure Boot First Vault Key 5 Register"]
pub mod secure914;
#[doc = "SECURE918 (rw) register accessor: Secure Boot First Vault Key 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure918::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure918::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure918`] module"]
#[doc(alias = "SECURE918")]
pub type Secure918 = crate::Reg<secure918::Secure918Spec>;
#[doc = "Secure Boot First Vault Key 6 Register"]
pub mod secure918;
#[doc = "SECURE91C (rw) register accessor: Secure Boot First Vault Key 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure91c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure91c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure91c`] module"]
#[doc(alias = "SECURE91C")]
pub type Secure91c = crate::Reg<secure91c::Secure91cSpec>;
#[doc = "Secure Boot First Vault Key 7 Register"]
pub mod secure91c;
#[doc = "SECURE920 (rw) register accessor: Secure Boot Second Vault Key 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure920::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure920::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure920`] module"]
#[doc(alias = "SECURE920")]
pub type Secure920 = crate::Reg<secure920::Secure920Spec>;
#[doc = "Secure Boot Second Vault Key 0 Register"]
pub mod secure920;
#[doc = "SECURE924 (rw) register accessor: Secure Boot Second Vault Key 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure924::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure924::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure924`] module"]
#[doc(alias = "SECURE924")]
pub type Secure924 = crate::Reg<secure924::Secure924Spec>;
#[doc = "Secure Boot Second Vault Key 1 Register"]
pub mod secure924;
#[doc = "SECURE928 (rw) register accessor: Secure Boot Second Vault Key 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure928::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure928::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure928`] module"]
#[doc(alias = "SECURE928")]
pub type Secure928 = crate::Reg<secure928::Secure928Spec>;
#[doc = "Secure Boot Second Vault Key 2 Register"]
pub mod secure928;
#[doc = "SECURE92C (rw) register accessor: Secure Boot Second Vault Key 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure92c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure92c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure92c`] module"]
#[doc(alias = "SECURE92C")]
pub type Secure92c = crate::Reg<secure92c::Secure92cSpec>;
#[doc = "Secure Boot Second Vault Key 3 Register"]
pub mod secure92c;
#[doc = "SECURE930 (rw) register accessor: Secure Boot Second Vault Key 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure930::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure930::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure930`] module"]
#[doc(alias = "SECURE930")]
pub type Secure930 = crate::Reg<secure930::Secure930Spec>;
#[doc = "Secure Boot Second Vault Key 4 Register"]
pub mod secure930;
#[doc = "SECURE934 (rw) register accessor: Secure Boot Second Vault Key 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure934::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure934::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure934`] module"]
#[doc(alias = "SECURE934")]
pub type Secure934 = crate::Reg<secure934::Secure934Spec>;
#[doc = "Secure Boot Second Vault Key 5 Register"]
pub mod secure934;
#[doc = "SECURE938 (rw) register accessor: Secure Boot Second Vault Key 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure938::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure938::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure938`] module"]
#[doc(alias = "SECURE938")]
pub type Secure938 = crate::Reg<secure938::Secure938Spec>;
#[doc = "Secure Boot Second Vault Key 6 Register"]
pub mod secure938;
#[doc = "SECURE93C (rw) register accessor: Secure Boot Second Vault Key 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure93c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure93c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure93c`] module"]
#[doc(alias = "SECURE93C")]
pub type Secure93c = crate::Reg<secure93c::Secure93cSpec>;
#[doc = "Secure Boot Second Vault Key 7 Register"]
pub mod secure93c;
#[doc = "SECURE980 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure980::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure980::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure980`] module"]
#[doc(alias = "SECURE980")]
pub type Secure980 = crate::Reg<secure980::Secure980Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure980;
#[doc = "SECURE984 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure984::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure984::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure984`] module"]
#[doc(alias = "SECURE984")]
pub type Secure984 = crate::Reg<secure984::Secure984Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure984;
#[doc = "SECURE988 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure988::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure988::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure988`] module"]
#[doc(alias = "SECURE988")]
pub type Secure988 = crate::Reg<secure988::Secure988Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure988;
#[doc = "SECURE98C (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure98c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure98c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure98c`] module"]
#[doc(alias = "SECURE98C")]
pub type Secure98c = crate::Reg<secure98c::Secure98cSpec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure98c;
#[doc = "SECURE990 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure990::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure990::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure990`] module"]
#[doc(alias = "SECURE990")]
pub type Secure990 = crate::Reg<secure990::Secure990Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure990;
#[doc = "SECURE994 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure994::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure994::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure994`] module"]
#[doc(alias = "SECURE994")]
pub type Secure994 = crate::Reg<secure994::Secure994Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure994;
#[doc = "SECURE998 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure998::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure998::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure998`] module"]
#[doc(alias = "SECURE998")]
pub type Secure998 = crate::Reg<secure998::Secure998Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure998;
#[doc = "SECURE99C (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure99c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure99c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure99c`] module"]
#[doc(alias = "SECURE99C")]
pub type Secure99c = crate::Reg<secure99c::Secure99cSpec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure99c;
#[doc = "SECURE9A0 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9a0`] module"]
#[doc(alias = "SECURE9A0")]
pub type Secure9a0 = crate::Reg<secure9a0::Secure9a0Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9a0;
#[doc = "SECURE9A4 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9a4`] module"]
#[doc(alias = "SECURE9A4")]
pub type Secure9a4 = crate::Reg<secure9a4::Secure9a4Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9a4;
#[doc = "SECURE9A8 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9a8`] module"]
#[doc(alias = "SECURE9A8")]
pub type Secure9a8 = crate::Reg<secure9a8::Secure9a8Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9a8;
#[doc = "SECURE9AC (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9ac`] module"]
#[doc(alias = "SECURE9AC")]
pub type Secure9ac = crate::Reg<secure9ac::Secure9acSpec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9ac;
#[doc = "SECURE9B0 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9b0`] module"]
#[doc(alias = "SECURE9B0")]
pub type Secure9b0 = crate::Reg<secure9b0::Secure9b0Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9b0;
#[doc = "SECURE9B4 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9b4`] module"]
#[doc(alias = "SECURE9B4")]
pub type Secure9b4 = crate::Reg<secure9b4::Secure9b4Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9b4;
#[doc = "SECURE9B8 (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9b8`] module"]
#[doc(alias = "SECURE9B8")]
pub type Secure9b8 = crate::Reg<secure9b8::Secure9b8Spec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9b8;
#[doc = "SECURE9BC (rw) register accessor: Secure Boot Image Digest Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secure9bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure9bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure9bc`] module"]
#[doc(alias = "SECURE9BC")]
pub type Secure9bc = crate::Reg<secure9bc::Secure9bcSpec>;
#[doc = "Secure Boot Image Digest Read Back \\#0 - \\#16"]
pub mod secure9bc;
#[doc = "SECUREA00 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea00`] module"]
#[doc(alias = "SECUREA00")]
pub type Securea00 = crate::Reg<securea00::Securea00Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea00;
#[doc = "SECUREA04 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea04`] module"]
#[doc(alias = "SECUREA04")]
pub type Securea04 = crate::Reg<securea04::Securea04Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea04;
#[doc = "SECUREA08 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea08`] module"]
#[doc(alias = "SECUREA08")]
pub type Securea08 = crate::Reg<securea08::Securea08Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea08;
#[doc = "SECUREA0C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea0c`] module"]
#[doc(alias = "SECUREA0C")]
pub type Securea0c = crate::Reg<securea0c::Securea0cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea0c;
#[doc = "SECUREA10 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea10`] module"]
#[doc(alias = "SECUREA10")]
pub type Securea10 = crate::Reg<securea10::Securea10Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea10;
#[doc = "SECUREA14 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea14`] module"]
#[doc(alias = "SECUREA14")]
pub type Securea14 = crate::Reg<securea14::Securea14Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea14;
#[doc = "SECUREA18 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea18`] module"]
#[doc(alias = "SECUREA18")]
pub type Securea18 = crate::Reg<securea18::Securea18Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea18;
#[doc = "SECUREA1C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea1c`] module"]
#[doc(alias = "SECUREA1C")]
pub type Securea1c = crate::Reg<securea1c::Securea1cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea1c;
#[doc = "SECUREA20 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea20`] module"]
#[doc(alias = "SECUREA20")]
pub type Securea20 = crate::Reg<securea20::Securea20Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea20;
#[doc = "SECUREA24 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea24`] module"]
#[doc(alias = "SECUREA24")]
pub type Securea24 = crate::Reg<securea24::Securea24Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea24;
#[doc = "SECUREA28 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea28`] module"]
#[doc(alias = "SECUREA28")]
pub type Securea28 = crate::Reg<securea28::Securea28Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea28;
#[doc = "SECUREA2C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea2c`] module"]
#[doc(alias = "SECUREA2C")]
pub type Securea2c = crate::Reg<securea2c::Securea2cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea2c;
#[doc = "SECUREA30 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea30`] module"]
#[doc(alias = "SECUREA30")]
pub type Securea30 = crate::Reg<securea30::Securea30Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea30;
#[doc = "SECUREA34 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea34`] module"]
#[doc(alias = "SECUREA34")]
pub type Securea34 = crate::Reg<securea34::Securea34Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea34;
#[doc = "SECUREA38 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea38`] module"]
#[doc(alias = "SECUREA38")]
pub type Securea38 = crate::Reg<securea38::Securea38Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea38;
#[doc = "SECUREA3C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea3c`] module"]
#[doc(alias = "SECUREA3C")]
pub type Securea3c = crate::Reg<securea3c::Securea3cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea3c;
#[doc = "SECUREA40 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea40`] module"]
#[doc(alias = "SECUREA40")]
pub type Securea40 = crate::Reg<securea40::Securea40Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea40;
#[doc = "SECUREA44 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea44`] module"]
#[doc(alias = "SECUREA44")]
pub type Securea44 = crate::Reg<securea44::Securea44Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea44;
#[doc = "SECUREA48 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea48`] module"]
#[doc(alias = "SECUREA48")]
pub type Securea48 = crate::Reg<securea48::Securea48Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea48;
#[doc = "SECUREA4C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea4c`] module"]
#[doc(alias = "SECUREA4C")]
pub type Securea4c = crate::Reg<securea4c::Securea4cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea4c;
#[doc = "SECUREA50 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea50`] module"]
#[doc(alias = "SECUREA50")]
pub type Securea50 = crate::Reg<securea50::Securea50Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea50;
#[doc = "SECUREA54 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea54`] module"]
#[doc(alias = "SECUREA54")]
pub type Securea54 = crate::Reg<securea54::Securea54Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea54;
#[doc = "SECUREA58 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea58`] module"]
#[doc(alias = "SECUREA58")]
pub type Securea58 = crate::Reg<securea58::Securea58Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea58;
#[doc = "SECUREA5C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea5c`] module"]
#[doc(alias = "SECUREA5C")]
pub type Securea5c = crate::Reg<securea5c::Securea5cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea5c;
#[doc = "SECUREA60 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea60`] module"]
#[doc(alias = "SECUREA60")]
pub type Securea60 = crate::Reg<securea60::Securea60Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea60;
#[doc = "SECUREA64 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea64`] module"]
#[doc(alias = "SECUREA64")]
pub type Securea64 = crate::Reg<securea64::Securea64Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea64;
#[doc = "SECUREA68 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea68`] module"]
#[doc(alias = "SECUREA68")]
pub type Securea68 = crate::Reg<securea68::Securea68Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea68;
#[doc = "SECUREA6C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea6c`] module"]
#[doc(alias = "SECUREA6C")]
pub type Securea6c = crate::Reg<securea6c::Securea6cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea6c;
#[doc = "SECUREA70 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea70`] module"]
#[doc(alias = "SECUREA70")]
pub type Securea70 = crate::Reg<securea70::Securea70Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea70;
#[doc = "SECUREA74 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea74`] module"]
#[doc(alias = "SECUREA74")]
pub type Securea74 = crate::Reg<securea74::Securea74Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea74;
#[doc = "SECUREA78 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea78`] module"]
#[doc(alias = "SECUREA78")]
pub type Securea78 = crate::Reg<securea78::Securea78Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea78;
#[doc = "SECUREA7C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea7c`] module"]
#[doc(alias = "SECUREA7C")]
pub type Securea7c = crate::Reg<securea7c::Securea7cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea7c;
#[doc = "SECUREA80 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea80`] module"]
#[doc(alias = "SECUREA80")]
pub type Securea80 = crate::Reg<securea80::Securea80Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea80;
#[doc = "SECUREA84 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea84`] module"]
#[doc(alias = "SECUREA84")]
pub type Securea84 = crate::Reg<securea84::Securea84Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea84;
#[doc = "SECUREA88 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea88`] module"]
#[doc(alias = "SECUREA88")]
pub type Securea88 = crate::Reg<securea88::Securea88Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea88;
#[doc = "SECUREA8C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea8c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea8c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea8c`] module"]
#[doc(alias = "SECUREA8C")]
pub type Securea8c = crate::Reg<securea8c::Securea8cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea8c;
#[doc = "SECUREA90 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea90`] module"]
#[doc(alias = "SECUREA90")]
pub type Securea90 = crate::Reg<securea90::Securea90Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea90;
#[doc = "SECUREA94 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea94`] module"]
#[doc(alias = "SECUREA94")]
pub type Securea94 = crate::Reg<securea94::Securea94Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea94;
#[doc = "SECUREA98 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea98`] module"]
#[doc(alias = "SECUREA98")]
pub type Securea98 = crate::Reg<securea98::Securea98Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea98;
#[doc = "SECUREA9C (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea9c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea9c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securea9c`] module"]
#[doc(alias = "SECUREA9C")]
pub type Securea9c = crate::Reg<securea9c::Securea9cSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securea9c;
#[doc = "SECUREAA0 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureaa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureaa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureaa0`] module"]
#[doc(alias = "SECUREAA0")]
pub type Secureaa0 = crate::Reg<secureaa0::Secureaa0Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureaa0;
#[doc = "SECUREAA4 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureaa4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureaa4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureaa4`] module"]
#[doc(alias = "SECUREAA4")]
pub type Secureaa4 = crate::Reg<secureaa4::Secureaa4Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureaa4;
#[doc = "SECUREAA8 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureaa8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureaa8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureaa8`] module"]
#[doc(alias = "SECUREAA8")]
pub type Secureaa8 = crate::Reg<secureaa8::Secureaa8Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureaa8;
#[doc = "SECUREAAC (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureaac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureaac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureaac`] module"]
#[doc(alias = "SECUREAAC")]
pub type Secureaac = crate::Reg<secureaac::SecureaacSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureaac;
#[doc = "SECUREAB0 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureab0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureab0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureab0`] module"]
#[doc(alias = "SECUREAB0")]
pub type Secureab0 = crate::Reg<secureab0::Secureab0Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureab0;
#[doc = "SECUREAB4 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureab4`] module"]
#[doc(alias = "SECUREAB4")]
pub type Secureab4 = crate::Reg<secureab4::Secureab4Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureab4;
#[doc = "SECUREAB8 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureab8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureab8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureab8`] module"]
#[doc(alias = "SECUREAB8")]
pub type Secureab8 = crate::Reg<secureab8::Secureab8Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureab8;
#[doc = "SECUREABC (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureabc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureabc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureabc`] module"]
#[doc(alias = "SECUREABC")]
pub type Secureabc = crate::Reg<secureabc::SecureabcSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureabc;
#[doc = "SECUREAC0 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureac0`] module"]
#[doc(alias = "SECUREAC0")]
pub type Secureac0 = crate::Reg<secureac0::Secureac0Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureac0;
#[doc = "SECUREAC4 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureac4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureac4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureac4`] module"]
#[doc(alias = "SECUREAC4")]
pub type Secureac4 = crate::Reg<secureac4::Secureac4Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureac4;
#[doc = "SECUREAC8 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureac8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureac8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureac8`] module"]
#[doc(alias = "SECUREAC8")]
pub type Secureac8 = crate::Reg<secureac8::Secureac8Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureac8;
#[doc = "SECUREACC (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureacc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureacc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureacc`] module"]
#[doc(alias = "SECUREACC")]
pub type Secureacc = crate::Reg<secureacc::SecureaccSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureacc;
#[doc = "SECUREAD0 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securead0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securead0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securead0`] module"]
#[doc(alias = "SECUREAD0")]
pub type Securead0 = crate::Reg<securead0::Securead0Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securead0;
#[doc = "SECUREAD4 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securead4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securead4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securead4`] module"]
#[doc(alias = "SECUREAD4")]
pub type Securead4 = crate::Reg<securead4::Securead4Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securead4;
#[doc = "SECUREAD8 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securead8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securead8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@securead8`] module"]
#[doc(alias = "SECUREAD8")]
pub type Securead8 = crate::Reg<securead8::Securead8Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod securead8;
#[doc = "SECUREADC (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureadc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureadc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureadc`] module"]
#[doc(alias = "SECUREADC")]
pub type Secureadc = crate::Reg<secureadc::SecureadcSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureadc;
#[doc = "SECUREAE0 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureae0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureae0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureae0`] module"]
#[doc(alias = "SECUREAE0")]
pub type Secureae0 = crate::Reg<secureae0::Secureae0Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureae0;
#[doc = "SECUREAE4 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureae4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureae4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureae4`] module"]
#[doc(alias = "SECUREAE4")]
pub type Secureae4 = crate::Reg<secureae4::Secureae4Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureae4;
#[doc = "SECUREAE8 (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureae8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureae8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureae8`] module"]
#[doc(alias = "SECUREAE8")]
pub type Secureae8 = crate::Reg<secureae8::Secureae8Spec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureae8;
#[doc = "SECUREAEC (rw) register accessor: Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureaec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureaec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureaec`] module"]
#[doc(alias = "SECUREAEC")]
pub type Secureaec = crate::Reg<secureaec::SecureaecSpec>;
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16"]
pub mod secureaec;

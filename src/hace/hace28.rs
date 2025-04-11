#[doc = "Register `HACE28` reader"]
pub type R = crate::R<Hace28Spec>;
#[doc = "Register `HACE28` writer"]
pub type W = crate::W<Hace28Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `BaseAddrOfHMACKeyBuf302` reader - Base address of HMAC key buffer\\[30:2\\] (8-byte aligned)"]
pub type BaseAddrOfHmackeyBuf302R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfHMACKeyBuf302` writer - Base address of HMAC key buffer\\[30:2\\] (8-byte aligned)"]
pub type BaseAddrOfHmackeyBuf302W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:30 - Base address of HMAC key buffer\\[30:2\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_hmackey_buf302(&self) -> BaseAddrOfHmackeyBuf302R {
        BaseAddrOfHmackeyBuf302R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:30 - Base address of HMAC key buffer\\[30:2\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_hmackey_buf302(&mut self) -> BaseAddrOfHmackeyBuf302W<Hace28Spec> {
        BaseAddrOfHmackeyBuf302W::new(self, 3)
    }
}
#[doc = "Hash HMAC Key Buffer Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace28Spec;
impl crate::RegisterSpec for Hace28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace28::R`](R) reader structure"]
impl crate::Readable for Hace28Spec {}
#[doc = "`write(|w| ..)` method takes [`hace28::W`](W) writer structure"]
impl crate::Writable for Hace28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE28 to value 0"]
impl crate::Resettable for Hace28Spec {}

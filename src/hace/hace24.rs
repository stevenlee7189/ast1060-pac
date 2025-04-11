#[doc = "Register `HACE24` reader"]
pub type R = crate::R<Hace24Spec>;
#[doc = "Register `HACE24` writer"]
pub type W = crate::W<Hace24Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `BaseAddrOfHashDigestWrBuf303` reader - Base address of hash digest write buffer\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfHashDigestWrBuf303R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfHashDigestWrBuf303` writer - Base address of hash digest write buffer\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfHashDigestWrBuf303W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:30 - Base address of hash digest write buffer\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_hash_digest_wr_buf303(&self) -> BaseAddrOfHashDigestWrBuf303R {
        BaseAddrOfHashDigestWrBuf303R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:30 - Base address of hash digest write buffer\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_hash_digest_wr_buf303(
        &mut self,
    ) -> BaseAddrOfHashDigestWrBuf303W<Hace24Spec> {
        BaseAddrOfHashDigestWrBuf303W::new(self, 3)
    }
}
#[doc = "Hash Digest Write Buffer Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace24Spec;
impl crate::RegisterSpec for Hace24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace24::R`](R) reader structure"]
impl crate::Readable for Hace24Spec {}
#[doc = "`write(|w| ..)` method takes [`hace24::W`](W) writer structure"]
impl crate::Writable for Hace24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE24 to value 0"]
impl crate::Resettable for Hace24Spec {}

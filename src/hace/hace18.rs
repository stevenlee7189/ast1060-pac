#[doc = "Register `HACE18` reader"]
pub type R = crate::R<Hace18Spec>;
#[doc = "Register `HACE18` writer"]
pub type W = crate::W<Hace18Spec>;
#[doc = "Field `BaseAddrOfCryptoAESGCMTagWrBuf300` reader - Base address of crypto AES-GCM Tag write buffer\\[30:0\\] (byte aligned)"]
pub type BaseAddrOfCryptoAesgcmtagWrBuf300R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfCryptoAESGCMTagWrBuf300` writer - Base address of crypto AES-GCM Tag write buffer\\[30:0\\] (byte aligned)"]
pub type BaseAddrOfCryptoAesgcmtagWrBuf300W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Base address of crypto AES-GCM Tag write buffer\\[30:0\\] (byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_crypto_aesgcmtag_wr_buf300(&self) -> BaseAddrOfCryptoAesgcmtagWrBuf300R {
        BaseAddrOfCryptoAesgcmtagWrBuf300R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Base address of crypto AES-GCM Tag write buffer\\[30:0\\] (byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_crypto_aesgcmtag_wr_buf300(
        &mut self,
    ) -> BaseAddrOfCryptoAesgcmtagWrBuf300W<Hace18Spec> {
        BaseAddrOfCryptoAesgcmtagWrBuf300W::new(self, 0)
    }
}
#[doc = "Crypto AES-GCM Tag Write Buffer Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace18Spec;
impl crate::RegisterSpec for Hace18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace18::R`](R) reader structure"]
impl crate::Readable for Hace18Spec {}
#[doc = "`write(|w| ..)` method takes [`hace18::W`](W) writer structure"]
impl crate::Writable for Hace18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE18 to value 0"]
impl crate::Resettable for Hace18Spec {}

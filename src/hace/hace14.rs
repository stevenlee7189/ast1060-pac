#[doc = "Register `HACE14` reader"]
pub type R = crate::R<Hace14Spec>;
#[doc = "Register `HACE14` writer"]
pub type W = crate::W<Hace14Spec>;
#[doc = "Field `CryptoAESGCMAddDataLenBytes` reader - Crypto AES-GCM additional data length (bytes)"]
pub type CryptoAesgcmaddDataLenBytesR = crate::FieldReader<u32>;
#[doc = "Field `CryptoAESGCMAddDataLenBytes` writer - Crypto AES-GCM additional data length (bytes)"]
pub type CryptoAesgcmaddDataLenBytesW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - Crypto AES-GCM additional data length (bytes)"]
    #[inline(always)]
    pub fn crypto_aesgcmadd_data_len_bytes(&self) -> CryptoAesgcmaddDataLenBytesR {
        CryptoAesgcmaddDataLenBytesR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - Crypto AES-GCM additional data length (bytes)"]
    #[inline(always)]
    pub fn crypto_aesgcmadd_data_len_bytes(&mut self) -> CryptoAesgcmaddDataLenBytesW<Hace14Spec> {
        CryptoAesgcmaddDataLenBytesW::new(self, 0)
    }
}
#[doc = "Crypto AES-GCM Additional Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace14Spec;
impl crate::RegisterSpec for Hace14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace14::R`](R) reader structure"]
impl crate::Readable for Hace14Spec {}
#[doc = "`write(|w| ..)` method takes [`hace14::W`](W) writer structure"]
impl crate::Writable for Hace14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE14 to value 0"]
impl crate::Resettable for Hace14Spec {}

#[doc = "Register `RCCUTOFF` reader"]
pub type R = crate::R<RccutoffSpec>;
#[doc = "Field `RCCUTOFF` reader - Repetition counter cutoff"]
pub type RccutoffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Repetition counter cutoff"]
    #[inline(always)]
    pub fn rccutoff(&self) -> RccutoffR {
        RccutoffR::new(self.bits)
    }
}
#[doc = "Repetition counter cutoff\n\nYou can [`read`](crate::Reg::read) this register and get [`rccutoff::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccutoffSpec;
impl crate::RegisterSpec for RccutoffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rccutoff::R`](R) reader structure"]
impl crate::Readable for RccutoffSpec {}
#[doc = "`reset()` method sets RCCUTOFF to value 0xffff_ffff"]
impl crate::Resettable for RccutoffSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

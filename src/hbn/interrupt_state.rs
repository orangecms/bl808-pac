#[doc = "Register `interrupt_state` reader"]
pub struct R(crate::R<INTERRUPT_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `interrupt_state` writer"]
pub struct W(crate::W<INTERRUPT_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTERRUPT_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_STATE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate interrupt state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_state](index.html) module"]
pub struct INTERRUPT_STATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_state::R](R) reader structure"]
impl crate::Readable for INTERRUPT_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_state::W](W) writer structure"]
impl crate::Writable for INTERRUPT_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_state to value 0"]
impl crate::Resettable for INTERRUPT_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

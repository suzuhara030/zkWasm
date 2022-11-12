use super::super::{Sha256HelperOp, Sha256HelperTableConfig};
use crate::foreign::sha256_helper::circuits::assign::Sha256HelperTableChip;
use crate::foreign::sha256_helper::circuits::ops::sigma::SigmaParam;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Region,
    plonk::{ConstraintSystem, Error},
};

// (x right_rotate 17) ^ (x right_rotate 19) ^ (x >> 10)
const PARAM: SigmaParam = SigmaParam {
    name: "ssigma1",
    op: Sha256HelperOp::SSigma1,
    rotates: &[17, 19],
    shifts: &[10],
};
impl<F: FieldExt> Sha256HelperTableConfig<F> {
    pub(crate) fn configure_ssigma1(&self, meta: &mut ConstraintSystem<F>) {
        self.configure_sigma(meta, PARAM);
    }
}

impl<F: FieldExt> Sha256HelperTableChip<F> {
    pub(crate) fn assign_ssigma1(
        &self,
        region: &mut Region<F>,
        offset: usize,
        args: &Vec<u32>,
    ) -> Result<(), Error> {
        self.assign_sigma(region, offset, args, PARAM)
    }
}

use super::super::{Sha256HelperOp, Sha256HelperTableConfig};
use crate::foreign::sha256_helper::circuits::assign::Sha256HelperTableChip;
use crate::foreign::sha256_helper::circuits::ops::sigma::SigmaParam;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Region,
    plonk::{ConstraintSystem, Error},
};

// (x right_rotate 7) ^ (x right_rotate 18) ^ (x >> 3)
const PARAM: SigmaParam = SigmaParam {
    name: "ssigma0",
    op: Sha256HelperOp::SSigma0,
    rotates: &[7, 18],
    shifts: &[3],
};
impl<F: FieldExt> Sha256HelperTableConfig<F> {
    pub(crate) fn configure_ssigma0(&self, meta: &mut ConstraintSystem<F>) {
        self.configure_sigma(meta, PARAM);
    }
}

impl<F: FieldExt> Sha256HelperTableChip<F> {
    pub(crate) fn assign_ssigma0(
        &self,
        region: &mut Region<F>,
        offset: usize,
        args: &Vec<u32>,
    ) -> Result<(), Error> {
        self.assign_sigma(region, offset, args, PARAM)
    }
}

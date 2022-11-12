use super::super::{Sha256HelperOp, Sha256HelperTableConfig};
use crate::foreign::sha256_helper::circuits::assign::Sha256HelperTableChip;
use crate::foreign::sha256_helper::circuits::ops::sigma::SigmaParam;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Region,
    plonk::{ConstraintSystem, Error},
};

// (x right_rotate 6) ^ (x right_rotate 11) ^ (x right_rotate 25)
const PARAM: SigmaParam = SigmaParam {
    name: "lsigma1",
    op: Sha256HelperOp::LSigma1,
    rotates: &[6, 11, 25],
    shifts: &[],
};

impl<F: FieldExt> Sha256HelperTableConfig<F> {
    pub(crate) fn configure_lsigma1(&self, meta: &mut ConstraintSystem<F>) {
        self.configure_sigma(meta, PARAM);
    }
}

impl<F: FieldExt> Sha256HelperTableChip<F> {
    pub(crate) fn assign_lsigma1(
        &self,
        region: &mut Region<F>,
        offset: usize,
        args: &Vec<u32>,
    ) -> Result<(), Error> {
        self.assign_sigma(region, offset, args, PARAM)
    }
}

use super::super::{Sha256HelperOp, Sha256HelperTableConfig};
use crate::foreign::sha256_helper::circuits::assign::Sha256HelperTableChip;
use crate::foreign::sha256_helper::circuits::ops::sigma::SigmaParam;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Region,
    plonk::{ConstraintSystem, Error},
};

// (x right_rotate 2) ^ (x right_rotate 13) ^ (x right_rotate 22)
const PARAM: SigmaParam = SigmaParam {
    name: "lsigma0",
    op: Sha256HelperOp::LSigma0,
    rotates: &[2, 13, 22],
    shifts: &[],
};

impl<F: FieldExt> Sha256HelperTableConfig<F> {
    pub(crate) fn configure_lsigma0(&self, meta: &mut ConstraintSystem<F>) {
        self.configure_sigma(meta, PARAM);
    }
}

impl<F: FieldExt> Sha256HelperTableChip<F> {
    pub(crate) fn assign_lsigma0(
        &self,
        region: &mut Region<F>,
        offset: usize,
        args: &Vec<u32>,
    ) -> Result<(), Error> {
        self.assign_sigma(region, offset, args, PARAM)
    }
}

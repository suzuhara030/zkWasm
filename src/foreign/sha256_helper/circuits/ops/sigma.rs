use super::super::{Sha256HelperOp, Sha256HelperTableConfig};
use crate::foreign::sha256_helper::circuits::assign::Sha256HelperTableChip;
use crate::{constant_from, curr, foreign::sha256_helper::circuits::Sha2HelperEncode};
use crate::{nextn, rotation_constraints, shift_constraints};
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Region,
    plonk::{ConstraintSystem, Error},
};

pub struct SigmaParam<'a> {
    pub name: &'a str,
    pub op: Sha256HelperOp,
    pub rotates: &'a [i32],
    pub shifts: &'a [i32],
}
impl<F: FieldExt> Sha256HelperTableConfig<F> {
    pub(crate) fn configure_sigma(&self, meta: &mut ConstraintSystem<F>, param: SigmaParam) {
        meta.create_gate("sha256 ssigma0 opcode", |meta| {
            let enable = self.is_op_enabled_expr(meta, param.op);

            let x = self.arg_to_rotate_u32_expr(meta, 0, 0);
            let res = self.arg_to_rotate_u32_expr(meta, 4, 0);

            vec![
                enable.clone() * (curr!(meta, self.op.0) - constant_from!(param.op)),
                enable.clone()
                    * (self.opcode_expr(meta)
                        - Sha2HelperEncode::encode_opcode_expr(
                            curr!(meta, self.op.0),
                            vec![x],
                            res,
                        )),
            ]
        });

        let rlength = param.rotates.len();
        let slength = param.shifts.len();
        for i in 0..rlength {
            rotation_constraints!(
                meta,
                self,
                "ssigma0 rotate 7",
                i + 1,
                param.rotates[i],
                param.op
            );
        }
        for i in 0..slength {
            shift_constraints!(
                meta,
                self,
                "ssigma0 shift 3",
                i + rlength + 1,
                param.shifts[i],
                param.op,
            );
        }
    }
}

impl<F: FieldExt> Sha256HelperTableChip<F> {
    pub(crate) fn assign_sigma(
        &self,
        region: &mut Region<F>,
        offset: usize,
        args: &Vec<u32>,
        param: SigmaParam,
    ) -> Result<(), Error> {
        let rlength = param.rotates.len();
        let slength = param.shifts.len();
        for i in 0..rlength {
            self.assign_rotate_aux(
                region,
                offset,
                args,
                i + 1,
                param.rotates[i].try_into().unwrap(),
                3 * i + 1,
                false,
            )?;
        }
        for i in 0..slength {
            self.assign_rotate_aux(
                region,
                offset,
                args,
                i + rlength + 1,
                param.shifts[i].try_into().unwrap(),
                3 * (i + rlength) + 1,
                true,
            )?;
        }
        Ok(())
    }
}
